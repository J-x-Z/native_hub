use tokio::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use crate::app_event::{AppAction, AppEvent};
use crate::context::AppContext;
use crate::modules::auth;

/// The main backend loop running on the tokio runtime
pub async fn run_backend(
    mut action_rx: Receiver<AppAction>,
    event_tx: Sender<AppEvent>,
    ctx: AppContext,
) {
    let _ = event_tx.send(AppEvent::Log("SYSTEM LINE ONLINE.".to_string()));
    let _ = event_tx.send(AppEvent::Log("AWAITING INPUT...".to_string()));

    while let Some(action) = action_rx.recv().await {
        match action {
            AppAction::Login => {
                let tx = event_tx.clone();
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    handle_login(ctx_clone, tx).await;
                });
            }
            AppAction::FetchRepos => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    use crate::engine::{GhCliEngine, Ops};
                    
                    let _ = tx.send(AppEvent::Log("FETCHING REPOS VIA GH CLI...".to_string()));
                    
                    let engine = GhCliEngine::new();
                    match engine.fetch_repos().await {
                        Ok(repos) => {
                            let _ = tx.send(AppEvent::Log(format!("FOUND {} REPOSITORIES.", repos.len())));
                            let _ = tx.send(AppEvent::RepoList(repos));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("FETCH FAILED: {}", e)));
                        }
                    }
                });
            }
            AppAction::SelectRepo(repo_name) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("OPENING REPO: {}...", repo_name)));
                    
                    // Use gh browse to open repo in browser
                    let result = tokio::process::Command::new("gh")
                        .args(["browse", "--repo", &repo_name])
                        .spawn();
                    
                    match result {
                        Ok(_) => {
                            let _ = tx.send(AppEvent::Log("BROWSER LAUNCHED.".to_string()));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("FAILED TO OPEN: {}", e)));
                        }
                    }
                });
            }
            AppAction::Cancel => {
            }
        }
    }
}

async fn handle_login(ctx: AppContext, event_tx: Sender<AppEvent>) {
    // Strategy 1: Try to get token from gh CLI (easiest, no registration needed)
    let _ = event_tx.send(AppEvent::Log("SCANNING FOR GH CLI...".to_string()));
    
    match auth::get_token_from_gh_cli() {
        Ok(token) => {
            let _ = event_tx.send(AppEvent::Log("GH CLI TOKEN FOUND!".to_string()));
            
            // Store in keyring for future sessions
            if let Ok(entry) = keyring::Entry::new("native_hub", "github_oauth") {
                let _ = entry.set_password(&token);
            }
            
            // Update global context
            *ctx.auth_token.write().await = Some(token.clone());
            
            let _ = event_tx.send(AppEvent::AuthSuccess(token));
            return;
        }
        Err(e) => {
            let _ = event_tx.send(AppEvent::Log(format!("GH CLI not available: {}", e)));
            let _ = event_tx.send(AppEvent::Log("FALLING BACK TO OAUTH DEVICE FLOW...".to_string()));
        }
    }
    
    // Strategy 2: OAuth Device Flow (requires GITHUB_CLIENT_ID env var)
    let _ = event_tx.send(AppEvent::Log("EXECUTING PROTOCOL: OAUTH_DEVICE_FLOW".to_string()));

    match auth::request_device_code(&ctx.http_client).await {
        Ok(res) => {
            let _ = event_tx.send(AppEvent::Log("DEVICE CODE RECEIVED.".to_string()));
            let _ = event_tx.send(AppEvent::DeviceCode(res.clone()));
            
            let _ = event_tx.send(AppEvent::Log("POLLING FOR TOKEN...".to_string()));
            
            // Poll for token
            match auth::poll_access_token(&ctx.http_client, &res.device_code, res.interval).await {
                Ok(token) => {
                    let _ = event_tx.send(AppEvent::Log("ACCESS TOKEN ACQUIRED.".to_string()));
                    
                    // Store in keyring
                    if let Ok(entry) = keyring::Entry::new("native_hub", "github_oauth") {
                         let _ = entry.set_password(&token);
                         let _ = event_tx.send(AppEvent::Log("TOKEN ENCRYPTED & STORED.".to_string()));
                    }

                    // Update global context
                    *ctx.auth_token.write().await = Some(token.clone());
                    
                    let _ = event_tx.send(AppEvent::AuthSuccess(token));
                }
                Err(e) => {
                    let _ = event_tx.send(AppEvent::Error(format!("AUTH FAILED: {}", e)));
                    let _ = event_tx.send(AppEvent::Log("ABORTING OAUTH FLOW.".to_string()));
                }
            }
        }
        Err(e) => {
            let _ = event_tx.send(AppEvent::Error(format!("NETWORK ERROR: {}", e)));
        }
    }
}
