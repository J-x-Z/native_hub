//! HTTP API Client for GitHub
//!
//! Uses reqwest to call GitHub REST API directly.
//! This is Android-compatible (no `gh` CLI dependency).

use anyhow::{Context, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

/// A file or directory node in a repository
#[derive(Debug, Clone, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub node_type: String, // "file" or "dir"
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub size: u64,
}

/// HTTP-based GitHub API client
pub struct ApiClient {
    client: reqwest::Client,
    token: String,
}

impl ApiClient {
    /// Create a new API client with the given OAuth token
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token,
        }
    }
    
    /// Fetch the file tree (contents) of a repository at a given path
    /// 
    /// # Arguments
    /// * `owner` - Repository owner (e.g., "octocat")
    /// * `repo` - Repository name (e.g., "Hello-World")
    /// * `path` - Path within the repo (e.g., "" for root, "src" for src folder)
    pub async fn fetch_file_tree(&self, owner: &str, repo: &str, path: &str) -> Result<Vec<FileNode>> {
        let url = if path.is_empty() {
            format!("https://api.github.com/repos/{}/{}/contents", owner, repo)
        } else {
            format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path)
        };
        
        let response = self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to send request to GitHub API")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API returned {}: {}", status, body);
        }
        
        let nodes: Vec<FileNode> = response
            .json()
            .await
            .context("Failed to parse file tree response")?;
        
        Ok(nodes)
    }
    
    /// Fetch raw file content from a download URL
    pub async fn fetch_file_content(&self, download_url: &str) -> Result<String> {
        let response = self.client
            .get(download_url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .send()
            .await
            .context("Failed to fetch file content")?;
        
        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to fetch file: {}", status);
        }
        
        response
            .text()
            .await
            .context("Failed to read file content")
    }
}
