use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
    time::{Duration, Instant},
};

use anyhow::Context;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tempfile::Builder;
use tokio::{net::TcpListener, process::Command};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    timeout: Duration,
    output_limit_bytes: usize,
    workdir_root: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ExecuteRequest {
    attempt_id: Uuid,
    exercise_id: String,
    code: String,
}

#[derive(Debug, Serialize)]
struct ExecuteResponse {
    status: String,
    stdout: String,
    stderr: String,
    duration_ms: u64,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = Config {
        host: env::var("EXECUTOR_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: env::var("EXECUTOR_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8082),
        timeout: env::var("EXECUTOR_TIMEOUT_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(8)),
        output_limit_bytes: env::var("EXECUTOR_OUTPUT_LIMIT_BYTES")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(20_000),
        workdir_root: env::var("EXECUTOR_WORKDIR_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp/rust-learning-executor")),
    };

    tokio::fs::create_dir_all(&config.workdir_root)
        .await
        .context("failed to initialize executor workdir")?;

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .context("failed to bind executor listener")?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/execute", post(execute))
        .with_state(config);

    info!("executor service started");
    axum::serve(listener, app).await.context("executor stopped unexpectedly")?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn execute(
    axum::extract::State(config): axum::extract::State<Config>,
    Json(payload): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, (StatusCode, Json<serde_json::Value>)> {
    let start = Instant::now();

    let result = execute_inner(&config, &payload).await;

    match result {
        Ok((status, stdout, stderr)) => Ok(Json(ExecuteResponse {
            status,
            stdout,
            stderr,
            duration_ms: start.elapsed().as_millis() as u64,
        })),
        Err(error) => {
            error!(%error, attempt_id = %payload.attempt_id, "execution failed");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "code": "EXECUTION_FAILED",
                        "message": error.to_string()
                    }
                })),
            ))
        }
    }
}

async fn execute_inner(
    config: &Config,
    payload: &ExecuteRequest,
) -> anyhow::Result<(String, String, String)> {
    let template_path = map_template_path(&payload.exercise_id)?;
    let tempdir = Builder::new()
        .prefix("attempt-")
        .tempdir_in(&config.workdir_root)
        .context("failed to create temp workspace")?;

    copy_dir_recursive(template_path, tempdir.path())
        .await
        .context("failed to copy template")?;

    let starter_path = tempdir.path().join("src/lib.rs");
    tokio::fs::write(&starter_path, &payload.code)
        .await
        .context("failed to write learner submission")?;

    let check = run_command(
        "cargo",
        &["check"],
        tempdir.path(),
        config.timeout,
        config.output_limit_bytes,
    )
    .await?;

    if check.timed_out {
        return Ok((
            "TIMEOUT".to_string(),
            check.stdout,
            format!("cargo check timed out\n{}", check.stderr),
        ));
    }

    if !check.success {
        return Ok(("FAILED".to_string(), check.stdout, check.stderr));
    }

    let tests = run_command(
        "cargo",
        &["test", "--quiet"],
        tempdir.path(),
        config.timeout,
        config.output_limit_bytes,
    )
    .await?;

    if tests.timed_out {
        return Ok((
            "TIMEOUT".to_string(),
            format!("{}\n{}", check.stdout, tests.stdout),
            format!("cargo test timed out\n{}", tests.stderr),
        ));
    }

    let combined_stdout = format!("{}\n{}", check.stdout, tests.stdout);
    let combined_stderr = format!("{}\n{}", check.stderr, tests.stderr);

    let status = if tests.success { "PASSED" } else { "FAILED" };

    Ok((
        status.to_string(),
        truncate_output(combined_stdout, config.output_limit_bytes),
        truncate_output(combined_stderr, config.output_limit_bytes),
    ))
}

fn map_template_path(exercise_id: &str) -> anyhow::Result<&'static Path> {
    match exercise_id {
        "exercise.rust.ownership.print-twice.v1" => {
            Ok(Path::new("content/exercises/ownership/v1/template"))
        }
        _ => anyhow::bail!("unsupported exercise_id: {exercise_id}"),
    }
}

async fn copy_dir_recursive(from: &Path, to: &Path) -> anyhow::Result<()> {
    let mut stack = vec![(from.to_path_buf(), to.to_path_buf())];

    while let Some((src, dst)) = stack.pop() {
        tokio::fs::create_dir_all(&dst).await?;

        let mut entries = tokio::fs::read_dir(&src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let target = dst.join(entry.file_name());
            let file_type = entry.file_type().await?;

            if file_type.is_dir() {
                stack.push((path, target));
            } else {
                tokio::fs::copy(path, target).await?;
            }
        }
    }

    Ok(())
}

struct CommandResult {
    success: bool,
    timed_out: bool,
    stdout: String,
    stderr: String,
}

async fn run_command(
    program: &str,
    args: &[&str],
    cwd: &Path,
    timeout: Duration,
    output_limit: usize,
) -> anyhow::Result<CommandResult> {
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = tokio::time::timeout(timeout, command.output()).await;

    match output {
        Ok(Ok(output)) => Ok(CommandResult {
            success: output.status.success(),
            timed_out: false,
            stdout: truncate_output(String::from_utf8_lossy(&output.stdout).to_string(), output_limit),
            stderr: truncate_output(String::from_utf8_lossy(&output.stderr).to_string(), output_limit),
        }),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Ok(CommandResult {
            success: false,
            timed_out: true,
            stdout: String::new(),
            stderr: format!("command timed out after {}s", timeout.as_secs()),
        }),
    }
}

fn truncate_output(value: String, max: usize) -> String {
    if value.len() <= max {
        return value;
    }

    let mut output = value;
    output.truncate(max);
    output.push_str("\n...[truncated]");
    output
}
