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
    
    /// Fetch repository info (description, stars, forks, topics)
    pub async fn fetch_repo_info(&self, owner: &str, repo: &str) -> Result<RepoInfo> {
        let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
        
        let response = self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to fetch repo info")?;
        
        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to fetch repo info: {}", status);
        }
        
        response
            .json()
            .await
            .context("Failed to parse repo info")
    }
    
    /// Search repositories on GitHub
    /// 
    /// # Arguments
    /// * `query` - Search query (e.g., "rust async")
    /// * `sort` - Sort by: "stars", "forks", "help-wanted-issues", "updated" (optional)
    /// * `per_page` - Results per page (max 100)
    pub async fn search_repos(&self, query: &str, sort: Option<&str>, per_page: u32) -> Result<SearchResult> {
        let mut url = format!(
            "https://api.github.com/search/repositories?q={}&per_page={}",
            urlencoding::encode(query),
            per_page.min(100)
        );
        
        if let Some(s) = sort {
            url.push_str(&format!("&sort={}", s));
        }
        
        let response = self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to search repositories")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Search failed {}: {}", status, body);
        }
        
        response
            .json()
            .await
            .context("Failed to parse search results")
    }
    
    // ========================================================================
    // Issues API
    // ========================================================================
    
    /// Fetch issues for a repository
    /// 
    /// # Arguments
    /// * `owner` - Repository owner
    /// * `repo` - Repository name
    /// * `state` - "open", "closed", or "all"
    pub async fn fetch_issues(&self, owner: &str, repo: &str, state: &str) -> Result<Vec<Issue>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues?state={}&per_page=30",
            owner, repo, state
        );
        
        let response = self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to fetch issues")?;
        
        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to fetch issues: {}", status);
        }
        
        response
            .json()
            .await
            .context("Failed to parse issues")
    }
    
    /// Fetch comments for an issue
    pub async fn fetch_issue_comments(&self, owner: &str, repo: &str, issue_number: u32) -> Result<Vec<IssueComment>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}/comments",
            owner, repo, issue_number
        );
        
        let response = self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to fetch comments")?;
        
        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to fetch comments: {}", status);
        }
        
        response
            .json()
            .await
            .context("Failed to parse comments")
    }
    
    /// Create a comment on an issue
    pub async fn create_comment(&self, owner: &str, repo: &str, issue_number: u32, body: &str) -> Result<IssueComment> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}/comments",
            owner, repo, issue_number
        );
        
        let response = self.client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&serde_json::json!({ "body": body }))
            .send()
            .await
            .context("Failed to create comment")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to create comment {}: {}", status, body);
        }
        
        response
            .json()
            .await
            .context("Failed to parse created comment")
    }
    
    /// Close or reopen an issue
    pub async fn update_issue_state(&self, owner: &str, repo: &str, issue_number: u32, state: &str) -> Result<Issue> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}",
            owner, repo, issue_number
        );
        
        let response = self.client
            .patch(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&serde_json::json!({ "state": state }))
            .send()
            .await
            .context("Failed to update issue")?;
        
        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to update issue: {}", status);
        }
        
        response
            .json()
            .await
            .context("Failed to parse updated issue")
    }
    
    // ========================================================================
    // Pull Request API
    // ========================================================================
    
    /// Fetch pull requests for a repository
    pub async fn fetch_pull_requests(&self, owner: &str, repo: &str, state: &str) -> Result<Vec<PullRequest>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls?state={}&per_page=30",
            owner, repo, state
        );
        
        let response = self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to fetch pull requests")?;
        
        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to fetch PRs: {}", status);
        }
        
        response
            .json()
            .await
            .context("Failed to parse pull requests")
    }
    
    /// Merge a pull request
    pub async fn merge_pull_request(&self, owner: &str, repo: &str, pr_number: u32, merge_method: &str) -> Result<MergeResult> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}/merge",
            owner, repo, pr_number
        );
        
        let response = self.client
            .put(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&serde_json::json!({ "merge_method": merge_method }))
            .send()
            .await
            .context("Failed to merge pull request")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to merge PR {}: {}", status, body);
        }
        
        response
            .json()
            .await
            .context("Failed to parse merge result")
    }
    
    /// Close a pull request
    pub async fn close_pull_request(&self, owner: &str, repo: &str, pr_number: u32) -> Result<PullRequest> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}",
            owner, repo, pr_number
        );
        
        let response = self.client
            .patch(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NativeHub-Rust-Client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&serde_json::json!({ "state": "closed" }))
            .send()
            .await
            .context("Failed to close pull request")?;
        
        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to close PR: {}", status);
        }
        
        response
            .json()
            .await
            .context("Failed to parse closed PR")
    }
}

/// Repository information from GitHub API
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RepoInfo {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub stargazers_count: u32,
    #[serde(default)]
    pub forks_count: u32,
    #[serde(default)]
    pub watchers_count: u32,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub license: Option<LicenseInfo>,
    #[serde(default)]
    pub open_issues_count: u32,
    #[serde(default)]
    pub default_branch: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LicenseInfo {
    #[serde(default)]
    pub name: String,
}

/// Search result from GitHub API
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub total_count: u32,
    pub incomplete_results: bool,
    pub items: Vec<SearchRepoItem>,
}

/// A repository item from search results
#[derive(Debug, Clone, Deserialize)]
pub struct SearchRepoItem {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "private")]
    pub is_private: bool,
    #[serde(default)]
    pub stargazers_count: u32,
    #[serde(default)]
    pub forks_count: u32,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    pub html_url: String,
    pub owner: RepoOwner,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepoOwner {
    pub login: String,
    pub avatar_url: String,
}

// ============================================================================
// Issue Types
// ============================================================================

/// An issue from GitHub API
#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u32,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    pub state: String, // "open" or "closed"
    pub user: IssueUser,
    #[serde(default)]
    pub labels: Vec<IssueLabel>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub comments: u32,
    pub html_url: String,
    #[serde(default)]
    pub pull_request: Option<serde_json::Value>, // If present, this is a PR not an issue
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueUser {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueLabel {
    pub name: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// A comment on an issue
#[derive(Debug, Clone, Deserialize)]
pub struct IssueComment {
    pub id: u64,
    pub body: String,
    pub user: IssueUser,
    pub created_at: String,
    pub updated_at: String,
}

// ============================================================================
// Pull Request Types
// ============================================================================

/// A pull request from GitHub API
#[derive(Debug, Clone, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub number: u32,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    pub state: String, // "open", "closed"
    pub user: IssueUser,
    #[serde(default)]
    pub labels: Vec<IssueLabel>,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
    pub head: PullRequestRef,
    pub base: PullRequestRef,
    #[serde(default)]
    pub merged: bool,
    #[serde(default)]
    pub mergeable: Option<bool>,
    #[serde(default)]
    pub mergeable_state: Option<String>,
    #[serde(default)]
    pub comments: u32,
    #[serde(default)]
    pub commits: u32,
    #[serde(default)]
    pub additions: u32,
    #[serde(default)]
    pub deletions: u32,
    #[serde(default)]
    pub changed_files: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestRef {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
}

/// Result of merging a pull request
#[derive(Debug, Clone, Deserialize)]
pub struct MergeResult {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}
