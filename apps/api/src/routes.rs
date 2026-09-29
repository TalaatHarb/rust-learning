use axum::{
    Json, Router,
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::{
    collections::HashMap,
    io::ErrorKind,
    path::{Path as FsPath, PathBuf},
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    AppState,
    auth::AuthenticatedUser,
    error::{AppError, AppResult},
    executor_client::{ExecuteRequest, execute_submission},
};

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct MeResponse {
    subject: String,
    roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionRequest {
    exercise_id: String,
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubmissionResponse {
    attempt_id: Uuid,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttemptResponse {
    attempt_id: Uuid,
    exercise_id: String,
    status: String,
    stdout: String,
    stderr: String,
    duration_ms: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgressOverviewResponse {
    resume_unit_id: String,
    resume_unit_slug: String,
    resume_unit_title: String,
    units: Vec<ProgressUnitResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgressUnitResponse {
    unit_id: String,
    unit_slug: String,
    unit_title: String,
    status: String,
    completed_attempts: i64,
    latest_attempt_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnitResponse {
    id: String,
    title: String,
    learning_objectives: Vec<String>,
    explanation: String,
    examples: Vec<UnitExample>,
    compiler_error_examples: Vec<String>,
    exercise_id: String,
    completion_criteria: Vec<String>,
    hints: Vec<String>,
    starter_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoadmapResponse {
    id: String,
    title: String,
    modules: Vec<RoadmapModuleResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoadmapModuleResponse {
    id: String,
    title: String,
    units: Vec<RoadmapUnitResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoadmapUnitResponse {
    id: String,
    slug: String,
    title: String,
    prerequisite_unit_ids: Vec<String>,
    exercise_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnitExample {
    id: String,
    title: String,
    code: String,
}

#[derive(Debug, Clone, Deserialize)]
struct UnitFile {
    id: String,
    title: String,
    learning_objectives: Vec<String>,
    prerequisite_unit_ids: Vec<String>,
    explanation: String,
    examples: Vec<UnitExample>,
    compiler_error_examples: Vec<String>,
    exercise_id: String,
    completion_criteria: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExerciseFile {
    id: String,
    starter_file: String,
    template_path: String,
    hints: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UnitExerciseIndex {
    id: String,
    exercise_id: String,
}

#[derive(Debug, Deserialize)]
struct RoadmapFile {
    id: String,
    title: String,
    modules: Vec<RoadmapModuleFile>,
}

#[derive(Debug, Deserialize)]
struct RoadmapModuleFile {
    id: String,
    title: String,
    unit_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct LoadedRoadmap {
    id: String,
    title: String,
    modules: Vec<LoadedRoadmapModule>,
}

#[derive(Debug, Clone)]
struct LoadedRoadmapModule {
    id: String,
    title: String,
    units: Vec<LoadedRoadmapUnit>,
}

#[derive(Debug, Clone)]
struct LoadedRoadmapUnit {
    slug: String,
    unit: UnitFile,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/me", get(me))
        .route("/api/v1/attempts/submissions", post(submit_attempt))
        .route("/api/v1/attempts/{attempt_id}", get(get_attempt))
        .route("/api/v1/roadmaps/foundations", get(foundations_roadmap))
        .route("/api/v1/progress/overview", get(progress_overview))
        .route("/api/v1/units/{unit_id}", get(unit_by_id))
        .layer(axum::middleware::from_fn(request_id_middleware))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}

async fn me(user: AuthenticatedUser) -> impl IntoResponse {
    Json(MeResponse {
        subject: user.subject,
        roles: user.roles,
    })
}

async fn submit_attempt(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<SubmissionRequest>,
) -> AppResult<Json<SubmissionResponse>> {
    ensure_role(&user, &["LEARNER", "AUTHOR", "ADMIN"])?;

    if payload.code.trim().is_empty() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "SUBMISSION_EMPTY_CODE",
            "submission code cannot be empty",
        ));
    }

    let attempt_id = Uuid::new_v4();
    let user_id = user.subject;

    sqlx::query("INSERT INTO users (id) VALUES ($1) ON CONFLICT (id) DO NOTHING")
        .bind(&user_id)
        .execute(&state.db)
        .await
        .map_err(|error| {
            error!(%error, "failed to upsert user");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "USER_UPSERT_FAILED",
                "failed to initialize user",
            )
        })?;

    sqlx::query(
        r#"
        INSERT INTO attempts (id, user_id, exercise_id, exercise_version, status, submission_code)
        VALUES ($1, $2, $3, $4, 'QUEUED', $5)
        "#,
    )
    .bind(attempt_id)
    .bind(&user_id)
    .bind(&payload.exercise_id)
    .bind(1_i32)
    .bind(&payload.code)
    .execute(&state.db)
    .await
    .map_err(|error| {
        error!(%error, "failed to create attempt");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "ATTEMPT_CREATE_FAILED",
            "failed to create attempt",
        )
    })?;

    if let Err(error) = update_progress(&state, attempt_id).await {
        error!(%error, attempt_id = %attempt_id, "failed to record queued progress");
    }

    let state_for_worker = state.clone();
    let exercise_id = payload.exercise_id.clone();
    let code = payload.code.clone();

    tokio::spawn(async move {
        if let Err(error) = process_attempt(state_for_worker, attempt_id, exercise_id, code).await {
            error!(%error, attempt_id = %attempt_id, "attempt processing failed");
        }
    });

    Ok(Json(SubmissionResponse {
        attempt_id,
        status: "QUEUED".to_string(),
    }))
}

async fn get_attempt(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(attempt_id): Path<Uuid>,
) -> AppResult<Json<AttemptResponse>> {
    ensure_role(&user, &["LEARNER", "AUTHOR", "ADMIN"])?;

    let row = sqlx::query(
        r#"
        SELECT id, exercise_id, status, COALESCE(stdout, ''), COALESCE(stderr, ''), duration_ms
        FROM attempts
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(attempt_id)
    .bind(&user.subject)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "ATTEMPT_READ_FAILED",
            "failed to read attempt",
        )
    })?
    .ok_or_else(|| {
        AppError::new(
            StatusCode::NOT_FOUND,
            "ATTEMPT_NOT_FOUND",
            "attempt not found",
        )
    })?;

    Ok(Json(AttemptResponse {
        attempt_id: row.get::<Uuid, _>(0),
        exercise_id: row.get::<String, _>(1),
        status: row.get::<String, _>(2),
        stdout: row.get::<String, _>(3),
        stderr: row.get::<String, _>(4),
        duration_ms: row.get::<Option<i64>, _>(5),
    }))
}

async fn foundations_roadmap() -> AppResult<Json<RoadmapResponse>> {
    let roadmap = load_foundations_roadmap().await?;

    Ok(Json(RoadmapResponse {
        id: roadmap.id,
        title: roadmap.title,
        modules: roadmap
            .modules
            .into_iter()
            .map(|module| RoadmapModuleResponse {
                id: module.id,
                title: module.title,
                units: module
                    .units
                    .into_iter()
                    .map(|unit| RoadmapUnitResponse {
                        id: unit.unit.id,
                        slug: unit.slug,
                        title: unit.unit.title,
                        prerequisite_unit_ids: unit.unit.prerequisite_unit_ids,
                        exercise_id: unit.unit.exercise_id,
                    })
                    .collect(),
            })
            .collect(),
    }))
}

async fn progress_overview(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<Json<ProgressOverviewResponse>> {
    ensure_role(&user, &["LEARNER", "AUTHOR", "ADMIN"])?;

    let roadmap = load_foundations_roadmap().await?;
    let rows = sqlx::query(
        r#"
        SELECT
          unit_id,
          status,
          completed_attempts,
          latest_attempt_id
        FROM progress
        WHERE user_id = $1
        "#,
    )
    .bind(&user.subject)
    .fetch_all(&state.db)
    .await
    .map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "PROGRESS_READ_FAILED",
            "failed to read progress",
        )
    })?;

    let mut progress_by_unit = HashMap::new();
    for row in rows {
        progress_by_unit.insert(
            row.get::<String, _>(0),
            (
                row.get::<String, _>(1),
                row.get::<i64, _>(2),
                row.get::<Option<Uuid>, _>(3),
            ),
        );
    }

    let units = roadmap
        .modules
        .iter()
        .flat_map(|module| module.units.iter())
        .map(|unit| {
            let (status, completed_attempts, latest_attempt_id) = progress_by_unit
                .get(&unit.unit.id)
                .cloned()
                .unwrap_or_else(|| ("NOT_STARTED".to_string(), 0, None));

            ProgressUnitResponse {
                unit_id: unit.unit.id.clone(),
                unit_slug: unit.slug.clone(),
                unit_title: unit.unit.title.clone(),
                status,
                completed_attempts,
                latest_attempt_id,
            }
        })
        .collect::<Vec<_>>();

    let resume_unit = units
        .iter()
        .find(|unit| unit.status != "PASSED" && unit.status != "MASTERED")
        .or_else(|| units.last())
        .ok_or_else(|| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "ROADMAP_EMPTY",
                "roadmap must contain at least one unit",
            )
        })?;

    Ok(Json(ProgressOverviewResponse {
        resume_unit_id: resume_unit.unit_id.clone(),
        resume_unit_slug: resume_unit.unit_slug.clone(),
        resume_unit_title: resume_unit.unit_title.clone(),
        units,
    }))
}

async fn unit_by_id(Path(unit_id): Path<String>) -> AppResult<Json<UnitResponse>> {
    let valid_unit_id = unit_id
        .chars()
        .all(|value| value.is_ascii_lowercase() || value == '-');
    if !valid_unit_id {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "UNIT_INVALID_ID",
            "invalid unit id",
        ));
    }

    let unit = load_unit_file_by_slug(&unit_id).await?;
    let exercise = load_exercise_file_by_slug(&unit_id).await?;
    if unit.exercise_id != exercise.id {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "EXERCISE_MISMATCH",
            "unit exercise mismatch",
        ));
    }

    let template_root = tokio::fs::canonicalize(&exercise.template_path)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "EXERCISE_READ_FAILED",
                "failed to load exercise template",
            )
        })?;
    let starter_path = template_root.join(&exercise.starter_file);
    let starter_path = tokio::fs::canonicalize(starter_path).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "EXERCISE_READ_FAILED",
            "failed to load starter code",
        )
    })?;
    if !starter_path.starts_with(&template_root) {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "EXERCISE_INVALID_PATH",
            "invalid exercise starter path",
        ));
    }
    let starter_code = tokio::fs::read_to_string(starter_path).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "EXERCISE_READ_FAILED",
            "failed to load starter code",
        )
    })?;

    Ok(Json(UnitResponse {
        id: unit.id,
        title: unit.title,
        learning_objectives: unit.learning_objectives,
        explanation: unit.explanation,
        examples: unit.examples,
        compiler_error_examples: unit.compiler_error_examples,
        exercise_id: unit.exercise_id,
        completion_criteria: unit.completion_criteria,
        hints: exercise.hints,
        starter_code,
    }))
}

/// Resolves the repository's `content` directory regardless of the process's
/// current working directory. `cargo run`/production deployments execute
/// with the workspace root as the working directory, while `cargo test`
/// executes with the crate's manifest directory as the working directory, so
/// a plain relative `content/...` path only resolves in one of those cases.
fn content_root() -> PathBuf {
    if let Ok(dir) = std::env::var("CONTENT_DIR") {
        return PathBuf::from(dir);
    }

    let candidates = [
        PathBuf::from("content"),
        FsPath::new(env!("CARGO_MANIFEST_DIR")).join("../../content"),
    ];

    candidates
        .into_iter()
        .find(|candidate| candidate.is_dir())
        .unwrap_or_else(|| PathBuf::from("content"))
}

async fn load_foundations_roadmap() -> AppResult<LoadedRoadmap> {
    let roadmap_path = content_root().join("roadmaps/foundations.json");
    let roadmap_data = tokio::fs::read_to_string(&roadmap_path)
        .await
        .map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                return AppError::new(
                    StatusCode::NOT_FOUND,
                    "ROADMAP_NOT_FOUND",
                    "roadmap not found",
                );
            }

            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "ROADMAP_READ_FAILED",
                "failed to load roadmap",
            )
        })?;
    let roadmap: RoadmapFile = serde_json::from_str(&roadmap_data).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "ROADMAP_PARSE_FAILED",
            "failed to parse roadmap",
        )
    })?;

    let mut modules = Vec::with_capacity(roadmap.modules.len());
    for module in roadmap.modules {
        let mut units = Vec::with_capacity(module.unit_ids.len());
        for unit_id in module.unit_ids {
            let slug = unit_id
                .split('.')
                .nth(2)
                .ok_or_else(|| {
                    AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "UNIT_INVALID_ID",
                        "failed to derive unit slug",
                    )
                })?
                .to_string();
            let unit = load_unit_file_by_slug(&slug).await?;
            units.push(LoadedRoadmapUnit { slug, unit });
        }

        modules.push(LoadedRoadmapModule {
            id: module.id,
            title: module.title,
            units,
        });
    }

    Ok(LoadedRoadmap {
        id: roadmap.id,
        title: roadmap.title,
        modules,
    })
}

async fn load_unit_file_by_slug(unit_slug: &str) -> AppResult<UnitFile> {
    let unit_path = content_root().join(format!("units/{unit_slug}.json"));
    let unit_data = tokio::fs::read_to_string(&unit_path)
        .await
        .map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                return AppError::new(StatusCode::NOT_FOUND, "UNIT_NOT_FOUND", "unit not found");
            }

            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "UNIT_READ_FAILED",
                "failed to load unit",
            )
        })?;

    serde_json::from_str(&unit_data).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "UNIT_PARSE_FAILED",
            "failed to parse unit",
        )
    })
}

async fn load_exercise_file_by_slug(unit_slug: &str) -> AppResult<ExerciseFile> {
    let exercise_path = content_root().join(format!("exercises/{unit_slug}.json"));
    let exercise_data = tokio::fs::read_to_string(&exercise_path)
        .await
        .map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                return AppError::new(
                    StatusCode::NOT_FOUND,
                    "EXERCISE_NOT_FOUND",
                    "exercise not found",
                );
            }

            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "EXERCISE_READ_FAILED",
                "failed to load exercise",
            )
        })?;

    serde_json::from_str(&exercise_data).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "EXERCISE_PARSE_FAILED",
            "failed to parse exercise",
        )
    })
}

async fn process_attempt(
    state: AppState,
    attempt_id: Uuid,
    exercise_id: String,
    code: String,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE attempts SET status = 'RUNNING', updated_at = NOW() WHERE id = $1")
        .bind(attempt_id)
        .execute(&state.db)
        .await?;

    if let Err(error) = update_progress(&state, attempt_id).await {
        error!(%error, attempt_id = %attempt_id, "failed to record running progress");
    }

    let result = execute_submission(
        &state.http_client,
        &state.config.executor_base_url,
        ExecuteRequest {
            attempt_id,
            exercise_id,
            code,
        },
    )
    .await;

    match result {
        Ok(executor_result) => {
            sqlx::query(
                r#"
                UPDATE attempts
                SET status = $2, stdout = $3, stderr = $4, duration_ms = $5, updated_at = NOW()
                WHERE id = $1
                "#,
            )
            .bind(attempt_id)
            .bind(executor_result.status)
            .bind(executor_result.stdout)
            .bind(executor_result.stderr)
            .bind(executor_result.duration_ms as i64)
            .execute(&state.db)
            .await?;

            update_progress(&state, attempt_id).await?;
        }
        Err(error) => {
            sqlx::query(
                "UPDATE attempts SET status = 'ERROR', stderr = $2, updated_at = NOW() WHERE id = $1",
            )
            .bind(attempt_id)
            .bind(error.to_string())
            .execute(&state.db)
            .await?;
        }
    }

    Ok(())
}

async fn update_progress(state: &AppState, attempt_id: Uuid) -> anyhow::Result<()> {
    let row = sqlx::query("SELECT user_id, status, exercise_id FROM attempts WHERE id = $1")
        .bind(attempt_id)
        .fetch_one(&state.db)
        .await?;

    let user_id: String = row.get(0);
    let attempt_status: String = row.get(1);
    let exercise_id: String = row.get(2);
    let unit_id = load_unit_id_for_exercise(&exercise_id).await?;

    let progress_status = match attempt_status.as_str() {
        "PASSED" => "PASSED",
        "RUNNING" | "QUEUED" => "STARTED",
        _ => "ATTEMPTED",
    };

    sqlx::query(
        r#"
        INSERT INTO progress (user_id, unit_id, status, completed_attempts, latest_attempt_id)
        VALUES ($1, $2, $3, CASE WHEN $3 = 'PASSED' THEN 1 ELSE 0 END, $4)
        ON CONFLICT (user_id, unit_id)
        DO UPDATE SET
          status = EXCLUDED.status,
          completed_attempts = progress.completed_attempts + CASE WHEN EXCLUDED.status = 'PASSED' THEN 1 ELSE 0 END,
          latest_attempt_id = EXCLUDED.latest_attempt_id,
          updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(unit_id)
    .bind(progress_status)
    .bind(attempt_id)
    .execute(&state.db)
    .await?;

    Ok(())
}

async fn load_unit_id_for_exercise(exercise_id: &str) -> anyhow::Result<String> {
    let mut unit_entries = tokio::fs::read_dir(content_root().join("units")).await?;

    while let Some(entry) = unit_entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }

        let unit_data = tokio::fs::read_to_string(path).await?;
        let unit: UnitExerciseIndex = serde_json::from_str(&unit_data)?;
        if unit.exercise_id == exercise_id {
            return Ok(unit.id);
        }
    }

    anyhow::bail!("unsupported exercise_id: {exercise_id}")
}

async fn request_id_middleware(request: Request<axum::body::Body>, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "x-request-id",
        axum::http::HeaderValue::from_str(&request_id)
            .unwrap_or(axum::http::HeaderValue::from_static("invalid")),
    );

    info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = %response.status(),
        "request handled"
    );

    response
}

fn ensure_role(user: &AuthenticatedUser, allowed_roles: &[&str]) -> AppResult<()> {
    let has_role = user
        .roles
        .iter()
        .any(|role| allowed_roles.iter().any(|allowed| role == allowed));

    if has_role {
        return Ok(());
    }

    Err(AppError::new(
        StatusCode::FORBIDDEN,
        "AUTH_FORBIDDEN",
        "insufficient role permissions",
    ))
}

pub async fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": {
                "code": "NOT_FOUND",
                "message": "not found"
            }
        })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use axum::{
        Json, Router,
        body::{Body, to_bytes},
        http::{
            Request, StatusCode,
            header::{AUTHORIZATION, CONTENT_TYPE},
        },
        routing::post,
    };
    use jsonwebtoken::{EncodingKey, Header, encode};
    use serde::{Deserialize, Serialize, de::DeserializeOwned};
    use sqlx::{PgPool, postgres::PgPoolOptions};
    use std::{sync::Arc, time::Duration};
    use tokio::{net::TcpListener, time::sleep};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{AppState, auth::Authenticator, config::Config};

    use super::{
        AttemptResponse, ProgressOverviewResponse, RoadmapResponse, SubmissionResponse, router,
    };

    #[derive(Serialize)]
    struct Claims<'a> {
        sub: &'a str,
        iss: &'a str,
        aud: &'a str,
        exp: usize,
        realm_access: RealmAccess<'a>,
    }

    #[derive(Serialize)]
    struct RealmAccess<'a> {
        roles: Vec<&'a str>,
    }

    #[derive(Deserialize)]
    struct MockExecuteRequest {
        exercise_id: String,
        code: String,
    }

    fn test_token(secret: &str, subject: &str, roles: &[&str]) -> String {
        encode(
            &Header::default(),
            &Claims {
                sub: subject,
                iss: "test-issuer",
                aud: "test-audience",
                exp: 4_000_000_000,
                realm_access: RealmAccess {
                    roles: roles.to_vec(),
                },
            },
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("token created")
    }

    async fn mock_execute(Json(payload): Json<MockExecuteRequest>) -> Json<serde_json::Value> {
        let (status, stderr) = match payload.exercise_id.as_str() {
            "exercise.rust.variables.mutable-counter.v1" => (
                if payload.code.contains("counter += 1") {
                    "PASSED"
                } else {
                    "FAILED"
                },
                String::new(),
            ),
            "exercise.rust.control-flow.classify-number.v1" => (
                if payload.code.contains("else if value < 0") && payload.code.contains("\"zero\"") {
                    "PASSED"
                } else {
                    "FAILED"
                },
                String::new(),
            ),
            _ => ("ERROR", "unsupported exercise".to_string()),
        };

        Json(serde_json::json!({
            "status": status,
            "stdout": "mock executor",
            "stderr": stderr,
            "duration_ms": 5
        }))
    }

    async fn spawn_mock_executor() -> Option<String> {
        let listener = TcpListener::bind("127.0.0.1:0").await.ok()?;
        let address = listener.local_addr().ok()?;
        let app = Router::new().route("/execute", post(mock_execute));

        tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        Some(format!("http://{address}"))
    }

    async fn test_state() -> Option<AppState> {
        let Ok(database_url) = std::env::var("TEST_DATABASE_URL") else {
            return None;
        };
        let executor_base_url = spawn_mock_executor().await?;

        let pool: PgPool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .ok()?;
        sqlx::migrate!("./migrations").run(&pool).await.ok()?;

        let config = Config {
            host: "127.0.0.1".to_string(),
            port: 0,
            database_url,
            executor_base_url,
            jwt_issuer: "test-issuer".to_string(),
            jwt_audience: "test-audience".to_string(),
            jwt_jwks_url: String::new(),
            jwt_hs256_secret: Some("secret".to_string()),
        };

        Some(AppState {
            db: pool,
            auth: Authenticator::hs256(
                "secret".to_string(),
                "test-issuer".to_string(),
                "test-audience".to_string(),
            ),
            http_client: reqwest::Client::new(),
            config: Arc::new(config),
        })
    }

    async fn response_json<T: DeserializeOwned>(response: axum::response::Response) -> T {
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body");
        serde_json::from_slice(&bytes).expect("valid json body")
    }

    fn bearer_header(token: &str) -> String {
        format!("Bearer {token}")
    }

    async fn wait_for_attempt(app: &Router, token: &str, attempt_id: Uuid) -> AttemptResponse {
        for _ in 0..40 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(format!("/api/v1/attempts/{attempt_id}"))
                        .header(AUTHORIZATION, bearer_header(token))
                        .body(Body::empty())
                        .expect("valid request"),
                )
                .await
                .expect("request handled");

            let attempt: AttemptResponse = response_json(response).await;
            if attempt.status != "QUEUED" && attempt.status != "RUNNING" {
                return attempt;
            }

            sleep(Duration::from_millis(50)).await;
        }

        panic!("attempt did not finish in time");
    }

    #[tokio::test]
    async fn health_endpoint_is_available() {
        let Some(state) = test_state().await else {
            return;
        };

        let response = router(state)
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn protected_endpoint_requires_token() {
        let Some(state) = test_state().await else {
            return;
        };

        let response = router(state)
            .oneshot(
                Request::builder()
                    .uri("/api/v1/me")
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn protected_endpoint_accepts_valid_token() {
        let Some(state) = test_state().await else {
            return;
        };

        let token = test_token("secret", "test-user-auth", &[]);

        let response = router(state)
            .oneshot(
                Request::builder()
                    .uri("/api/v1/me")
                    .header(AUTHORIZATION, bearer_header(&token))
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn roadmap_endpoint_lists_current_foundations_units() {
        let Some(state) = test_state().await else {
            return;
        };

        let response = router(state)
            .oneshot(
                Request::builder()
                    .uri("/api/v1/roadmaps/foundations")
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(response.status(), StatusCode::OK);

        let roadmap: RoadmapResponse = response_json(response).await;
        let slugs = roadmap.modules[0]
            .units
            .iter()
            .map(|unit| unit.slug.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            slugs,
            vec![
                "variables",
                "functions",
                "control-flow",
                "ownership",
                "borrowing",
                "references",
                "slices"
            ]
        );
    }

    #[tokio::test]
    async fn multi_unit_submissions_update_progress_and_resume_target() {
        let Some(state) = test_state().await else {
            return;
        };

        let app = router(state);
        let token = test_token("secret", "test-user-progress", &["LEARNER"]);

        let variables_submission = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/attempts/submissions")
                    .header(AUTHORIZATION, bearer_header(&token))
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "exercise_id": "exercise.rust.variables.mutable-counter.v1",
                            "code": "pub fn increment_counter() -> i32 { let mut counter = 0; counter += 1; counter }\n#[cfg(test)] mod tests { use super::increment_counter; #[test] fn increments_from_zero_to_one() { assert_eq!(increment_counter(), 1); } }"
                        })
                        .to_string(),
                    ))
                    .expect("valid request"),
            )
            .await
            .expect("request handled");
        assert_eq!(variables_submission.status(), StatusCode::OK);
        let variables_submission: SubmissionResponse = response_json(variables_submission).await;
        let variables_attempt =
            wait_for_attempt(&app, &token, variables_submission.attempt_id).await;
        assert_eq!(variables_attempt.status, "PASSED");

        let control_flow_submission = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/attempts/submissions")
                    .header(AUTHORIZATION, bearer_header(&token))
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "exercise_id": "exercise.rust.control-flow.classify-number.v1",
                            "code": "pub fn classify_number(value: i32) -> &'static str { if value > 0 { \"positive\" } else if value < 0 { \"negative\" } else { \"zero\" } }\n#[cfg(test)] mod tests { use super::classify_number; #[test] fn classifies_negative_numbers() { assert_eq!(classify_number(-4), \"negative\"); } #[test] fn classifies_zero_separately() { assert_eq!(classify_number(0), \"zero\"); } #[test] fn classifies_positive_numbers() { assert_eq!(classify_number(7), \"positive\"); } }"
                        })
                        .to_string(),
                    ))
                    .expect("valid request"),
            )
            .await
            .expect("request handled");
        assert_eq!(control_flow_submission.status(), StatusCode::OK);
        let control_flow_submission: SubmissionResponse =
            response_json(control_flow_submission).await;
        let control_flow_attempt =
            wait_for_attempt(&app, &token, control_flow_submission.attempt_id).await;
        assert_eq!(control_flow_attempt.status, "PASSED");

        let progress_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/progress/overview")
                    .header(AUTHORIZATION, bearer_header(&token))
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(progress_response.status(), StatusCode::OK);
        let progress: ProgressOverviewResponse = response_json(progress_response).await;

        assert_eq!(progress.resume_unit_slug, "functions");
        assert_eq!(progress.units.len(), 7);
        assert_eq!(
            progress
                .units
                .iter()
                .find(|unit| unit.unit_slug == "variables")
                .map(|unit| unit.status.as_str()),
            Some("PASSED")
        );
        assert_eq!(
            progress
                .units
                .iter()
                .find(|unit| unit.unit_slug == "control-flow")
                .map(|unit| unit.status.as_str()),
            Some("PASSED")
        );
        assert_eq!(
            progress
                .units
                .iter()
                .find(|unit| unit.unit_slug == "functions")
                .map(|unit| unit.status.as_str()),
            Some("NOT_STARTED")
        );
    }

    #[tokio::test]
    async fn submission_immediately_records_started_progress() {
        let Some(state) = test_state().await else {
            return;
        };

        let app = router(state);
        let token = test_token("secret", "test-user-started-progress", &["LEARNER"]);

        let submission = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/attempts/submissions")
                    .header(AUTHORIZATION, bearer_header(&token))
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "exercise_id": "exercise.rust.variables.mutable-counter.v1",
                            "code": "pub fn increment_counter() -> i32 { let mut counter = 0; counter += 1; counter }\n#[cfg(test)] mod tests { use super::increment_counter; #[test] fn increments_from_zero_to_one() { assert_eq!(increment_counter(), 1); } }"
                        })
                        .to_string(),
                    ))
                    .expect("valid request"),
            )
            .await
            .expect("request handled");
        assert_eq!(submission.status(), StatusCode::OK);

        // The progress row must be recorded before the queued submission
        // response is returned, so the dashboard reflects a started unit
        // immediately instead of waiting for the attempt to finish.
        let progress_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/progress/overview")
                    .header(AUTHORIZATION, bearer_header(&token))
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(progress_response.status(), StatusCode::OK);
        let progress: ProgressOverviewResponse = response_json(progress_response).await;

        let variables_status = progress
            .units
            .iter()
            .find(|unit| unit.unit_slug == "variables")
            .map(|unit| unit.status.as_str());

        assert_ne!(variables_status, Some("NOT_STARTED"));
        assert!(variables_status.is_some());
    }
}
