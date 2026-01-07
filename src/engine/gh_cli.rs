//! GH CLI Engine - Implementation using `gh` command line tool.

use anyhow::{Result, Context, anyhow};
use async_trait::async_trait;
use serde::Deserialize;
use tokio::process::Command;
use crate::app_event::RepoData;
use super::Ops;

/// Engine that wraps the `gh` CLI tool.
pub struct GhCliEngine;

impl GhCliEngine {
    pub fn new() -> Self {
        Self
    }
}

/// Raw JSON structure from `gh repo list --json`
#[derive(Debug, Deserialize)]
struct GhRepoJson {
    name: String,
    #[serde(rename = "nameWithOwner")]
    name_with_owner: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "isPrivate")]
    is_private: bool,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(rename = "stargazerCount", default)]
    stargazer_count: u32,
    #[serde(rename = "forkCount", default)]
    fork_count: u32,
}

#[async_trait]
impl Ops for GhCliEngine {
    async fn fetch_repos(&self) -> Result<Vec<RepoData>> {
        let output = Command::new("gh")
            .args([
                "repo", "list",
                "--json", "name,nameWithOwner,description,isPrivate,updatedAt,stargazerCount,forkCount",
                "--limit", "50"
            ])
            .output()
            .await
            .context("Failed to run 'gh repo list'. Is GitHub CLI installed?")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("gh repo list failed: {}", stderr.trim()));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let raw_repos: Vec<GhRepoJson> = serde_json::from_str(&stdout)
            .context("Failed to parse gh repo list output")?;
        
        // Convert to our RepoData format
        let repos = raw_repos.into_iter().map(|r| {
            RepoData {
                name: r.name,
                full_name: r.name_with_owner,
                description: r.description.unwrap_or_default(),
                is_private: r.is_private,
                last_updated: format_relative_time(&r.updated_at),
                stars_count: r.stargazer_count,
                forks_count: r.fork_count,
            }
        }).collect();
        
        Ok(repos)
    }
}

/// Convert ISO timestamp to relative time (e.g., "2 hours ago")
fn format_relative_time(iso: &str) -> String {
    // Simple implementation - just show the date for now
    // TODO: Use chrono for proper relative time formatting
    if let Some(date_part) = iso.split('T').next() {
        date_part.to_string()
    } else {
        iso.to_string()
    }
}
