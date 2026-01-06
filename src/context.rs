use keyring::Entry;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application-wide context holding global state
#[derive(Clone)]
pub struct AppContext {
    /// HTTP Client for API requests
    pub http_client: Client,
    /// Current authentication token (if logged in)
    pub auth_token: Arc<RwLock<Option<String>>>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            http_client: Client::builder()
                .user_agent("NativeHub/0.1.0")
                .build()
                .unwrap_or_default(),
            auth_token: Arc::new(RwLock::new(None)),
        }
    }

    /// Try to load token from system keyring on startup
    pub async fn load_token_from_keyring(&self) {
        let entry = Entry::new("native_hub", "github_oauth");
        if let Ok(token) = entry.and_then(|e| e.get_password()) {
            *self.auth_token.write().await = Some(token);
            tracing::info!("Token loaded from keyring");
        }
    }
}
