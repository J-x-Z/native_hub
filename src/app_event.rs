// Re-export types for convenience
pub use crate::engine::api_client::FileNode;
pub use crate::engine::api_client::RepoInfo;
pub use crate::engine::api_client::SearchRepoItem;
pub use crate::engine::api_client::Issue;
pub use crate::engine::api_client::IssueComment;
pub use crate::engine::api_client::IssueLabel;
pub use crate::engine::api_client::PullRequest;
pub use crate::engine::api_client::MergeResult;

/// Actions sent from the UI to the Backend
#[derive(Debug, Clone)]
pub enum AppAction {
    Login,
    Cancel,
    FetchRepos,
    SelectRepo(String),      // Repo name/full_name to browse
    FetchDir(String, String), // (full_name, path) - fetch directory contents
    ReadFile(String),         // (download_url) - fetch file content
    SearchRepos(String),      // Search query
    
    // Issue actions
    FetchIssues(String, String),                    // (full_name, state: "open"/"closed"/"all")
    FetchIssueComments(String, u32),                // (full_name, issue_number)
    CreateComment(String, u32, String),             // (full_name, issue_number, body)
    UpdateIssueState(String, u32, String),          // (full_name, issue_number, state)
    
    // Pull Request actions
    FetchPullRequests(String, String),              // (full_name, state: "open"/"closed"/"all")
    MergePullRequest(String, u32, String),          // (full_name, pr_number, merge_method)
    ClosePullRequest(String, u32),                  // (full_name, pr_number)
}

#[derive(Debug, Clone)]
pub struct RepoData {
    pub name: String,
    pub full_name: String, // owner/repo format for API calls
    pub description: String,
    pub is_private: bool,
    pub last_updated: String,
    pub stars_count: u32,
    pub forks_count: u32,
}

/// Events sent from the Backend to the UI
#[derive(Debug, Clone)]
pub enum AppEvent {
    Log(String),
    DeviceCode(crate::modules::auth::DeviceCodeResponse),
    AuthSuccess(String),
    Error(String),
    RepoList(Vec<RepoData>),
    FileTree(String, Vec<FileNode>), // (current_path, file list)
    FileContent(String, String),      // (filename, content)
    RepoInfoLoaded(RepoInfo),         // Repo metadata (stars, forks, etc.)
    ReadmeLoaded(String),             // README content
    SearchResults(Vec<SearchRepoItem>), // Search results
    
    // Issue events
    IssueList(Vec<Issue>),            // List of issues
    IssueComments(u32, Vec<IssueComment>), // (issue_number, comments)
    CommentCreated(IssueComment),     // New comment created
    IssueUpdated(Issue),              // Issue state updated
    
    // Pull Request events
    PullRequestList(Vec<PullRequest>), // List of PRs
    PullRequestMerged(MergeResult),   // PR merge result
    PullRequestClosed(PullRequest),   // PR closed
}
