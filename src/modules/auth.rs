use serde::Deserialize;
use std::time::Duration;
use std::process::Command;
use anyhow::{Result, anyhow, Context};
use reqwest::Client;

/// Try to get GitHub token from gh CLI (if installed and authenticated).
/// This is the easiest method - no OAuth App registration needed!
pub fn get_token_from_gh_cli() -> Result<String> {
    let output = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .context("Failed to run 'gh' command. Is GitHub CLI installed?")?;
    
    if output.status.success() {
        let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !token.is_empty() {
            tracing::info!("Got token from gh CLI");
            return Ok(token);
        }
    }
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(anyhow!("gh auth token failed: {}. Run 'gh auth login' first.", stderr.trim()))
}

/// Get GitHub OAuth Client ID from environment variable (fallback method).
fn get_client_id() -> Result<String> {
    std::env::var("GITHUB_CLIENT_ID")
        .context("GITHUB_CLIENT_ID environment variable not set. Please set it to your GitHub OAuth App Client ID.")
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PollResponse {
    Success(AccessTokenResponse),
    Error { error: String, error_description: Option<String> },
}

pub async fn request_device_code(client: &Client) -> Result<DeviceCodeResponse> {
    let client_id = get_client_id()?;
    
    let response = client.post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[("client_id", client_id.as_str()), ("scope", "repo user read:org")])
        .send()
        .await?;
    
    let status = response.status();
    let text = response.text().await?;
    
    tracing::debug!("Device code response (status {}): {}", status, text);
    
    if !status.is_success() {
        return Err(anyhow!("GitHub API error ({}): {}", status, text));
    }
    
    serde_json::from_str(&text)
        .map_err(|e| anyhow!("Failed to parse device code response: {}. Raw: {}", e, text))
}

pub async fn poll_access_token(client: &Client, device_code: &str, interval: u64) -> Result<String> {
    let client_id = get_client_id()?;
    let mut interval = Duration::from_secs(interval + 1); // Add a small buffer

    loop {
        tokio::time::sleep(interval).await;

        let res = client.post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", client_id.as_str()),
                ("device_code", device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await?;

        let text = res.text().await?;
        // tracing::debug!("Poll raw response: {}", text);

        let data: PollResponse = serde_json::from_str(&text)?;

        match data {
            PollResponse::Success(token_data) => {
                return Ok(token_data.access_token);
            }
            PollResponse::Error { error, .. } => {
                match error.as_str() {
                    "authorization_pending" => {
                        // Continue polling
                         tracing::debug!("Authorization pending... waiting");
                    }
                    "slow_down" => {
                        interval += Duration::from_secs(5);
                        tracing::warn!("Polling too fast, slowing down");
                    }
                    "expired_token" => {
                        return Err(anyhow!("Device code expired"));
                    }
                    "access_denied" => {
                         return Err(anyhow!("User denied access"));
                    }
                    _ => {
                        return Err(anyhow!("Auth error: {}", error));
                    }
                }
            }
        }
    }
}
