//! Engine Layer - Abstract GitHub Operations
//! 
//! This module provides a unified interface for GitHub operations.
//! The primary implementation uses `gh` CLI, with a future fallback to native HTTP API.

pub mod gh_cli;
pub mod api_client;

use anyhow::Result;
use async_trait::async_trait;
use crate::app_event::RepoData;

/// Core operations trait - all engines must implement this.
#[async_trait]
pub trait Ops: Send + Sync {
    /// Fetch list of repositories for the authenticated user.
    async fn fetch_repos(&self) -> Result<Vec<RepoData>>;
    
    // Future methods:
    // async fn fetch_issues(&self, repo: &str) -> Result<Vec<IssueData>>;
    // async fn fetch_file_tree(&self, repo: &str, path: &str) -> Result<Vec<FileEntry>>;
}

// Re-export default engine
pub use gh_cli::GhCliEngine;
