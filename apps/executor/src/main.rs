use std::{
    env, io,
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
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    net::TcpListener,
    process::{Child, Command},
};
use tracing::{debug, error, info};
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

    debug!(?config, "executor configuration initialized");

    tokio::fs::create_dir_all(&config.workdir_root)
        .await
        .context("failed to initialize executor workdir")?;
    debug!(workdir = ?config.workdir_root, "executor workdir initialized");

    let bind_address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&bind_address)
        .await
        .context("failed to bind executor listener")?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/execute", post(execute))
        .with_state(config);

    info!(address = %bind_address, "executor service started");
    axum::serve(listener, app)
        .await
        .context("executor stopped unexpectedly")?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();
}

async fn health() -> Json<HealthResponse> {
    info!("health check API hit");
    debug!("health check responding ok");
    Json(HealthResponse { status: "ok" })
}

async fn execute(
    axum::extract::State(config): axum::extract::State<Config>,
    Json(payload): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!(
        attempt_id = %payload.attempt_id,
        exercise_id = %payload.exercise_id,
        "execute API hit"
    );
    let start = Instant::now();

    debug!(
        attempt_id = %payload.attempt_id,
        exercise_id = %payload.exercise_id,
        code_bytes = payload.code.len(),
        "starting exercise execution"
    );
    let result = execute_inner(&config, &payload).await;

    match result {
        Ok((status, stdout, stderr)) => {
            let duration_ms = start.elapsed().as_millis() as u64;
            debug!(
                attempt_id = %payload.attempt_id,
                status = %status,
                duration_ms = duration_ms,
                "execution completed successfully"
            );
            Ok(Json(ExecuteResponse {
                status,
                stdout,
                stderr,
                duration_ms,
            }))
        }
        Err(error) => {
            error!(%error, attempt_id = %payload.attempt_id, "execution failed");
            debug!(attempt_id = %payload.attempt_id, %error, "execution failed with internal error");
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
    debug!(
        attempt_id = %payload.attempt_id,
        exercise_id = %payload.exercise_id,
        "mapping exercise template path"
    );
    let template_path = map_template_path(&payload.exercise_id)?;
    let tempdir = Builder::new()
        .prefix("attempt-")
        .tempdir_in(&config.workdir_root)
        .context("failed to create temp workspace")?;

    debug!(
        attempt_id = %payload.attempt_id,
        template_path = ?template_path,
        tempdir = ?tempdir.path(),
        "copying template to temp workspace"
    );
    copy_dir_recursive(template_path, tempdir.path())
        .await
        .context("failed to copy template")?;

    let starter_path = tempdir.path().join("src/lib.rs");
    debug!(
        attempt_id = %payload.attempt_id,
        starter_path = ?starter_path,
        "writing learner submission code"
    );
    tokio::fs::write(&starter_path, &payload.code)
        .await
        .context("failed to write learner submission")?;

    debug!(attempt_id = %payload.attempt_id, "running cargo check");
    let check = run_command(
        "cargo",
        &["check"],
        tempdir.path(),
        config.timeout,
        config.output_limit_bytes,
    )
    .await?;

    if check.timed_out {
        debug!(attempt_id = %payload.attempt_id, "cargo check timed out");
        return Ok((
            "TIMEOUT".to_string(),
            check.stdout,
            format!("cargo check timed out\n{}", check.stderr),
        ));
    }

    if !check.success {
        debug!(attempt_id = %payload.attempt_id, "cargo check failed");
        return Ok(("FAILED".to_string(), check.stdout, check.stderr));
    }

    debug!(attempt_id = %payload.attempt_id, "cargo check passed, running cargo test");
    let tests = run_command(
        "cargo",
        &["test", "--quiet"],
        tempdir.path(),
        config.timeout,
        config.output_limit_bytes,
    )
    .await?;

    if tests.timed_out {
        debug!(attempt_id = %payload.attempt_id, "cargo test timed out");
        return Ok((
            "TIMEOUT".to_string(),
            format!("{}\n{}", check.stdout, tests.stdout),
            format!("cargo test timed out\n{}", tests.stderr),
        ));
    }

    let combined_stdout = format!("{}\n{}", check.stdout, tests.stdout);
    let combined_stderr = format!("{}\n{}", check.stderr, tests.stderr);

    let status = if tests.success { "PASSED" } else { "FAILED" };
    debug!(
        attempt_id = %payload.attempt_id,
        status = %status,
        test_success = tests.success,
        "cargo test finished"
    );

    Ok((
        status.to_string(),
        truncate_output(combined_stdout, config.output_limit_bytes),
        truncate_output(combined_stderr, config.output_limit_bytes),
    ))
}

fn map_template_path(exercise_id: &str) -> anyhow::Result<&'static Path> {
    debug!(exercise_id = %exercise_id, "mapping template path for exercise");
    match exercise_id {
        "exercise.rust.variables.mutable-counter.v1" => {
            Ok(Path::new("content/exercises/variables/v1/template"))
        }
        "exercise.rust.functions.rectangle-area.v1" => {
            Ok(Path::new("content/exercises/functions/v1/template"))
        }
        "exercise.rust.ownership.print-twice.v1" => {
            Ok(Path::new("content/exercises/ownership/v1/template"))
        }
        "exercise.rust.control-flow.classify-number.v1" => {
            Ok(Path::new("content/exercises/control-flow/v1/template"))
        }
        "exercise.rust.borrowing.longer-label.v1" => {
            Ok(Path::new("content/exercises/borrowing/v1/template"))
        }
        "exercise.rust.references.append-rust.v1" => {
            Ok(Path::new("content/exercises/references/v1/template"))
        }
        "exercise.rust.slices.first-word.v1" => {
            Ok(Path::new("content/exercises/slices/v1/template"))
        }
        "exercise.rust.types.tuple-basics.v1" => {
            Ok(Path::new("content/exercises/types/v1/template"))
        }
        "exercise.rust.pattern-matching.classify-value.v1" => {
            Ok(Path::new("content/exercises/pattern-matching/v1/template"))
        }
        "exercise.rust.modules.public-api.v1" => {
            Ok(Path::new("content/exercises/modules/v1/template"))
        }
        "exercise.rust.structs.build-profile.v1" => {
            Ok(Path::new("content/exercises/structs/v1/template"))
        }
        _ => {
            debug!(exercise_id = %exercise_id, "unsupported exercise template requested");
            anyhow::bail!("unsupported exercise_id: {exercise_id}")
        }
    }
}

async fn copy_dir_recursive(from: &Path, to: &Path) -> anyhow::Result<()> {
    debug!(from = ?from, to = ?to, "copying directory recursively");
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
    debug!(
        program = %program,
        args = ?args,
        cwd = ?cwd,
        timeout_ms = timeout.as_millis() as u64,
        output_limit = output_limit,
        "running command"
    );
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(unix)]
    command.process_group(0);

    let mut child = command.spawn()?;
    let process_id = child.id();
    debug!(program = %program, pid = ?process_id, "spawned child process");
    let stdout = child
        .stdout
        .take()
        .context("failed to capture command stdout")?;
    let stderr = child
        .stderr
        .take()
        .context("failed to capture command stderr")?;
    let mut stdout_task = tokio::spawn(read_limited(stdout, output_limit));
    let mut stderr_task = tokio::spawn(read_limited(stderr, output_limit));

    let command_result = tokio::time::timeout(timeout, async {
        let status = child.wait().await?;
        let stdout = (&mut stdout_task)
            .await
            .context("failed to read command stdout")??;
        let stderr = (&mut stderr_task)
            .await
            .context("failed to read command stderr")??;
        Ok::<_, anyhow::Error>((status.success(), stdout, stderr))
    })
    .await;

    match command_result {
        Ok(result) => {
            let (success, stdout, stderr) = result?;
            debug!(
                program = %program,
                pid = ?process_id,
                success = success,
                "command finished execution"
            );
            Ok(CommandResult {
                success,
                timed_out: false,
                stdout,
                stderr,
            })
        }
        Err(_) => {
            debug!(
                program = %program,
                pid = ?process_id,
                "command execution timed out, terminating process"
            );
            terminate_process_group(process_id, &mut child).await?;
            child.wait().await?;
            let stdout = stdout_task
                .await
                .context("failed to read command stdout")??;
            let stderr = stderr_task
                .await
                .context("failed to read command stderr")??;
            Ok(CommandResult {
                success: false,
                timed_out: true,
                stdout,
                stderr,
            })
        }
    }
}

async fn read_limited(
    mut reader: impl AsyncRead + Unpin,
    output_limit: usize,
) -> io::Result<String> {
    let mut output = Vec::with_capacity(output_limit.min(8192));
    let mut buffer = [0; 8192];
    let mut truncated = false;

    loop {
        let bytes_read = reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        let bytes_to_keep = output_limit.saturating_sub(output.len()).min(bytes_read);
        output.extend_from_slice(&buffer[..bytes_to_keep]);
        truncated |= bytes_to_keep < bytes_read;
    }

    let mut output = String::from_utf8_lossy(&output).to_string();
    if truncated {
        output.push_str("\n...[truncated]");
    }

    Ok(output)
}

async fn terminate_process_group(process_id: Option<u32>, child: &mut Child) -> anyhow::Result<()> {
    debug!(pid = ?process_id, "terminating process group");
    #[cfg(unix)]
    let _ = child;
    #[cfg(unix)]
    {
        let Some(pid) = process_id else {
            return Ok(());
        };
        let process_group = i32::try_from(pid).context("process id exceeds supported range")?;
        let result = unsafe { libc::kill(-process_group, libc::SIGKILL) };
        if result == -1 {
            let error = io::Error::last_os_error();
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error.into());
            }
        }
    }

    #[cfg(not(unix))]
    child.kill().await?;

    Ok(())
}

fn truncate_output(value: String, max: usize) -> String {
    if value.len() <= max {
        return value;
    }

    debug!(
        original_len = value.len(),
        max = max,
        "truncating command output"
    );
    let mut output = value;
    output.truncate(max);
    output.push_str("\n...[truncated]");
    output
}

#[cfg(all(test, unix))]
mod tests {
    use super::run_command;
    use std::time::Duration;

    #[tokio::test]
    async fn command_output_is_bounded() {
        let workdir = tempfile::tempdir().expect("temporary working directory");
        let result = run_command(
            "sh",
            &["-c", "printf '%2048s' ''"],
            workdir.path(),
            Duration::from_secs(2),
            32,
        )
        .await
        .expect("command ran");

        assert!(result.success);
        assert!(result.stdout.starts_with(&" ".repeat(32)));
        assert!(result.stdout.ends_with("\n...[truncated]"));
        assert!(result.stdout.len() <= 32 + "\n...[truncated]".len());
    }

    #[tokio::test]
    async fn command_timeout_kills_descendants_in_its_process_group() {
        let workdir = tempfile::tempdir().expect("temporary working directory");
        let result = tokio::time::timeout(
            Duration::from_secs(2),
            run_command(
                "sh",
                &["-c", "sleep 10 &"],
                workdir.path(),
                Duration::from_millis(50),
                32,
            ),
        )
        .await
        .expect("timed-out command process group was terminated")
        .expect("command result");

        assert!(result.timed_out);
    }
}
