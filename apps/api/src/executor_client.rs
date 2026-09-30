use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ExecuteRequest {
    pub attempt_id: Uuid,
    pub exercise_id: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteResponse {
    pub status: String,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

pub async fn execute_submission(
    client: &Client,
    base_url: &str,
    request: ExecuteRequest,
) -> anyhow::Result<ExecuteResponse> {
    debug!(
        attempt_id = %request.attempt_id,
        exercise_id = %request.exercise_id,
        base_url = %base_url,
        "sending execution request to executor service"
    );
    let response = client
        .post(format!("{base_url}/execute"))
        .json(&request)
        .send()
        .await?
        .error_for_status()?;

    let response_body = response.json::<ExecuteResponse>().await?;
    debug!(
        attempt_id = %request.attempt_id,
        status = %response_body.status,
        duration_ms = response_body.duration_ms,
        "received execution response from executor service"
    );

    Ok(response_body)
}
