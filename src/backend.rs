use tokio::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use crate::app_event::{AppAction, AppEvent};
use crate::context::AppContext;
use crate::modules::auth;
use crate::engine::api_client::ApiClient;

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
            AppAction::SelectRepo(full_name) => {
                // Fetch root file tree for the repo
                let tx = event_tx.clone();
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在浏览仓库: {}...", full_name)));
                    
                    // Get token from keyring
                    let token = match keyring::Entry::new("native_hub", "github_oauth")
                        .and_then(|e| e.get_password())
                    {
                        Ok(t) => t,
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("无法获取 Token: {}", e)));
                            return;
                        }
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 {
                        let _ = tx.send(AppEvent::Error("仓库名格式错误".to_string()));
                        return;
                    }
                    
                    match api.fetch_file_tree(parts[0], parts[1], "").await {
                        Ok(files) => {
                            let _ = tx.send(AppEvent::Log(format!("找到 {} 个文件/目录", files.len())));
                            let _ = tx.send(AppEvent::FileTree("".to_string(), files));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("获取文件列表失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::FetchDir(full_name, path) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在加载目录: /{}", path)));
                    
                    let token = match keyring::Entry::new("native_hub", "github_oauth")
                        .and_then(|e| e.get_password())
                    {
                        Ok(t) => t,
                        Err(_) => return,
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 { return; }
                    
                    match api.fetch_file_tree(parts[0], parts[1], &path).await {
                        Ok(files) => {
                            let _ = tx.send(AppEvent::FileTree(path, files));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("加载目录失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::ReadFile(download_url) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log("正在读取文件内容...".to_string()));
                    
                    let token = match keyring::Entry::new("native_hub", "github_oauth")
                        .and_then(|e| e.get_password())
                    {
                        Ok(t) => t,
                        Err(_) => return,
                    };
                    
                    let api = ApiClient::new(token);
                    
                    // Extract filename from URL
                    let filename = download_url.split('/').last().unwrap_or("file").to_string();
                    
                    match api.fetch_file_content(&download_url).await {
                        Ok(content) => {
                            let _ = tx.send(AppEvent::Log(format!("文件 {} 已加载", filename)));
                            let _ = tx.send(AppEvent::FileContent(filename, content));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("读取文件失败: {}", e)));
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
