// Re-export FileNode for convenience
pub use crate::engine::api_client::FileNode;

/// Actions sent from the UI to the Backend
#[derive(Debug, Clone)]
pub enum AppAction {
    Login,
    Cancel,
    FetchRepos,
    SelectRepo(String),      // Repo name/full_name to browse
    FetchDir(String, String), // (full_name, path) - fetch directory contents
    ReadFile(String),         // (download_url) - fetch file content
}

#[derive(Debug, Clone)]
pub struct RepoData {
    pub name: String,
    pub full_name: String, // owner/repo format for API calls
    pub description: String,
    pub is_private: bool,
    pub last_updated: String,
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
}
