use tokio::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use crate::app_event::{AppAction, AppEvent};
use crate::context::AppContext;
use crate::modules::auth;
use crate::engine::api_client::ApiClient;

/// Helper function to get GitHub token (tries gh CLI first, then keyring)
fn get_github_token() -> Option<String> {
    // First try gh CLI (always works if installed)
    if let Ok(token) = auth::get_token_from_gh_cli() {
        return Some(token);
    }
    
    // Fallback to keyring
    if let Ok(entry) = keyring::Entry::new("native_hub", "github_oauth") {
        if let Ok(token) = entry.get_password() {
            return Some(token);
        }
    }
    
    None
}

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
                // Fetch root file tree, repo info, and README for the repo
                let tx = event_tx.clone();
                let full_name_clone = full_name.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在浏览仓库: {}...", full_name)));
                    
                    // Get token from gh CLI or keyring
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => {
                            let _ = tx.send(AppEvent::Error("无法获取 Token (请确保已登录 gh CLI)".to_string()));
                            return;
                        }
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 {
                        let _ = tx.send(AppEvent::Error("仓库名格式错误".to_string()));
                        return;
                    }
                    
                    let (owner, repo) = (parts[0], parts[1]);
                    
                    // Fetch file tree
                    match api.fetch_file_tree(owner, repo, "").await {
                        Ok(files) => {
                            let _ = tx.send(AppEvent::Log(format!("找到 {} 个文件/目录", files.len())));
                            
                            // Send FileTree FIRST so UI transitions to Browsing state
                            let _ = tx.send(AppEvent::FileTree("".to_string(), files.clone()));
                            
                            // NOW load README (after state has transitioned)
                            for file in &files {
                                if file.name.to_lowercase().starts_with("readme") {
                                    if let Some(ref url) = file.download_url {
                                        let _ = tx.send(AppEvent::Log("正在加载 README...".to_string()));
                                        if let Ok(readme) = api.fetch_file_content(url).await {
                                            let _ = tx.send(AppEvent::ReadmeLoaded(readme));
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("获取文件列表失败: {}", e)));
                        }
                    }
                    
                    // Fetch repo info
                    if let Ok(info) = api.fetch_repo_info(owner, repo).await {
                        let _ = tx.send(AppEvent::Log(format!("⭐ {} | 🍴 {}", info.stargazers_count, info.forks_count)));
                        let _ = tx.send(AppEvent::RepoInfoLoaded(info));
                    }
                });
            }
            AppAction::FetchDir(full_name, path) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在加载目录: /{}", path)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => return,
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
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => return,
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
            AppAction::SearchRepos(query) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在搜索: {}...", query)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => {
                            let _ = tx.send(AppEvent::Error("无法获取 Token".to_string()));
                            return;
                        }
                    };
                    
                    let api = ApiClient::new(token);
                    
                    match api.search_repos(&query, Some("stars"), 30).await {
                        Ok(result) => {
                            let _ = tx.send(AppEvent::Log(format!("找到 {} 个仓库", result.total_count)));
                            let _ = tx.send(AppEvent::SearchResults(result.items));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("搜索失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::FetchIssues(full_name, state) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在获取 {} 的 Issues...", full_name)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => {
                            let _ = tx.send(AppEvent::Error("无法获取 Token".to_string()));
                            return;
                        }
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 {
                        let _ = tx.send(AppEvent::Error("无效的仓库名".to_string()));
                        return;
                    }
                    
                    match api.fetch_issues(parts[0], parts[1], &state).await {
                        Ok(issues) => {
                            // Filter out PRs (they have pull_request field)
                            let issues: Vec<_> = issues.into_iter()
                                .filter(|i| i.pull_request.is_none())
                                .collect();
                            let _ = tx.send(AppEvent::Log(format!("找到 {} 个 Issues", issues.len())));
                            let _ = tx.send(AppEvent::IssueList(issues));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("获取 Issues 失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::FetchIssueComments(full_name, issue_number) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在获取 Issue #{} 的评论...", issue_number)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => return,
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 { return; }
                    
                    match api.fetch_issue_comments(parts[0], parts[1], issue_number).await {
                        Ok(comments) => {
                            let _ = tx.send(AppEvent::IssueComments(issue_number, comments));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("获取评论失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::CreateComment(full_name, issue_number, body) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在发表评论...")));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => return,
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 { return; }
                    
                    match api.create_comment(parts[0], parts[1], issue_number, &body).await {
                        Ok(comment) => {
                            let _ = tx.send(AppEvent::Log("评论已发表".to_string()));
                            let _ = tx.send(AppEvent::CommentCreated(comment));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("发表评论失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::UpdateIssueState(full_name, issue_number, state) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let action_text = if state == "closed" { "关闭" } else { "重新打开" };
                    let _ = tx.send(AppEvent::Log(format!("正在{} Issue #{}...", action_text, issue_number)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => return,
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 { return; }
                    
                    match api.update_issue_state(parts[0], parts[1], issue_number, &state).await {
                        Ok(issue) => {
                            let _ = tx.send(AppEvent::Log(format!("Issue #{} 已{}", issue_number, action_text)));
                            let _ = tx.send(AppEvent::IssueUpdated(issue));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("操作失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::FetchPullRequests(full_name, state) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在获取 {} 的 Pull Requests...", full_name)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => {
                            let _ = tx.send(AppEvent::Error("无法获取 Token".to_string()));
                            return;
                        }
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 {
                        let _ = tx.send(AppEvent::Error("无效的仓库名".to_string()));
                        return;
                    }
                    
                    match api.fetch_pull_requests(parts[0], parts[1], &state).await {
                        Ok(prs) => {
                            let _ = tx.send(AppEvent::Log(format!("找到 {} 个 Pull Requests", prs.len())));
                            let _ = tx.send(AppEvent::PullRequestList(prs));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("获取 PRs 失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::MergePullRequest(full_name, pr_number, merge_method) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在合并 PR #{}...", pr_number)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => return,
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 { return; }
                    
                    match api.merge_pull_request(parts[0], parts[1], pr_number, &merge_method).await {
                        Ok(result) => {
                            let _ = tx.send(AppEvent::Log(format!("PR #{} 已合并: {}", pr_number, result.message)));
                            let _ = tx.send(AppEvent::PullRequestMerged(result));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("合并失败: {}", e)));
                        }
                    }
                });
            }
            AppAction::ClosePullRequest(full_name, pr_number) => {
                let tx = event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::Log(format!("正在关闭 PR #{}...", pr_number)));
                    
                    let token = match get_github_token() {
                        Some(t) => t,
                        None => return,
                    };
                    
                    let api = ApiClient::new(token);
                    let parts: Vec<&str> = full_name.split('/').collect();
                    if parts.len() != 2 { return; }
                    
                    match api.close_pull_request(parts[0], parts[1], pr_number).await {
                        Ok(pr) => {
                            let _ = tx.send(AppEvent::Log(format!("PR #{} 已关闭", pr_number)));
                            let _ = tx.send(AppEvent::PullRequestClosed(pr));
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(format!("关闭失败: {}", e)));
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
