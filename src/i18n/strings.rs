//! Translation strings for all supported languages

use super::Lang;

/// Get translated string by key and language
pub fn get(lang: Lang, key: &str) -> &'static str {
    match lang {
        Lang::ZhCn => zh_cn(key),
        Lang::En => en(key),
    }
}

/// Chinese (Simplified) translations
fn zh_cn(key: &str) -> &'static str {
    match key {
        // App
        "app.title" => "NativeHub",
        "app.subtitle" => "原生 GitHub 客户端",
        
        // Login Screen
        "login.title" => "连接到 GitHub",
        "login.button" => "登录 GitHub",
        "login.button_icon" => "🔐",
        "login.connecting" => "正在建立连接...",
        "login.error_prefix" => "错误",
        
        // Auth Modal
        "auth.title" => "安全验证",
        "auth.instruction" => "请在浏览器中输入以下验证码:",
        "auth.copy_code" => "复制验证码",
        "auth.open_browser" => "打开浏览器",
        "auth.waiting" => "等待验证中...",
        
        // Repo Browser
        "repos.title" => "仓库列表",
        "repos.refresh" => "刷新",
        "repos.loading" => "正在加载仓库...",
        "repos.empty" => "暂无数据，请点击刷新",
        "repos.private" => "私有",
        "repos.public" => "公开",
        
        // Log Viewer
        "log.title" => "系统日志",
        "log.system_online" => "系统已就绪",
        "log.awaiting" => "等待操作...",
        "log.scanning_gh" => "正在检测 GH CLI...",
        "log.gh_found" => "已找到 GH CLI 令牌!",
        "log.connection_ok" => "安全连接已建立",
        "log.fetching_repos" => "正在获取仓库列表...",
        "log.found_repos" => "找到 {} 个仓库",
        "log.opening_repo" => "正在打开仓库: {}...",
        "log.browser_launched" => "浏览器已启动",
        
        // Settings
        "settings.language" => "语言",
        
        // Common
        "common.cancel" => "取消",
        "common.confirm" => "确认",
        "common.error" => "错误",
        "common.success" => "成功",
        
        // Fallback - return the key itself for debugging (unsafe but works with leaked string)
        _ => "[MISSING]",
    }
}

/// English translations
fn en(key: &str) -> &'static str {
    match key {
        // App
        "app.title" => "NativeHub",
        "app.subtitle" => "Native GitHub Client",
        
        // Login Screen
        "login.title" => "Connect to GitHub",
        "login.button" => "LOGIN WITH GITHUB",
        "login.button_icon" => "🔐",
        "login.connecting" => "ESTABLISHING UPLINK...",
        "login.error_prefix" => "ERROR",
        
        // Auth Modal
        "auth.title" => "SECURITY CHECKPOINT",
        "auth.instruction" => "Enter this code in your browser:",
        "auth.copy_code" => "COPY CODE",
        "auth.open_browser" => "OPEN BROWSER",
        "auth.waiting" => "Waiting for verification...",
        
        // Repo Browser
        "repos.title" => "REPOSITORIES",
        "repos.refresh" => "REFRESH",
        "repos.loading" => "Accessing GitHub Uplink...",
        "repos.empty" => "No Data Stream. Click Refresh.",
        "repos.private" => "Private",
        "repos.public" => "Public",
        
        // Log Viewer
        "log.title" => "SYSTEM LOG",
        "log.system_online" => "SYSTEM LINE ONLINE.",
        "log.awaiting" => "AWAITING INPUT...",
        "log.scanning_gh" => "SCANNING FOR GH CLI...",
        "log.gh_found" => "GH CLI TOKEN FOUND!",
        "log.connection_ok" => "Secure Connection Established.",
        "log.fetching_repos" => "FETCHING REPOS VIA GH CLI...",
        "log.found_repos" => "FOUND {} REPOSITORIES.",
        "log.opening_repo" => "OPENING REPO: {}...",
        "log.browser_launched" => "BROWSER LAUNCHED.",
        
        // Settings
        "settings.language" => "Language",
        
        // Common
        "common.cancel" => "Cancel",
        "common.confirm" => "Confirm",
        "common.error" => "Error",
        "common.success" => "Success",
        
        // Fallback - return the key itself for debugging
        _ => "[MISSING]",
    }
}
