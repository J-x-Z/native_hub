/// Actions sent from the UI to the Backend
#[derive(Debug, Clone)]
pub enum AppAction {
    Login,
    Cancel,
    FetchRepos,
    SelectRepo(String), // Repo name to open/browse
}

#[derive(Debug, Clone)]
pub struct RepoData {
    pub name: String,
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
}
