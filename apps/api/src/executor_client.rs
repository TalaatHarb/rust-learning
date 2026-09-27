use reqwest::Client;
use serde::{Deserialize, Serialize};
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
    let response = client
        .post(format!("{base_url}/execute"))
        .json(&request)
        .send()
        .await?
        .error_for_status()?;

    Ok(response.json::<ExecuteResponse>().await?)
}
