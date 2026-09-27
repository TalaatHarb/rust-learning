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

#[derive(Debug, Serialize)]
pub struct SubmissionResponse {
    attempt_id: Uuid,
    status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct AttemptResponse {
    attempt_id: Uuid,
    exercise_id: String,
    status: String,
    stdout: String,
    stderr: String,
    duration_ms: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ProgressOverviewResponse {
    unit_id: String,
    status: String,
    completed_attempts: i64,
    latest_attempt_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
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
pub struct UnitExample {
    id: String,
    title: String,
    code: String,
}

#[derive(Debug, Deserialize)]
struct OwnershipUnitFile {
    id: String,
    title: String,
    learning_objectives: Vec<String>,
    explanation: String,
    examples: Vec<UnitExample>,
    compiler_error_examples: Vec<String>,
    exercise_id: String,
    completion_criteria: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OwnershipExerciseFile {
    hints: Vec<String>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/me", get(me))
        .route("/api/v1/attempts/submissions", post(submit_attempt))
        .route("/api/v1/attempts/{attempt_id}", get(get_attempt))
        .route("/api/v1/progress/overview", get(progress_overview))
        .route("/api/v1/units/ownership", get(ownership_unit))
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
        status: "QUEUED",
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

async fn progress_overview(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<Json<ProgressOverviewResponse>> {
    ensure_role(&user, &["LEARNER", "AUTHOR", "ADMIN"])?;

    let row = sqlx::query(
        r#"
        SELECT
          unit_id,
          status,
          completed_attempts,
          latest_attempt_id
        FROM progress
        WHERE user_id = $1 AND unit_id = 'unit.rust.ownership.v1'
        "#,
    )
    .bind(&user.subject)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "PROGRESS_READ_FAILED",
            "failed to read progress",
        )
    })?;

    let response = if let Some(row) = row {
        ProgressOverviewResponse {
            unit_id: row.get::<String, _>(0),
            status: row.get::<String, _>(1),
            completed_attempts: row.get::<i64, _>(2),
            latest_attempt_id: row.get::<Option<Uuid>, _>(3),
        }
    } else {
        ProgressOverviewResponse {
            unit_id: "unit.rust.ownership.v1".to_string(),
            status: "NOT_STARTED".to_string(),
            completed_attempts: 0,
            latest_attempt_id: None,
        }
    };

    Ok(Json(response))
}

async fn ownership_unit() -> AppResult<Json<UnitResponse>> {
    let unit_data = tokio::fs::read_to_string("content/units/ownership.json")
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "UNIT_READ_FAILED",
                "failed to load ownership unit",
            )
        })?;
    let exercise_data = tokio::fs::read_to_string("content/exercises/ownership.json")
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "EXERCISE_READ_FAILED",
                "failed to load ownership exercise",
            )
        })?;
    let starter_code =
        tokio::fs::read_to_string("content/exercises/ownership/v1/template/src/lib.rs")
            .await
            .map_err(|_| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "EXERCISE_READ_FAILED",
                    "failed to load starter code",
                )
            })?;

    let unit: OwnershipUnitFile = serde_json::from_str(&unit_data).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "UNIT_PARSE_FAILED",
            "failed to parse ownership unit",
        )
    })?;
    let exercise: OwnershipExerciseFile = serde_json::from_str(&exercise_data).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "EXERCISE_PARSE_FAILED",
            "failed to parse ownership exercise",
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
    let row = sqlx::query("SELECT user_id, status FROM attempts WHERE id = $1")
        .bind(attempt_id)
        .fetch_one(&state.db)
        .await?;

    let user_id: String = row.get(0);
    let attempt_status: String = row.get(1);

    let progress_status = match attempt_status.as_str() {
        "PASSED" => "PASSED",
        "RUNNING" | "QUEUED" => "STARTED",
        _ => "ATTEMPTED",
    };

    sqlx::query(
        r#"
        INSERT INTO progress (user_id, unit_id, status, completed_attempts, latest_attempt_id)
        VALUES ($1, 'unit.rust.ownership.v1', $2, CASE WHEN $2 = 'PASSED' THEN 1 ELSE 0 END, $3)
        ON CONFLICT (user_id, unit_id)
        DO UPDATE SET
          status = EXCLUDED.status,
          completed_attempts = progress.completed_attempts + CASE WHEN EXCLUDED.status = 'PASSED' THEN 1 ELSE 0 END,
          latest_attempt_id = EXCLUDED.latest_attempt_id,
          updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(progress_status)
    .bind(attempt_id)
    .execute(&state.db)
    .await?;

    Ok(())
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
        body::Body,
        http::{Request, StatusCode, header::AUTHORIZATION},
    };
    use jsonwebtoken::{EncodingKey, Header, encode};
    use serde::Serialize;
    use sqlx::{PgPool, postgres::PgPoolOptions};
    use std::sync::Arc;
    use tower::ServiceExt;

    use crate::{AppState, auth::Authenticator, config::Config};

    use super::router;

    #[derive(Serialize)]
    struct Claims<'a> {
        sub: &'a str,
        iss: &'a str,
        aud: &'a str,
        exp: usize,
    }

    fn test_token(secret: &str) -> String {
        encode(
            &Header::default(),
            &Claims {
                sub: "test-user",
                iss: "test-issuer",
                aud: "test-audience",
                exp: 4_000_000_000,
            },
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("token created")
    }

    async fn test_state() -> Option<AppState> {
        let Ok(database_url) = std::env::var("TEST_DATABASE_URL") else {
            return None;
        };

        let pool: PgPool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .ok()?;

        let config = Config {
            host: "127.0.0.1".to_string(),
            port: 0,
            database_url,
            executor_base_url: "http://127.0.0.1:8082".to_string(),
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

        let token = test_token("secret");

        let response = router(state)
            .oneshot(
                Request::builder()
                    .uri("/api/v1/me")
                    .header(
                        AUTHORIZATION,
                        format!(
                            "{} {}",
                            ['B', 'e', 'a', 'r', 'e', 'r'].iter().collect::<String>(),
                            token
                        ),
                    )
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
