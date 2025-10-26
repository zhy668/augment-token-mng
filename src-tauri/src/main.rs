// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod augment_oauth;
mod augment_user_info;
mod bookmarks;
mod http_server;
mod database;
mod storage;
mod http_client;
mod proxy_config;
mod proxy_helper;

use augment_oauth::{create_augment_oauth_state, generate_augment_authorize_url, complete_augment_oauth_flow, check_account_ban_status, batch_check_account_status, extract_token_from_session, get_batch_credit_consumption_with_app_session, AugmentOAuthState, AugmentTokenResponse, AccountStatus, TokenInfo, TokenStatusResult, BatchCreditConsumptionResponse};
use augment_user_info::{get_user_info, get_user_info_with_app_session, CompleteUserInfo, exchange_auth_session_for_app_session};
use bookmarks::{BookmarkManager, Bookmark};
use http_server::HttpServer;
use database::{DatabaseConfig, DatabaseConfigManager, DatabaseManager};
use storage::{DualStorage, LocalFileStorage, PostgreSQLStorage, TokenStorage, SyncManager};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::SystemTime;
use tauri::{State, Manager, Emitter, WebviewWindowBuilder, WebviewUrl};
use chrono;
use serde::{Serialize, Deserialize};

// Update check structures
#[derive(Debug, Serialize, Deserialize)]
struct UpdateInfo {
    current_version: String,
    latest_version: String,
    has_update: bool,
    download_url: String,
    release_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
}

// App Session 缓存结构 (公开以便其他模块使用)
#[derive(Clone)]
pub struct AppSessionCache {
    pub app_session: String,
    pub created_at: SystemTime,
}

// 一键获取模式配置
#[derive(Clone)]
struct QuickGetMode {
    email: String,
    password: String,
    register_only: bool,
}

// Global state to store OAuth state and storage managers
struct AppState {
    augment_oauth_state: Mutex<Option<AugmentOAuthState>>,
    http_server: Mutex<Option<HttpServer>>,
    storage_manager: Arc<Mutex<Option<Arc<DualStorage>>>>,
    database_manager: Arc<Mutex<Option<Arc<DatabaseManager>>>>,
    // App session 缓存: key 为 auth_session, value 为缓存的 app_session
    app_session_cache: Arc<Mutex<HashMap<String, AppSessionCache>>>,
    // 邮箱助手相关状态
    monitoring_email: Mutex<Option<String>>,
    verification_code: Mutex<Option<String>>,
    quick_get_mode: Mutex<Option<QuickGetMode>>,
    current_login_window: Mutex<Option<String>>, // 当前登录窗口标签
    card_bin: Mutex<String>, // 信用卡BIN码
    card_address: Mutex<CardAddress>, // 信用卡地址信息
}

// 信用卡地址信息结构
#[derive(Debug, Clone, Default)]
struct CardAddress {
    country: String,
    province: String,
    city: String,
    street: String,
    postal_code: String,
}

#[tauri::command]
async fn generate_auth_url(state: State<'_, AppState>) -> Result<String, String> {
    let augment_oauth_state = create_augment_oauth_state();
    let auth_url = generate_augment_authorize_url(&augment_oauth_state)
        .map_err(|e| format!("Failed to generate auth URL: {}", e))?;
    
    // Store the Augment OAuth state
    *state.augment_oauth_state.lock().unwrap() = Some(augment_oauth_state);
    
    Ok(auth_url)
}

#[tauri::command]
async fn generate_augment_auth_url(state: State<'_, AppState>) -> Result<String, String> {
    let augment_oauth_state = create_augment_oauth_state();
    let auth_url = generate_augment_authorize_url(&augment_oauth_state)
        .map_err(|e| format!("Failed to generate Augment auth URL: {}", e))?;
    
    // Store the Augment OAuth state
    *state.augment_oauth_state.lock().unwrap() = Some(augment_oauth_state);
    
    Ok(auth_url)
}



#[tauri::command]
async fn get_token(code: String, state: State<'_, AppState>) -> Result<AugmentTokenResponse, String> {
    let augment_oauth_state = {
        let guard = state.augment_oauth_state.lock().unwrap();
        guard.clone()
            .ok_or("No Augment OAuth state found. Please generate auth URL first.")?
    };

    complete_augment_oauth_flow(&augment_oauth_state, &code)
        .await
        .map_err(|e| format!("Failed to complete OAuth flow: {}", e))
}

#[tauri::command]
async fn get_augment_token(code: String, state: State<'_, AppState>) -> Result<AugmentTokenResponse, String> {
    let augment_oauth_state = {
        let guard = state.augment_oauth_state.lock().unwrap();
        guard.clone()
            .ok_or("No Augment OAuth state found. Please generate auth URL first.")?
    };

    complete_augment_oauth_flow(&augment_oauth_state, &code)
        .await
        .map_err(|e| format!("Failed to complete Augment OAuth flow: {}", e))
}

#[tauri::command]
async fn check_account_status(token: String, tenant_url: String) -> Result<AccountStatus, String> {
    check_account_ban_status(&token, &tenant_url)
        .await
        .map_err(|e| format!("Failed to check account status: {}", e))
}

#[tauri::command]
async fn batch_check_tokens_status(
    tokens: Vec<TokenInfo>,
    _state: State<'_, AppState>,
) -> Result<Vec<TokenStatusResult>, String> {
    batch_check_account_status(tokens)
        .await
        .map_err(|e| format!("Failed to batch check tokens status: {}", e))
}

/// 批量获取 Credit 消费数据(stats 和 chart),使用缓存的 app_session
#[tauri::command]
async fn fetch_batch_credit_consumption(
    auth_session: String,
    state: State<'_, AppState>,
) -> Result<BatchCreditConsumptionResponse, String> {
    // 1. 检查缓存中是否有有效的 app_session
    let cached_app_session = {
        let cache = state.app_session_cache.lock().unwrap();
        cache.get(&auth_session).map(|c| c.app_session.clone())
    };

    // 2. 如果有缓存，先尝试使用缓存的 app_session
    if let Some(app_session) = cached_app_session {
        println!("Using cached app_session for credit consumption");

        // 尝试使用缓存的 app_session 获取数据
        match get_batch_credit_consumption_with_app_session(&app_session).await {
            Ok(result) => {
                println!("Successfully fetched credit data with cached app_session");
                return Ok(result);
            }
            Err(e) => {
                // 如果失败（可能是 session 过期），记录日志并继续获取新的
                println!("Cached app_session failed: {}, will refresh", e);
            }
        }
    }

    // 3. 没有缓存或缓存失效，获取新的 app_session
    println!("Exchanging auth_session for new app_session...");
    let app_session = exchange_auth_session_for_app_session(&auth_session).await?;
    println!("New app session obtained: {}", &app_session[..20.min(app_session.len())]);

    // 4. 更新缓存
    {
        let mut cache = state.app_session_cache.lock().unwrap();
        cache.insert(
            auth_session.clone(),
            AppSessionCache {
                app_session: app_session.clone(),
                created_at: SystemTime::now(),
            },
        );
        println!("App session cached for future use");
    }

    // 5. 使用新的 app_session 获取数据
    get_batch_credit_consumption_with_app_session(&app_session).await
}

// Version comparison helper
fn compare_versions(current: &str, latest: &str) -> bool {
    let parse_version = |v: &str| -> Vec<u32> {
        v.trim_start_matches('v')
            .split('.')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    };

    let current_parts = parse_version(current);
    let latest_parts = parse_version(latest);

    for i in 0..std::cmp::max(current_parts.len(), latest_parts.len()) {
        let current_part = current_parts.get(i).unwrap_or(&0);
        let latest_part = latest_parts.get(i).unwrap_or(&0);

        if latest_part > current_part {
            return true;
        } else if latest_part < current_part {
            return false;
        }
    }

    false
}

#[tauri::command]
async fn check_for_updates() -> Result<UpdateInfo, String> {
    let current_version = env!("CARGO_PKG_VERSION");

    // 使用 ProxyClient，自动处理 Edge Function
    let client = http_client::create_proxy_client()?;
    let response = client
        .get("https://api.github.com/repos/zhaochengcube/augment-token-mng/releases/latest")
        .header("User-Agent", "Mozilla/5.0 (compatible; ATM-App/1.0)")
        .send()
        .await
        .map_err(|e| format!("Failed to check for updates: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }

    let release: GitHubRelease = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub response: {}", e))?;

    let latest_version = release.tag_name.trim_start_matches('v');
    let has_update = compare_versions(current_version, latest_version);

    Ok(UpdateInfo {
        current_version: current_version.to_string(),
        latest_version: latest_version.to_string(),
        has_update,
        download_url: release.html_url,
        release_notes: release.body,
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenFromSessionResponse {
    access_token: String,
    tenant_url: String,
    user_info: CompleteUserInfo,
}

// 内部函数,不发送进度事件,使用缓存的 app_session
async fn add_token_from_session_internal_with_cache(
    session: &str,
    state: &AppState,
) -> Result<TokenFromSessionResponse, String> {
    // 1. 从 session 提取 token
    let token_response = extract_token_from_session(session).await?;

    // 2. 检查缓存中是否有有效的 app_session
    let cached_app_session = {
        let cache = state.app_session_cache.lock().unwrap();
        cache.get(session).map(|c| c.app_session.clone())
    };

    // 3. 尝试使用缓存的 app_session 获取用户信息
    let user_info = if let Some(app_session) = cached_app_session {
        println!("Using cached app_session for user info");
        match get_user_info_with_app_session(&app_session).await {
            Ok(info) => {
                println!("Successfully fetched user info with cached app_session");
                info
            }
            Err(e) => {
                println!("Cached app_session failed: {}, will refresh", e);
                // 缓存失效，获取新的
                let app_session = exchange_auth_session_for_app_session(session).await?;
                // 更新缓存
                {
                    let mut cache = state.app_session_cache.lock().unwrap();
                    cache.insert(
                        session.to_string(),
                        AppSessionCache {
                            app_session: app_session.clone(),
                            created_at: SystemTime::now(),
                        },
                    );
                }
                get_user_info_with_app_session(&app_session).await?
            }
        }
    } else {
        // 没有缓存，获取新的 app_session
        println!("No cached app_session, exchanging new one");
        let app_session = exchange_auth_session_for_app_session(session).await?;
        // 更新缓存
        {
            let mut cache = state.app_session_cache.lock().unwrap();
            cache.insert(
                session.to_string(),
                AppSessionCache {
                    app_session: app_session.clone(),
                    created_at: SystemTime::now(),
                },
            );
        }
        get_user_info_with_app_session(&app_session).await?
    };

    Ok(TokenFromSessionResponse {
        access_token: token_response.access_token,
        tenant_url: token_response.tenant_url,
        user_info,
    })
}

// 内部函数,不发送进度事件（保留用于向后兼容）
async fn add_token_from_session_internal(session: &str) -> Result<TokenFromSessionResponse, String> {
    // 1. 从 session 提取 token
    let token_response = extract_token_from_session(session).await?;

    // 2. 获取用户信息
    let user_info = get_user_info(session).await?;

    Ok(TokenFromSessionResponse {
        access_token: token_response.access_token,
        tenant_url: token_response.tenant_url,
        user_info,
    })
}

#[tauri::command]
async fn add_token_from_session(
    session: String,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<TokenFromSessionResponse, String> {
    // 1. 从 session 提取 token
    let _ = app.emit("session-import-progress", "sessionImportExtractingToken");
    let token_response = extract_token_from_session(&session).await?;

    // 2. 检查缓存中是否有有效的 app_session
    let _ = app.emit("session-import-progress", "sessionImportGettingUserInfo");

    let cached_app_session = {
        let cache = state.app_session_cache.lock().unwrap();
        cache.get(&session).map(|c| c.app_session.clone())
    };

    // 3. 尝试使用缓存的 app_session 获取用户信息
    let user_info = if let Some(app_session) = cached_app_session {
        println!("Using cached app_session for user info");
        match get_user_info_with_app_session(&app_session).await {
            Ok(info) => {
                println!("Successfully fetched user info with cached app_session");
                info
            }
            Err(e) => {
                println!("Cached app_session failed: {}, will refresh", e);
                // 缓存失效，获取新的
                let app_session = exchange_auth_session_for_app_session(&session).await?;
                // 更新缓存
                {
                    let mut cache = state.app_session_cache.lock().unwrap();
                    cache.insert(
                        session.clone(),
                        AppSessionCache {
                            app_session: app_session.clone(),
                            created_at: SystemTime::now(),
                        },
                    );
                }
                get_user_info_with_app_session(&app_session).await?
            }
        }
    } else {
        // 没有缓存，获取新的 app_session
        println!("No cached app_session, exchanging new one");
        let app_session = exchange_auth_session_for_app_session(&session).await?;
        // 更新缓存
        {
            let mut cache = state.app_session_cache.lock().unwrap();
            cache.insert(
                session.clone(),
                AppSessionCache {
                    app_session: app_session.clone(),
                    created_at: SystemTime::now(),
                },
            );
        }
        get_user_info_with_app_session(&app_session).await?
    };

    let _ = app.emit("session-import-progress", "sessionImportComplete");

    Ok(TokenFromSessionResponse {
        access_token: token_response.access_token,
        tenant_url: token_response.tenant_url,
        user_info,
    })
}

#[tauri::command]
async fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))
}

#[tauri::command]
async fn save_tokens_json(json_string: String, app: tauri::AppHandle) -> Result<(), String> {
    use std::fs;
    use std::io::Write;

    // 获取应用数据目录
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // 确保目录存在
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    let storage_path = app_data_dir.join("tokens.json");
    let temp_path = storage_path.with_extension("tmp");

    // 基本的 JSON 格式验证
    serde_json::from_str::<serde_json::Value>(&json_string)
        .map_err(|e| format!("Invalid JSON format: {}", e))?;

    // 原子性写入：先写临时文件，再重命名
    {
        let mut temp_file = fs::File::create(&temp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        temp_file.write_all(json_string.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;

        temp_file.sync_all()
            .map_err(|e| format!("Failed to sync temp file: {}", e))?;
    }

    // 原子性重命名
    fs::rename(&temp_path, &storage_path)
        .map_err(|e| format!("Failed to rename temp file: {}", e))?;

    Ok(())
}


#[tauri::command]
async fn load_tokens_json(app: tauri::AppHandle) -> Result<String, String> {
    use std::fs;

    // 获取新的应用数据目录
    let new_app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let new_storage_path = new_app_data_dir.join("tokens.json");

    println!("尝试读取新文件路径: {:?}", new_storage_path);

    // 首先尝试从新目录读取
    if new_storage_path.exists() {
        let content = fs::read_to_string(&new_storage_path)
            .map_err(|e| format!("Failed to read tokens file: {}", e))?;

        println!("从新目录读取到的文件内容: {}", content);

        // 如果文件为空，返回空数组的 JSON
        if content.trim().is_empty() {
            return Ok("[]".to_string());
        }

        return process_token_content(content);
    }

    // 如果新目录没有文件，尝试从旧目录读取
    println!("新目录中没有文件，尝试从旧目录读取...");

    // 构造旧的应用数据目录路径
    let old_app_data_dir = get_old_app_data_dir()?;
    let old_storage_path = old_app_data_dir.join("tokens.json");

    println!("尝试读取旧文件路径: {:?}", old_storage_path);

    if old_storage_path.exists() {
        let content = fs::read_to_string(&old_storage_path)
            .map_err(|e| format!("Failed to read old tokens file: {}", e))?;

        println!("从旧目录读取到的文件内容: {}", content);

        // 如果文件为空，返回空数组的 JSON
        if content.trim().is_empty() {
            return Ok("[]".to_string());
        }

        // 创建新目录（如果不存在）
        if let Some(parent) = new_storage_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create new app data directory: {}", e))?;
        }

        // 将文件迁移到新目录
        fs::copy(&old_storage_path, &new_storage_path)
            .map_err(|e| format!("Failed to migrate tokens file: {}", e))?;

        println!("文件已迁移到新目录: {:?}", new_storage_path);

        return process_token_content(content);
    }

    // 两个目录都没有文件
    println!("新旧目录都没有找到 tokens.json 文件");
    Ok("[]".to_string())
}

// 获取旧的应用数据目录
fn get_old_app_data_dir() -> Result<PathBuf, String> {
    use std::env;
    use std::path::PathBuf;

    let home_dir = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map_err(|_| "Failed to get home directory".to_string())?;

    // 旧的 identifier: com.capslockCube.augment-token-manager
    let old_path = if cfg!(target_os = "windows") {
        // Windows: %APPDATA%\com.capslockCube.augment-token-manager
        PathBuf::from(home_dir)
            .join("AppData")
            .join("Roaming")
            .join("com.capslockCube.augment-token-manager")
    } else if cfg!(target_os = "macos") {
        // macOS: ~/Library/Application Support/com.capslockCube.augment-token-manager
        PathBuf::from(home_dir)
            .join("Library")
            .join("Application Support")
            .join("com.capslockCube.augment-token-manager")
    } else {
        // Linux: ~/.config/com.capslockCube.augment-token-manager
        PathBuf::from(home_dir)
            .join(".config")
            .join("com.capslockCube.augment-token-manager")
    };

    Ok(old_path)
}

// 处理 token 内容的通用函数
fn process_token_content(content: String) -> Result<String, String> {
    // 尝试解析 JSON 内容
    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(value) => {
            // 如果解析成功，检查是否需要转换格式
            match value {
                serde_json::Value::Array(_) => {
                    // 如果已经是数组格式，直接返回原内容
                    Ok(content)
                }
                serde_json::Value::Object(ref obj) => {
                    // 检查是否是旧格式 {tokens: [...]}
                    if let Some(tokens_array) = obj.get("tokens") {
                        if tokens_array.is_array() {
                            // 旧格式，提取 tokens 数组
                            Ok(serde_json::to_string_pretty(tokens_array)
                                .map_err(|e| format!("Failed to serialize tokens: {}", e))?)
                        } else {
                            Ok("[]".to_string())
                        }
                    } else {
                        // 如果是单个对象格式，包装成数组
                        let array = serde_json::Value::Array(vec![value]);
                        Ok(serde_json::to_string_pretty(&array)
                            .map_err(|e| format!("Failed to serialize tokens: {}", e))?)
                    }
                }
                _ => {
                    // 其他格式，返回空数组
                    Ok("[]".to_string())
                }
            }
        }
        Err(_) => {
            // 如果 JSON 解析失败，可能是其他格式的旧数据，返回空数组
            Ok("[]".to_string())
        }
    }
}



// Bookmark management commands
#[tauri::command]
async fn add_bookmark(
    name: String,
    url: String,
    description: Option<String>,
    category: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.add_bookmark(name, url, description, category)
        .map_err(|e| format!("Failed to add bookmark: {}", e))
}

#[tauri::command]
async fn update_bookmark(
    id: String,
    name: String,
    url: String,
    description: Option<String>,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.update_bookmark(&id, name, url, description)
        .map_err(|e| format!("Failed to update bookmark: {}", e))
}

#[tauri::command]
async fn delete_bookmark(
    id: String,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.remove_bookmark(&id)
        .map_err(|e| format!("Failed to delete bookmark: {}", e))
}

#[tauri::command]
async fn get_bookmarks(
    category: String,
    app: tauri::AppHandle,
) -> Result<Vec<Bookmark>, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.get_bookmarks_by_category(&category)
        .map_err(|e| format!("Failed to get bookmarks: {}", e))
}

#[tauri::command]
async fn get_all_bookmarks(
    app: tauri::AppHandle,
) -> Result<Vec<Bookmark>, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.get_all_bookmarks()
        .map_err(|e| format!("Failed to get all bookmarks: {}", e))
}







#[tauri::command]
async fn open_internal_browser(
    app: tauri::AppHandle,
    url: String,
    title: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    use tauri::webview::PageLoadEvent;
    use std::time::Duration;

    let window_label = format!("browser_{}", chrono::Utc::now().timestamp());
    let app_handle = app.clone();

    // 获取监控邮箱
    let monitoring_email = {
        let email_guard = state.monitoring_email.lock().unwrap();
        email_guard.clone()
    };

    eprintln!("[InternalBrowser] Opening browser, monitoring email: {:?}", monitoring_email);

    let window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
    )
    .title(&title.unwrap_or_else(|| "内置浏览器".to_string()))
    .inner_size(1000.0, 700.0)
    .center()
    .resizable(true)
    .incognito(true)  // 无痕模式,关闭后自动清除所有数据
    .initialization_script(&format!(r#"
        console.log('[Tauri] Initialization script loaded');

        // 自动填充邮箱 - 使用更激进的策略
        const monitoringEmail = '{}';

        function tryAutoFillEmail() {{
            const emailInput = document.querySelector('input[type="email"]') ||
                              document.querySelector('input[autocomplete="email"]') ||
                              document.querySelector('input[name="email"]');

            if (emailInput && monitoringEmail && !emailInput.value) {{
                emailInput.value = monitoringEmail;
                emailInput.dispatchEvent(new Event('input', {{ bubbles: true }}));
                emailInput.dispatchEvent(new Event('change', {{ bubbles: true }}));
                emailInput.dispatchEvent(new Event('blur', {{ bubbles: true }}));
                console.log('[Tauri] Auto-filled email:', monitoringEmail);
                return true;
            }}
            return false;
        }}

        // 监听 DOM 变化,一旦发现邮箱输入框就填充
        const emailObserver = new MutationObserver(() => {{
            if (tryAutoFillEmail()) {{
                console.log('[Tauri] Email filled via MutationObserver');
            }}
        }});

        // 开始观察
        if (document.body) {{
            emailObserver.observe(document.body, {{ childList: true, subtree: true }});
        }} else {{
            document.addEventListener('DOMContentLoaded', () => {{
                emailObserver.observe(document.body, {{ childList: true, subtree: true }});
            }});
        }}

        // 同时使用定时器作为备份
        let fillAttempts = 0;
        const fillInterval = setInterval(() => {{
            if (tryAutoFillEmail() || fillAttempts++ > 20) {{
                clearInterval(fillInterval);
                if (fillAttempts > 20) {{
                    emailObserver.disconnect();
                }}
            }}
        }}, 500);

        // 全局变量 - 用于控制继续按钮检查
        window.continueCheckInterval = null;
        window.continueButtonClicked = false;

        // 自动点击 continue 按钮
        function tryAutoClickContinue() {{
            const continueBtn = document.querySelector('button[type="submit"]') ||
                               Array.from(document.querySelectorAll('button')).find(btn =>
                                 btn.textContent.toLowerCase().includes('continue') ||
                                 btn.textContent.includes('继续')
                               );

            if (continueBtn && !continueBtn.disabled && !continueBtn.hasAttribute('disabled')) {{
                console.log('[AutoClick] Clicking continue button');
                continueBtn.click();
                return true;
            }}

            return false;
        }}

        // 定时检查 continue 按钮(仅在邮箱输入页面,邮箱填充后开始,等待6秒)
        setTimeout(() => {{
            // 只在登录/注册页面检查继续按钮
            const currentUrl = window.location.href;
            if (!currentUrl.includes('/login') && !currentUrl.includes('/signup')) {{
                console.log('[AutoClick] Not on login/signup page, skipping continue button checks');
                return;
            }}

            let checkCount = 0;
            const maxChecks = 5; // 最多检查5次
            window.continueCheckInterval = setInterval(() => {{
                if (window.continueButtonClicked) {{
                    clearInterval(window.continueCheckInterval);
                    window.continueCheckInterval = null;
                    console.log('[AutoClick] Continue button already clicked, stopping checks');
                    return;
                }}

                checkCount++;
                if (checkCount > maxChecks) {{
                    clearInterval(window.continueCheckInterval);
                    window.continueCheckInterval = null;
                    console.log('[AutoClick] Max checks reached (5 attempts)');
                    return;
                }}

                console.log(`[AutoClick] Checking continue button (attempt ${{checkCount}}/${{maxChecks}})`);
                if (tryAutoClickContinue()) {{
                    window.continueButtonClicked = true;
                    clearInterval(window.continueCheckInterval);
                    window.continueCheckInterval = null;
                    console.log('[AutoClick] Continue button clicked successfully, stopped checking');
                }}
            }}, 3000);
        }}, 6000);

        // 创建导航栏的函数
        function createNavbar() {{
            console.log('[Tauri] Creating navbar...');

            // 只在 augmentcode.com 域名下显示
            if (!window.location.hostname.includes('augmentcode.com')) {{
                console.log('[Tauri] Not on augmentcode.com, skipping navbar');
                return;
            }}

            // 检查是否已存在
            if (document.getElementById('tauri-navbar')) {{
                console.log('[Tauri] Navbar already exists');
                return;
            }}

            const navbar = document.createElement('div');
            navbar.id = 'tauri-navbar';
            navbar.style.cssText = 'position: fixed; top: 50%; right: 20px; transform: translateY(-50%); z-index: 2147483647; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;';

            const button = document.createElement('button');
            button.id = 'tauri-import-button';

            // 检查当前页面状态
            const isLoginPage = window.location.hostname.includes('login.augmentcode.com') ||
                                window.location.href.includes('/login');

            // 根据状态设置按钮
            if (isLoginPage) {{
                // 在登录页面,提示登录后会自动导入
                button.textContent = '🔒 登录后自动导入';
                button.disabled = true;
                button.style.cssText = 'background: #fef3c7; color: #92400e; border: 1px solid #fbbf24; padding: 12px 24px; border-radius: 8px; cursor: not-allowed; font-size: 14px; font-weight: 500; opacity: 0.9; box-shadow: 0 4px 12px rgba(0,0,0,0.15); white-space: nowrap;';
            }} else {{
                // 其他页面(主页/auth页面),显示正在导入
                button.textContent = '⏳ 正在导入...';
                button.disabled = true;
                button.style.cssText = 'background: #f3f4f6; color: #6b7280; border: 1px solid #d1d5db; padding: 12px 24px; border-radius: 8px; cursor: not-allowed; font-size: 14px; font-weight: 500; opacity: 0.7; box-shadow: 0 4px 12px rgba(0,0,0,0.15); white-space: nowrap;';
            }}

            // 按钮仅用于显示状态,不需要交互事件

            navbar.appendChild(button);

            // 插入到页面
            if (document.body) {{
                document.body.appendChild(navbar);
                console.log('[Tauri] Navbar inserted at right middle');
            }} else if (document.documentElement) {{
                document.documentElement.appendChild(navbar);
                console.log('[Tauri] Navbar inserted to documentElement');
            }}
        }}

        // 多种方式尝试插入导航栏
        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', createNavbar);
        }} else {{
            createNavbar();
        }}

        // 监听页面变化,确保导航栏始终存在
        setInterval(function() {{
            if (!document.getElementById('tauri-navbar')) {{
                createNavbar();
            }}
        }}, 1000);
    "#, monitoring_email.unwrap_or_default()))
    .on_page_load(move |window, payload| {
        if payload.event() == PageLoadEvent::Finished {
            let url_str = payload.url().to_string();

            // 检查是否是 auth.augmentcode.com
            if url_str.contains("auth.augmentcode.com") {
                let window_clone = window.clone();
                let app_handle_clone = app_handle.clone();

                // 在后台线程中获取 Cookie (使用 tauri 的 async runtime)
                tauri::async_runtime::spawn(async move {
                    // 等待一小段时间确保 Cookie 已设置
                    tokio::time::sleep(Duration::from_millis(1000)).await;

                    match window_clone.cookies_for_url(
                        "https://auth.augmentcode.com".parse().unwrap()
                    ) {
                        Ok(cookies) => {
                            // 查找 session Cookie
                            if let Some(session_cookie) = cookies.iter()
                                .find(|c| c.name() == "session")
                            {
                                let session_value = session_cookie.value().to_string();
                                eprintln!("Found session cookie, attempting to import token...");

                                // 获取 AppState 并调用带缓存的内部函数
                                let state = app_handle_clone.state::<AppState>();
                                match add_token_from_session_internal_with_cache(&session_value, &state).await {
                                    Ok(token_data) => {
                                        eprintln!("Successfully imported token from session");

                                        // 发送成功事件到前端，包含 session
                                        let _ = app_handle_clone.emit(
                                            "session-auto-imported",
                                            serde_json::json!({
                                                "success": true,
                                                "token": token_data,
                                                "session": session_value
                                            })
                                        );

                                        // 延迟关闭浏览器窗口,让用户看到成功提示
                                        tokio::time::sleep(Duration::from_millis(500)).await;
                                        let _ = window_clone.close();

                                        // 聚焦主窗口
                                        if let Some(main_window) = app_handle_clone.get_webview_window("main") {
                                            let _ = main_window.set_focus();
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to import token: {}", e);
                                        // 发送失败事件
                                        let _ = app_handle_clone.emit(
                                            "session-auto-import-failed",
                                            serde_json::json!({
                                                "success": false,
                                                "error": e.to_string()
                                            })
                                        );
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to get cookies: {}", e);
                        }
                    }
                });
            }
        }
    })
    .build()
    .map_err(|e| format!("Failed to create browser window: {}", e))?;

    Ok(window_label)
}

#[tauri::command]
async fn close_window(app: tauri::AppHandle, window_label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&window_label) {
        window.close().map_err(|e| format!("Failed to close window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn get_customer_info(token: String) -> Result<String, String> {
    let url = format!("https://portal.withorb.com/api/v1/customer_from_link?token={}", token);

    // 使用 ProxyClient，自动处理 Edge Function
    let client = http_client::create_proxy_client()?;
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Charset", "utf-8")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        let response_text = String::from_utf8_lossy(&bytes).to_string();

        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                match serde_json::to_string_pretty(&json_value) {
                    Ok(formatted) => Ok(formatted),
                    Err(_) => Ok(response_text),
                }
            }
            Err(_) => Ok(response_text),
        }
    } else {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Err(format!("API request failed with status {}: {}", status, response_text))
    }
}

#[tauri::command]
async fn get_ledger_summary(customer_id: String, pricing_unit_id: String, token: String) -> Result<String, String> {
    let url = format!("https://portal.withorb.com/api/v1/customers/{}/ledger_summary?pricing_unit_id={}&token={}",
                     customer_id, pricing_unit_id, token);

    // 使用 ProxyClient，自动处理 Edge Function
    let client = http_client::create_proxy_client()?;
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Charset", "utf-8")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        let response_text = String::from_utf8_lossy(&bytes).to_string();

        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                match serde_json::to_string_pretty(&json_value) {
                    Ok(formatted) => Ok(formatted),
                    Err(_) => Ok(response_text),
                }
            }
            Err(_) => Ok(response_text),
        }
    } else {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Err(format!("API request failed with status {}: {}", status, response_text))
    }
}



#[tauri::command]
async fn test_api_call() -> Result<String, String> {
    let url = "https://portal.withorb.com/api/v1/customer_from_link?token=ImRhUHFhU3ZtelpKdEJrUVci.1konHDs_4UqVUJWcxaZpKV4nQik";

    // 使用 ProxyClient，自动处理 Edge Function
    let client = http_client::create_proxy_client()?;
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Charset", "utf-8")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        // 尝试获取JSON并格式化
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        // 确保使用UTF-8解码
        let response_text = String::from_utf8_lossy(&bytes).to_string();

        // 尝试解析并格式化JSON
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                // 格式化JSON输出
                match serde_json::to_string_pretty(&json_value) {
                    Ok(formatted) => Ok(formatted),
                    Err(_) => Ok(response_text), // 如果格式化失败，返回原始文本
                }
            }
            Err(_) => Ok(response_text), // 如果不是有效JSON，返回原始文本
        }
    } else {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Err(format!("API request failed with status {}: {}", status, response_text))
    }
}

#[tauri::command]
async fn open_data_folder(
    app: tauri::AppHandle,
) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    // Open folder using system default file manager
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
async fn create_jetbrains_token_file(
    editor_type: String,
    token_data: String,
) -> Result<String, String> {
    use std::fs;
    use std::env;
    use std::path::PathBuf;

    // 获取用户主目录
    let home_dir = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map_err(|_| "Failed to get home directory".to_string())?;

    let augment_dir = PathBuf::from(&home_dir).join(".augment");

    // 确保 .augment 目录存在
    fs::create_dir_all(&augment_dir)
        .map_err(|e| format!("Failed to create .augment directory: {}", e))?;

    // 创建文件路径
    let file_name = format!("{}_token.json", editor_type);
    let file_path = augment_dir.join(&file_name);

    // 写入文件
    fs::write(&file_path, token_data)
        .map_err(|e| format!("Failed to write token file: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn open_editor_with_protocol(
    app: tauri::AppHandle,
    protocol_url: String,
) -> Result<(), String> {
    println!("Opening editor with protocol URL: {}", protocol_url);

    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(protocol_url, None::<&str>)
        .map_err(|e| format!("Failed to open editor with protocol: {}", e))
}

// 邮箱助手相关命令
#[tauri::command]
async fn set_monitoring_email(email: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut monitoring_email = state.monitoring_email.lock().unwrap();
    *monitoring_email = Some(email.clone());
    eprintln!("[EmailHelper] Set monitoring email: {}", email);
    Ok(())
}

#[tauri::command]
async fn set_verification_code(app: tauri::AppHandle, code: String, state: State<'_, AppState>) -> Result<(), String> {
    use tauri::Manager;

    let mut verification_code = state.verification_code.lock().unwrap();
    *verification_code = Some(code.clone());
    eprintln!("[EmailHelper] Set verification code: {}", code);

    // 查找登录窗口并直接注入验证码
    let windows = app.webview_windows();
    for (label, window) in windows.iter() {
        if label.starts_with("login_") {
            eprintln!("[EmailHelper] Found login window: {}", label);

            // 构造 JavaScript 代码来填充验证码
            let js_code = format!(r#"
                (function() {{
                    const code = '{}';
                    console.log('[AutoFill] Injected code:', code);

                    const codeInput = document.querySelector('input[type="text"][placeholder*="code" i]') ||
                                     document.querySelector('input[type="text"][placeholder*="验证码" i]') ||
                                     document.querySelector('input[name*="code" i]') ||
                                     document.querySelector('input[name*="otp" i]') ||
                                     document.querySelector('input[autocomplete="one-time-code"]');

                    if (codeInput) {{
                        codeInput.value = code;
                        codeInput.dispatchEvent(new Event('input', {{ bubbles: true }}));
                        codeInput.dispatchEvent(new Event('change', {{ bubbles: true }}));
                        console.log('[AutoFill] Auto-filled verification code:', code);

                        // 停止继续按钮检查
                        console.log('[AutoFill] Attempting to stop continue button checking...');
                        console.log('[AutoFill] continueCheckInterval exists:', !!window.continueCheckInterval);
                        if (window.continueCheckInterval) {{
                            clearInterval(window.continueCheckInterval);
                            window.continueCheckInterval = null;
                            window.continueButtonClicked = true;
                            console.log('[AutoFill] ✓ Successfully stopped continue button checking');
                        }} else {{
                            console.log('[AutoFill] ✗ continueCheckInterval not found, setting flag anyway');
                            window.continueButtonClicked = true;
                        }}

                        // 尝试查找并点击提交按钮
                        setTimeout(() => {{
                            const submitBtn = document.querySelector('button[type="submit"]') ||
                                            Array.from(document.querySelectorAll('button')).find(btn =>
                                              btn.textContent.includes('确定') ||
                                              btn.textContent.toLowerCase().includes('submit') ||
                                              btn.textContent.toLowerCase().includes('continue') ||
                                              btn.textContent.includes('继续')
                                            );

                            if (submitBtn && !submitBtn.disabled) {{
                                console.log('[AutoFill] Clicking submit button');
                                submitBtn.click();

                                // 提交后等待人机验证,不要进行任何自动操作
                                console.log('[AutoFill] Waiting for human verification, no auto actions');
                            }} else {{
                                console.log('[AutoFill] Submit button not found or disabled');
                            }}
                        }}, 500);

                        return true;
                    }} else {{
                        console.log('[AutoFill] Code input not found');
                        return false;
                    }}
                }})();
            "#, code);

            // 执行 JavaScript 代码
            match window.eval(&js_code) {
                Ok(_) => {
                    eprintln!("[EmailHelper] Successfully injected verification code to login window");
                }
                Err(e) => {
                    eprintln!("[EmailHelper] Failed to inject verification code: {}", e);
                }
            }

            break;
        }
    }

    Ok(())
}

#[tauri::command]
async fn get_current_verification_code(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let verification_code = state.verification_code.lock().unwrap();
    Ok(serde_json::json!({
        "code": verification_code.clone()
    }))
}

#[tauri::command]
async fn clear_verification_code(state: State<'_, AppState>) -> Result<(), String> {
    let mut verification_code = state.verification_code.lock().unwrap();
    *verification_code = None;
    eprintln!("[EmailHelper] Cleared verification code");
    Ok(())
}

#[tauri::command]
async fn set_card_bin(bin: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut card_bin = state.card_bin.lock().unwrap();
    *card_bin = bin.clone();
    eprintln!("[EmailHelper] Set card BIN: {}", bin);
    Ok(())
}

#[tauri::command]
async fn get_card_bin(state: State<'_, AppState>) -> Result<String, String> {
    let card_bin = state.card_bin.lock().unwrap();
    Ok(card_bin.clone())
}

#[tauri::command]
async fn set_card_address(
    country: String,
    province: String,
    city: String,
    street: String,
    postal_code: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut card_address = state.card_address.lock().unwrap();
    *card_address = CardAddress {
        country,
        province,
        city,
        street,
        postal_code,
    };
    eprintln!("[EmailHelper] Set card address: {:?}", *card_address);
    Ok(())
}

#[tauri::command]
async fn get_card_address(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let card_address = state.card_address.lock().unwrap();
    Ok(serde_json::json!({
        "country": card_address.country,
        "province": card_address.province,
        "city": card_address.city,
        "street": card_address.street,
        "postalCode": card_address.postal_code
    }))
}

#[tauri::command]
async fn set_quick_get_mode(
    email: String,
    password: String,
    register_only: bool,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut quick_get_mode = state.quick_get_mode.lock().unwrap();
    *quick_get_mode = Some(QuickGetMode {
        email: email.clone(),
        password,
        register_only,
    });
    eprintln!("[EmailHelper] Set quick get mode for: {}, register_only: {}", email, register_only);
    Ok(())
}

#[tauri::command]
async fn get_token_from_session_cookie(session_cookie: String, state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    // 使用 extract_token_from_session 函数
    let token_info = extract_token_from_session(&session_cookie)
        .await
        .map_err(|e| format!("Failed to extract token from session: {}", e))?;

    // 获取用户信息
    let app_session = exchange_auth_session_for_app_session(&session_cookie)
        .await
        .map_err(|e| format!("Failed to exchange session: {}", e))?;

    let user_info = get_user_info_with_app_session(&app_session)
        .await
        .map_err(|e| format!("Failed to get user info: {}", e))?;

    Ok(serde_json::json!({
        "tenant_url": token_info.tenant_url,
        "access_token": token_info.access_token,
        "portal_url": user_info.portal_url,
        "email": user_info.email_note,
    }))
}

#[tauri::command]
async fn save_token_from_email_helper(
    tenant_url: String,
    access_token: String,
    portal_url: Option<String>,
    email_note: String,
    session_cookie: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    use storage::{TokenStorage, TokenData};
    use chrono::Utc;

    // 获取存储管理器
    let storage = {
        let guard = state.storage_manager.lock().unwrap();
        guard.clone().ok_or("Storage manager not initialized")?
    };

    // 创建新 token
    let mut new_token = TokenData::new(
        uuid::Uuid::new_v4().to_string(),
        tenant_url,
        access_token,
        portal_url,
        Some(email_note.clone()),
    );

    // 设置 auth_session
    new_token.auth_session = Some(session_cookie);

    // 保存
    storage.save_token(&new_token).await
        .map_err(|e| format!("Failed to save token: {}", e))?;

    eprintln!("[EmailHelper] Token saved for email: {}", email_note);
    Ok(())
}

#[tauri::command]
async fn open_email_helper_window(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;

    // 检查窗口是否已经存在
    if let Some(window) = app.get_webview_window("email-helper") {
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // 在开发模式下使用完整 URL,生产模式下使用相对路径
    #[cfg(debug_assertions)]
    let url = WebviewUrl::External("http://localhost:1420/email-helper.html".parse().unwrap());

    #[cfg(not(debug_assertions))]
    let url = WebviewUrl::App("email-helper.html".into());

    // 创建新窗口
    let _window = WebviewWindowBuilder::new(
        &app,
        "email-helper",
        url
    )
    .title("邮箱助手 - Email Helper")
    .inner_size(850.0, 750.0)
    .min_inner_size(850.0, 700.0)
    .center()
    .resizable(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn open_login_window(
    app: tauri::AppHandle,
    should_clear_cache: bool,
    is_batch_mode: bool,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let window_label = format!("login_{}", chrono::Utc::now().timestamp());

    // 获取监控邮箱
    let monitoring_email = {
        let email_guard = state.monitoring_email.lock().unwrap();
        email_guard.clone()
    };

    eprintln!("[Login] Opening login window, batch mode: {}, monitoring email: {:?}", is_batch_mode, monitoring_email);

    // 保存当前登录窗口标签
    {
        let mut window_guard = state.current_login_window.lock().unwrap();
        *window_guard = Some(window_label.clone());
    }

    let monitoring_email_clone = monitoring_email.clone();
    let app_clone = app.clone();
    let window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::External("https://app.augmentcode.com/login".parse().unwrap())
    )
    .title("登录 Augment Code")
    .inner_size(1000.0, 700.0)
    .center()
    .resizable(true)
    .incognito(should_clear_cache)
    .on_navigation(move |url| {
        // 允许所有导航
        eprintln!("[Login] Navigating to: {}", url);
        true
    })
    .initialization_script(&format!(r#"
        console.log('[Tauri] Login window initialization script loaded');

        // 全局变量 - 用于控制继续按钮检查
        window.continueCheckInterval = null;
        window.continueButtonClicked = false;

        // 自动填充邮箱 - 使用更激进的策略
        const monitoringEmail = '{}';

        function tryAutoFillEmail() {{
            const emailInput = document.querySelector('input[type="email"]') ||
                              document.querySelector('input[autocomplete="email"]') ||
                              document.querySelector('input[name="email"]');

            if (emailInput && monitoringEmail && !emailInput.value) {{
                emailInput.value = monitoringEmail;
                emailInput.dispatchEvent(new Event('input', {{ bubbles: true }}));
                emailInput.dispatchEvent(new Event('change', {{ bubbles: true }}));
                emailInput.dispatchEvent(new Event('blur', {{ bubbles: true }}));
                console.log('[Tauri] Auto-filled email:', monitoringEmail);
                return true;
            }}
            return false;
        }}

        // 监听 DOM 变化,一旦发现邮箱输入框就填充
        const observer = new MutationObserver(() => {{
            if (tryAutoFillEmail()) {{
                console.log('[Tauri] Email filled via MutationObserver');
            }}
        }});

        // 开始观察
        if (document.body) {{
            observer.observe(document.body, {{ childList: true, subtree: true }});
        }} else {{
            document.addEventListener('DOMContentLoaded', () => {{
                observer.observe(document.body, {{ childList: true, subtree: true }});
            }});
        }}

        // 同时使用定时器作为备份
        let fillAttempts = 0;
        const fillInterval = setInterval(() => {{
            if (tryAutoFillEmail() || fillAttempts++ > 20) {{
                clearInterval(fillInterval);
                if (fillAttempts > 20) {{
                    observer.disconnect();
                }}
            }}
        }}, 500);

        // 自动点击 continue 按钮
        function tryAutoClickContinue() {{
            const continueBtn = document.querySelector('button[type="submit"]') ||
                               Array.from(document.querySelectorAll('button')).find(btn =>
                                 btn.textContent.toLowerCase().includes('continue') ||
                                 btn.textContent.includes('继续')
                               );

            if (continueBtn && !continueBtn.disabled && !continueBtn.hasAttribute('disabled')) {{
                console.log('[AutoClick] Clicking continue button');
                continueBtn.click();
                return true;
            }}

            return false;
        }}

        // 定时检查 continue 按钮(仅在邮箱输入页面,邮箱填充后开始,等待6秒)
        setTimeout(() => {{
            // 只在登录/注册页面检查继续按钮
            const currentUrl = window.location.href;
            if (!currentUrl.includes('/login') && !currentUrl.includes('/signup')) {{
                console.log('[AutoClick] Not on login/signup page, skipping continue button checks');
                return;
            }}

            let checkCount = 0;
            const maxChecks = 5; // 最多检查5次
            window.continueCheckInterval = setInterval(() => {{
                if (window.continueButtonClicked) {{
                    clearInterval(window.continueCheckInterval);
                    window.continueCheckInterval = null;
                    console.log('[AutoClick] Continue button already clicked, stopping checks');
                    return;
                }}

                checkCount++;
                if (checkCount > maxChecks) {{
                    clearInterval(window.continueCheckInterval);
                    window.continueCheckInterval = null;
                    console.log('[AutoClick] Max checks reached (5 attempts)');
                    return;
                }}

                console.log(`[AutoClick] Checking continue button (attempt ${{checkCount}}/${{maxChecks}})`);
                if (tryAutoClickContinue()) {{
                    window.continueButtonClicked = true;
                    clearInterval(window.continueCheckInterval);
                    window.continueCheckInterval = null;
                    console.log('[AutoClick] Continue button clicked successfully, stopped checking');
                }}
            }}, 3000);
        }}, 6000);

        // 处理 URL 的函数
        function handleUrl(currentUrl, isInitial = false) {{
            if (isInitial) {{
                console.log('[Tauri] Initial URL check:', currentUrl);
            }} else {{
                console.log('[Tauri] ✓ URL changed to:', currentUrl);
            }}

                if (currentUrl.includes('auth.augmentcode.com') ||
                    currentUrl.includes('app.augmentcode.com/get-started') ||
                    currentUrl.includes('app.augmentcode.com/account')) {{
                    console.log('[Tauri] Detected auth/get-started/account page, will check for session cookie');
                    // 等待一下让 cookie 设置完成
                    setTimeout(() => {{
                        // 通知 Rust 后端检查 cookie
                        if (window.__TAURI__ && window.__TAURI__.event) {{
                            window.__TAURI__.event.emit('check-session-cookie');
                        }}
                    }}, 1000);
                }}

                // 如果是验证码页面,打印日志
                if (currentUrl.includes('passwordless-email-challenge')) {{
                    console.log('[Tauri] Verification code page detected, waiting for code injection from backend');
                }}

                // 如果跳转到 onboard 页面,自动完成选择和点击
                if (currentUrl.includes('app.augmentcode.com/onboard')) {{
                    console.log('[Tauri] ✓ Onboard page detected, will auto-select Other option and continue');

                    // 使用轮询方式等待按钮加载完成
                    let retryCount = 0;
                    const maxRetries = 20; // 最多重试20次(10秒)

                    const trySelectOption = () => {{
                        const allButtons = document.querySelectorAll('button');

                        if (allButtons.length > 0) {{
                            console.log('[Onboard] Found', allButtons.length, 'button elements');
                            handleOnboardSelection();
                        }} else if (retryCount < maxRetries) {{
                            retryCount++;
                            console.log(`[Onboard] Buttons not found, retrying... (${{retryCount}}/${{maxRetries}})`);
                            setTimeout(trySelectOption, 500);
                        }} else {{
                            console.log('[Onboard] Timeout waiting for buttons');
                        }}
                    }};

                    // 延迟1秒后开始尝试
                    setTimeout(trySelectOption, 1000);

                    function handleOnboardSelection() {{
                        // 1. 查找所有选项按钮(排除 Continue 按钮)
                        const allButtons = document.querySelectorAll('button');
                        console.log('[Onboard] Found', allButtons.length, 'button elements');

                        // 输出所有 button 的文本内容和类名
                        Array.from(allButtons).forEach((btn, index) => {{
                            const text = btn.textContent.trim();
                            const className = btn.className;
                            if (text) console.log(`[Onboard] Button[${{index}}]: "${{text}}" (class: ${{className}})`);
                        }});

                        // 查找 "Other" 按钮
                        const otherButton = Array.from(allButtons).find(btn => {{
                            const text = btn.textContent.trim().toLowerCase();
                            return text === 'other';
                        }});

                        let selectedButton = null;
                        if (otherButton) {{
                            selectedButton = otherButton;
                            console.log(`[Onboard] Found "Other" button: "${{selectedButton.textContent.trim()}}"`);
                        }} else {{
                            console.log('[Onboard] "Other" button not found');
                        }}

                        if (selectedButton) {{
                            console.log('[Onboard] ✓ Clicking selected option...');
                            selectedButton.click();

                            // 2. 等待一下后点击 Continue 按钮
                            setTimeout(() => {{
                                const continueBtn = Array.from(document.querySelectorAll('button')).find(btn =>
                                    btn.textContent.toLowerCase().includes('continue') ||
                                    btn.textContent.includes('继续')
                                );
                                if (continueBtn) {{
                                    console.log('[Onboard] Clicking Continue button');
                                    continueBtn.click();

                                    // 3. 等待2秒后点击 Add Payment Method 按钮(增加等待时间)
                                    setTimeout(() => {{
                                        console.log('[Onboard] Looking for Add Payment Method button...');
                                        const addPaymentBtn = document.querySelector('button.payment-button') ||
                                                            Array.from(document.querySelectorAll('button')).find(btn => {{
                                                                const text = btn.textContent.trim();
                                                                return text.includes('Add Payment Method') ||
                                                                       text.includes('payment');
                                                            }});
                                        if (addPaymentBtn) {{
                                            console.log('[Onboard] Found Add Payment Method button:', addPaymentBtn.textContent.trim());
                                            console.log('[Onboard] Clicking Add Payment Method button');
                                            addPaymentBtn.click();
                                        }} else {{
                                            console.log('[Onboard] Add Payment Method button not found');
                                            // 输出所有按钮帮助调试
                                            const allBtns = document.querySelectorAll('button');
                                            console.log('[Onboard] All buttons on page:');
                                            Array.from(allBtns).forEach((btn, idx) => {{
                                                console.log(`  [${{idx}}]: "${{btn.textContent.trim()}}"`);
                                            }});
                                        }}
                                    }}, 2000);
                                }} else {{
                                    console.log('[Onboard] Continue button not found');
                                }}
                            }}, 500);
                        }} else {{
                            console.log('[Onboard] No valid option found (all buttons filtered out)');
                        }}
                    }}
                }}

            // 如果跳转到支付页面,自动填充信用卡信息
            // 只在主支付页面执行,不在 iframe 中执行
            if ((currentUrl.includes('buy.stripe.com') || currentUrl.includes('billing.augmentcode.com/c/pay'))
                && window.self === window.top) {{
                console.log('[Tauri] Main payment page detected, will auto-fill card info');
                setTimeout(() => {{
                    autoFillPaymentInfo();
                }}, 2000);
            }}
        }}

        // 监听 URL 变化
        window.lastUrl = window.location.href;

        // 初始检查一次当前 URL
        handleUrl(window.lastUrl, true);

        // 定时检查 URL 变化
        setInterval(() => {{
            const currentUrl = window.location.href;
            if (currentUrl !== window.lastUrl) {{
                window.lastUrl = currentUrl;
                handleUrl(currentUrl, false);
            }}
        }}, 500);

        // ========== 卡号生成相关函数 (参考 zhifu.js) ==========

        // Luhn 算法生成校验位
        function generateCheckDigit(number) {{
            let sum = 0;
            let shouldDouble = true;
            for (let i = number.length - 1; i >= 0; i--) {{
                let digit = parseInt(number.charAt(i), 10);
                if (shouldDouble) {{
                    digit *= 2;
                    if (digit > 9) digit -= 9;
                }}
                sum += digit;
                shouldDouble = !shouldDouble;
            }}
            return ((10 - (sum % 10)) % 10).toString();
        }}

        // 根据 BIN 判断 CVV 长度
        function getCvvLength(bin) {{
            return (bin.startsWith('34') || bin.startsWith('37')) ? 4 : 3;
        }}

        // 生成随机 CVV
        function generateRandomCVV(length) {{
            return Math.floor(Math.random() * (10 ** length)).toString().padStart(length, '0');
        }}

        // 生成随机有效期
        function generateRandomExpiryDate() {{
            const now = new Date();
            const currentYear = now.getFullYear();
            const currentMonth = now.getMonth() + 1;
            let year = currentYear + Math.floor(Math.random() * 5);
            let month = (year === currentYear)
                ? currentMonth + Math.floor(Math.random() * (13 - currentMonth))
                : Math.floor(Math.random() * 12) + 1;
            return {{
                month: month.toString().padStart(2, '0'),
                year: (year % 100).toString().padStart(2, '0')
            }};
        }}

        // 生成单个卡号
        function generateCard(bin) {{
            const cvvLength = getCvvLength(bin);
            let number = bin;
            const targetLength = (cvvLength === 4) ? 15 : 16;
            while (number.length < targetLength - 1) {{
                number += Math.floor(Math.random() * 10);
            }}
            number += generateCheckDigit(number);
            const cvv = generateRandomCVV(cvvLength);
            const expiryDate = generateRandomExpiryDate();
            return {{
                number: number,
                month: expiryDate.month,
                year: expiryDate.year,
                cvv: cvv
            }};
        }}

        // 生成中国地址
        function generateChineseAddress() {{
            // 使用 Stripe 下拉框中的 value 值(完整省份名称)
            const provinces = [
                '安徽省', '北京市', '重庆市', '福建省', '甘肃省', '广东省',
                '贵州省', '海南省', '河北省', '河南省', '黑龙江省', '湖北省',
                '湖南省', '吉林省', '江苏省', '江西省', '辽宁省', '青海省',
                '山东省', '山西省', '陕西省', '上海市', '四川省', '天津市',
                '云南省', '浙江省'
            ];

            const cities = [
                '北京', '上海', '广州', '深圳', '杭州', '成都',
                '武汉', '长沙', '郑州', '石家庄', '济南', '西安',
                '福州', '合肥', '南昌', '昆明', '贵阳', '南宁',
                '天津', '重庆', '长春', '沈阳', '哈尔滨', '太原',
                '兰州', '西宁', '海口', '南京', '苏州', '宁波'
            ];

            const streets = [
                '中山路', '人民路', '解放路', '建设路', '和平路', '胜利路',
                '文化路', '新华路', '光明路', '友谊路', '团结路', '幸福路',
                '长江路', '黄河路', '珠江路', '淮河路', '松花江路', '嘉陵江路'
            ];

            const randomProvince = provinces[Math.floor(Math.random() * provinces.length)];
            const randomCity = cities[Math.floor(Math.random() * cities.length)];
            const randomStreet = streets[Math.floor(Math.random() * streets.length)];
            const streetNumber = Math.floor(Math.random() * 900) + 100;
            const zipCode = Math.floor(Math.random() * 900000) + 100000; // 6位邮编

            return {{
                address: `${{randomStreet}}${{streetNumber}}号`,
                city: randomCity,
                province: randomProvince,
                postalCode: zipCode.toString(),
                country: 'CN'
            }};
        }}

        // 生成完整的卡号信息
        async function generateCardInfo() {{
            try {{
                // 从后端获取 BIN 和地址配置
                let bin = '515462002040'; // 默认值
                let addressConfig = null;

                try {{
                    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {{
                        bin = await window.__TAURI__.core.invoke('get_card_bin');
                        console.log('[Payment] Got BIN from backend:', bin);

                        addressConfig = await window.__TAURI__.core.invoke('get_card_address');
                        console.log('[Payment] Got address config from backend:', addressConfig);
                    }}
                }} catch (error) {{
                    console.log('[Payment] Failed to get config from backend, using default:', error);
                }}

                // 生成卡号
                const card = generateCard(bin);

                // 生成地址 - 优先使用配置的地址,没有配置则随机生成
                let address;
                if (addressConfig && (addressConfig.country || addressConfig.province || addressConfig.city || addressConfig.street || addressConfig.postalCode)) {{
                    // 使用配置的地址,未配置的字段随机生成
                    const randomAddress = generateChineseAddress();
                    address = {{
                        country: addressConfig.country || randomAddress.country,
                        province: addressConfig.province || randomAddress.province,
                        city: addressConfig.city || randomAddress.city,
                        address: addressConfig.street || randomAddress.address,
                        postalCode: addressConfig.postalCode || randomAddress.postalCode
                    }};
                    console.log('[Payment] Using configured address (with random fallback):', address);
                }} else {{
                    // 完全随机生成
                    address = generateChineseAddress();
                    console.log('[Payment] Using random address:', address);
                }}

                return {{
                    cardNumber: card.number,
                    month: card.month,
                    year: card.year,
                    cvc: card.cvv,
                    name: 'Test User',
                    address: address
                }};
            }} catch (error) {{
                console.error('[Payment] Error generating card info:', error);
                return null;
            }}
        }}

        // 自动填充支付信息函数
        async function autoFillPaymentInfo() {{
            console.log('[Payment] Starting auto-fill payment info');

            // 生成卡号信息
            const cardInfo = await generateCardInfo();
            if (!cardInfo) {{
                console.error('[Payment] Failed to generate card info');
                return;
            }}

            const {{ cardNumber, month, year, cvc, name, address }} = cardInfo;
            console.log('[Payment] Generated card info:', {{ cardNumber, month, year, cvc, name }});

            try {{
                // 辅助函数:设置输入框的值并触发事件
                async function setNativeValue(element, value) {{
                    if (!element) return false;

                    // 如果是 select 下拉框,需要特殊处理
                    if (element.tagName === 'SELECT') {{
                        // 尝试通过文本内容匹配选项
                        const options = Array.from(element.options);
                        let matchedOption = options.find(opt => opt.text.trim() === value);

                        // 如果没有精确匹配,尝试部分匹配
                        if (!matchedOption) {{
                            matchedOption = options.find(opt => opt.text.includes(value) || value.includes(opt.text));
                        }}

                        if (matchedOption) {{
                            element.value = matchedOption.value;
                            console.log(`[Payment] Selected option: ${{matchedOption.text}} (value: ${{matchedOption.value}})`);
                        }} else {{
                            console.log(`[Payment] No matching option found for: ${{value}}`);
                            console.log(`[Payment] Available options:`, options.map(opt => opt.text));
                            return false;
                        }}
                    }} else {{
                        // 普通输入框
                        const valueSetter = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(element), 'value')?.set;
                        if (valueSetter) {{
                            valueSetter.call(element, value);
                        }} else {{
                            element.value = value;
                        }}
                    }}

                    ['input', 'change', 'blur'].forEach(eventType => {{
                        element.dispatchEvent(new Event(eventType, {{ bubbles: true }}));
                    }});

                    await new Promise(resolve => setTimeout(resolve, 50));
                    return true;
                }}

                // 查找并填充卡号
                const cardNumberInput = document.getElementById('cardNumber');
                if (cardNumberInput) {{
                    await setNativeValue(cardNumberInput, cardNumber);
                    console.log('[Payment] Card number filled');
                }} else {{
                    console.log('[Payment] Card number input not found');
                }}

                // 查找并填充有效期
                const cardExpiryInput = document.getElementById('cardExpiry');
                if (cardExpiryInput) {{
                    await setNativeValue(cardExpiryInput, `${{month}}/${{year}}`);
                    console.log('[Payment] Card expiry filled');
                }} else {{
                    console.log('[Payment] Card expiry input not found');
                }}

                // 查找并填充 CVC
                const cardCvcInput = document.getElementById('cardCvc');
                if (cardCvcInput) {{
                    await setNativeValue(cardCvcInput, cvc);
                    console.log('[Payment] Card CVC filled');
                }} else {{
                    console.log('[Payment] Card CVC input not found');
                }}

                // 查找并填充姓名
                const billingNameInput = document.getElementById('billingName');
                if (billingNameInput) {{
                    await setNativeValue(billingNameInput, name);
                    console.log('[Payment] Billing name filled');
                }} else {{
                    console.log('[Payment] Billing name input not found');
                }}

                // 填充地址信息
                const addressMappings = [
                    {{ id: 'billingAddressLine1', value: address.address, label: '地址' }},
                    {{ id: 'billingLocality', value: address.city, label: '城市' }},
                    {{ id: 'billingAdministrativeArea', value: address.province, label: '省份' }},
                    {{ id: 'billingPostalCode', value: address.postalCode, label: '邮政编码' }}
                ];

                for (const mapping of addressMappings) {{
                    const element = document.getElementById(mapping.id);
                    if (element && mapping.value) {{
                        await setNativeValue(element, mapping.value);
                        console.log(`[Payment] ${{mapping.label}}已填写: ${{mapping.value}}`);
                    }} else if (!element) {{
                        console.log(`[Payment] ${{mapping.label}}字段未找到 (ID: ${{mapping.id}})`);
                    }}
                }}

                // 等待一下后点击提交按钮
                setTimeout(() => {{
                    const submitButton = document.querySelector("button[data-testid='hosted-payment-submit-button']");
                    if (submitButton) {{
                        console.log('[Payment] Clicking submit button');
                        submitButton.click();
                    }} else {{
                        console.log('[Payment] Submit button not found');
                    }}
                }}, 1000);

            }} catch (error) {{
                console.error('[Payment] Error filling payment info:', error);
            }}
        }}
    "#, monitoring_email.unwrap_or_default()))
    .on_page_load(move |window, payload| {
        use tauri::webview::PageLoadEvent;
        if payload.event() == PageLoadEvent::Finished {
            let url_str = payload.url().to_string();

            // 检查是否是 auth.augmentcode.com 或 app.augmentcode.com/account 或 app.augmentcode.com/get-started
            if url_str.contains("auth.augmentcode.com") ||
               url_str.contains("app.augmentcode.com/account") ||
               url_str.contains("app.augmentcode.com/get-started") {
                let window_clone = window.clone();
                let app_handle_clone = app_clone.clone();

                eprintln!("[Login] Page loaded: {}, checking for session cookie", url_str);

                // 在后台线程中获取 Cookie
                tauri::async_runtime::spawn(async move {
                    // 检查是否是 register_only 模式
                    let is_register_only = {
                        let app_state: tauri::State<AppState> = app_handle_clone.state();
                        let mode_guard = app_state.quick_get_mode.lock().unwrap();
                        mode_guard.as_ref().map(|m| m.register_only).unwrap_or(false)
                    };

                    eprintln!("[Login] Checking session cookie, register_only: {}, url: {}", is_register_only, url_str);

                    // 如果是 register_only 模式,不要在登录成功后立即发送 session cookie
                    // 只在 get-started 或 account 页面发送(表示 onboarding 完成)
                    if is_register_only {
                        if !url_str.contains("app.augmentcode.com/get-started") && !url_str.contains("app.augmentcode.com/account") {
                            eprintln!("[Login] Register-only mode, skipping session cookie check until onboarding complete (waiting for get-started or account page)");
                            return;
                        }
                        eprintln!("[Login] Register-only mode, onboarding complete, will get session cookie");
                    }

                    // 等待一小段时间确保 Cookie 已设置
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

                    match window_clone.cookies_for_url(
                        "https://auth.augmentcode.com".parse().unwrap()
                    ) {
                        Ok(cookies) => {
                            // 查找 session Cookie
                            if let Some(session_cookie) = cookies.iter()
                                .find(|c| c.name() == "session")
                            {
                                let session_value = session_cookie.value().to_string();
                                eprintln!("[Login] Found session cookie, sending to frontend");

                                // 发送 session cookie 到前端
                                let _ = app_handle_clone.emit(
                                    "session-cookie-received",
                                    session_value
                                );

                                eprintln!("[Login] Session cookie sent to frontend");

                            } else {
                                eprintln!("[Login] Session cookie not found");
                            }
                        }
                        Err(e) => {
                            eprintln!("[Login] Failed to get cookies: {}", e);
                        }
                    }
                });
            }
        }
    })
    .build()
    .map_err(|e| format!("Failed to create login window: {}", e))?;

    Ok(format!("Login window opened: {}", window_label))
}

// 关闭登录窗口
#[tauri::command]
async fn close_login_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // 获取当前登录窗口标签
    let window_label = {
        let window_guard = state.current_login_window.lock().unwrap();
        window_guard.clone()
    };

    if let Some(label) = window_label {
        eprintln!("[Login] Closing login window: {}", label);

        // 查找并关闭窗口
        if let Some(window) = app.get_webview_window(&label) {
            window.close().map_err(|e| format!("Failed to close window: {}", e))?;

            // 清除当前登录窗口标签
            let mut window_guard = state.current_login_window.lock().unwrap();
            *window_guard = None;

            Ok(format!("Login window closed: {}", label))
        } else {
            Err(format!("Login window not found: {}", label))
        }
    } else {
        Err("No login window is currently open".to_string())
    }
}

// 数据库配置相关命令
#[tauri::command]
async fn save_database_config(
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    ssl_mode: Option<String>,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let config_manager = DatabaseConfigManager::new(&app)
        .map_err(|e| format!("Failed to create config manager: {}", e))?;

    let ssl_mode = match ssl_mode.as_deref() {
        Some("disable") => database::SslMode::Disable,
        Some("require") => database::SslMode::Require,
        _ => database::SslMode::Prefer,
    };

    let config = DatabaseConfig::new_with_ssl(host, port, database, username, password, ssl_mode);

    config_manager.save_config(&config)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    // 尝试初始化数据库连接
    let mut db_manager = DatabaseManager::new(config);
    match db_manager.initialize().await {
        Ok(_) => {
            // 检查数据库表是否已存在
            if let Some(pool) = db_manager.get_pool() {
                let client = pool.get().await
                    .map_err(|e| format!("Failed to get database client: {}", e))?;

                let tables_exist = database::check_tables_exist(&client).await
                    .map_err(|e| format!("Failed to check tables: {}", e))?;

                if !tables_exist {
                    // 表不存在，创建表
                    database::create_tables(&client).await
                        .map_err(|e| format!("Failed to create tables: {}", e))?;
                } else {
                    // 表已存在，检查并添加新字段
                    println!("Database tables already exist, checking for new fields");
                    database::add_new_fields_if_not_exist(&client).await
                        .map_err(|e| format!("Failed to add new fields: {}", e))?;
                }
            }

            // 更新应用状态
            *state.database_manager.lock().unwrap() = Some(Arc::new(db_manager));

            // 重新初始化存储管理器
            initialize_storage_manager(&app, &state).await
                .map_err(|e| format!("Failed to initialize storage: {}", e))?;

            // 如果表已存在，执行初始同步
            let pool_option = {
                let db_guard = state.database_manager.lock().unwrap();
                db_guard.as_ref().and_then(|db| db.get_pool())
            };

            if let Some(pool) = pool_option {
                let client = pool.get().await
                    .map_err(|e| format!("Failed to get database client for sync check: {}", e))?;

                let tables_exist = database::check_tables_exist(&client).await
                    .map_err(|e| format!("Failed to check tables for sync: {}", e))?;

                if tables_exist {
                    // 执行初始双向同步
                    let storage_manager = {
                        let storage_guard = state.storage_manager.lock().unwrap();
                        storage_guard.as_ref().cloned()
                    };

                    if let Some(storage_manager) = storage_manager {
                        match storage_manager.bidirectional_sync().await {
                            Ok(sync_result) => {
                                println!("Initial sync completed: {} tokens synced", sync_result.tokens_synced);
                            }
                            Err(e) => {
                                eprintln!("Initial sync failed: {}", e);
                                // 同步失败不影响配置保存
                            }
                        }
                    }
                }
            }

            Ok(())
        }
        Err(e) => Err(format!("Failed to connect to database: {}", e))
    }
}

#[tauri::command]
async fn load_database_config(
    app: tauri::AppHandle,
) -> Result<DatabaseConfig, String> {
    let config_manager = DatabaseConfigManager::new(&app)
        .map_err(|e| format!("Failed to create config manager: {}", e))?;

    config_manager.load_config()
        .map_err(|e| format!("Failed to load config: {}", e))
}

#[tauri::command]
async fn test_database_connection(
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    ssl_mode: Option<String>,
) -> Result<(), String> {
    let ssl_mode = match ssl_mode.as_deref() {
        Some("disable") => database::SslMode::Disable,
        Some("require") => database::SslMode::Require,
        _ => database::SslMode::Prefer,
    };

    let config = DatabaseConfig::new_with_ssl(host, port, database, username, password, ssl_mode);

    database::test_database_connection(&config).await
        .map_err(|e| format!("Connection test failed: {}", e))
}

#[tauri::command]
async fn delete_database_config(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let config_manager = DatabaseConfigManager::new(&app)
        .map_err(|e| format!("Failed to create config manager: {}", e))?;

    config_manager.delete_config()
        .map_err(|e| format!("Failed to delete config: {}", e))?;

    // 清除应用状态中的数据库管理器
    *state.database_manager.lock().unwrap() = None;

    // 重新初始化存储管理器（仅本地存储）
    initialize_storage_manager(&app, &state).await
        .map_err(|e| format!("Failed to reinitialize storage: {}", e))?;

    Ok(())
}

// 同步相关命令
#[tauri::command]
async fn sync_tokens_to_database(
    state: State<'_, AppState>,
) -> Result<storage::SyncStatus, String> {
    let storage_manager = {
        let guard = state.storage_manager.lock().unwrap();
        guard.clone().ok_or("Storage manager not initialized")?
    };

    storage_manager.sync_local_to_remote().await
        .map_err(|e| format!("Sync failed: {}", e))
}

#[tauri::command]
async fn sync_tokens_from_database(
    state: State<'_, AppState>,
) -> Result<storage::SyncStatus, String> {
    let storage_manager = {
        let guard = state.storage_manager.lock().unwrap();
        guard.clone().ok_or("Storage manager not initialized")?
    };

    storage_manager.sync_remote_to_local().await
        .map_err(|e| format!("Sync failed: {}", e))
}

#[tauri::command]
async fn delete_token(
    token_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let storage_manager = {
        let guard = state.storage_manager.lock().unwrap();
        guard.clone().ok_or("Storage manager not initialized")?
    };

    storage_manager.delete_token(&token_id).await
        .map_err(|e| format!("Delete failed: {}", e))
}

#[tauri::command]
async fn bidirectional_sync_tokens(
    state: State<'_, AppState>,
) -> Result<storage::SyncStatus, String> {
    let storage_manager = {
        let guard = state.storage_manager.lock().unwrap();
        guard.clone().ok_or("Storage manager not initialized")?
    };

    storage_manager.bidirectional_sync().await
        .map_err(|e| format!("Sync failed: {}", e))
}

#[tauri::command]
async fn bidirectional_sync_tokens_with_data(
    tokens_json: String,
    state: State<'_, AppState>,
) -> Result<storage::SyncStatus, String> {
    let storage_manager = {
        let guard = state.storage_manager.lock().unwrap();
        guard.clone().ok_or("Storage manager not initialized")?
    };

    // 解析前端传入的 tokens JSON
    let tokens: Vec<storage::TokenData> = serde_json::from_str(&tokens_json)
        .map_err(|e| format!("Failed to parse tokens JSON: {}", e))?;

    storage_manager.bidirectional_sync_with_tokens(tokens).await
        .map_err(|e| format!("Sync failed: {}", e))
}

#[tauri::command]
async fn get_storage_status(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // 首先尝试重新加载数据库配置（如果还没有的话）
    let db_manager_exists = {
        let guard = state.database_manager.lock().unwrap();
        guard.is_some()
    };

    if !db_manager_exists {
        // 尝试加载数据库配置
        match database::DatabaseConfigManager::new(&app) {
            Ok(config_manager) => {
                match config_manager.load_config() {
                    Ok(config) => {
                        if config.enabled {
                            let mut db_manager = database::DatabaseManager::new(config);
                            if db_manager.initialize().await.is_ok() {
                                *state.database_manager.lock().unwrap() = Some(Arc::new(db_manager));
                            }
                        }
                    }
                    Err(_) => {
                        // 配置加载失败，继续使用本地存储
                    }
                }
            }
            Err(_) => {
                // 配置管理器创建失败，继续使用本地存储
            }
        }
    }

    // 检查存储管理器是否已初始化，如果没有则尝试初始化
    let storage_manager = {
        let manager_option = {
            let guard = state.storage_manager.lock().unwrap();
            guard.clone()
        };

        if let Some(manager) = manager_option {
            manager
        } else {
            // 尝试初始化存储管理器
            if let Err(e) = initialize_storage_manager(&app, &state).await {
                return Err(format!("Failed to initialize storage manager: {}", e));
            }
            // 重新获取存储管理器
            let guard = state.storage_manager.lock().unwrap();
            guard.clone().ok_or("Storage manager still not initialized after initialization attempt")?
        }
    };

    let is_available = storage_manager.is_available().await;
    let storage_type = storage_manager.storage_type();
    let is_database_available = storage_manager.is_database_available();

    Ok(serde_json::json!({
        "is_available": is_available,
        "storage_type": storage_type,
        "is_database_available": is_database_available
    }))
}

#[tauri::command]
async fn get_sync_status(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Option<storage::SyncStatus>, String> {
    // 检查存储管理器是否已初始化，如果没有则尝试初始化
    let storage_manager = {
        let manager_option = {
            let guard = state.storage_manager.lock().unwrap();
            guard.clone()
        };

        if let Some(manager) = manager_option {
            manager
        } else {
            // 尝试初始化存储管理器
            if let Err(e) = initialize_storage_manager(&app, &state).await {
                return Err(format!("Failed to initialize storage manager: {}", e));
            }
            // 重新获取存储管理器
            let guard = state.storage_manager.lock().unwrap();
            guard.clone().ok_or("Storage manager still not initialized after initialization attempt")?
        }
    };

    storage_manager.get_sync_status().await
        .map_err(|e| format!("Failed to get sync status: {}", e))
}

// 代理配置相关命令
#[tauri::command]
async fn save_proxy_config(
    app: tauri::AppHandle,
    proxy_type: String,
    enabled: bool,
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    custom_url: Option<String>,
) -> Result<(), String> {
    let proxy_type = match proxy_type.as_str() {
        "system" => proxy_config::ProxyType::System,
        "http" => proxy_config::ProxyType::Http,
        "https" => proxy_config::ProxyType::Https,
        "socks5" => proxy_config::ProxyType::Socks5,
        "custom_url" => proxy_config::ProxyType::CustomUrl,
        _ => return Err(format!("Unknown proxy type: {}", proxy_type)),
    };

    let config = proxy_config::ProxyConfig {
        enabled,
        proxy_type,
        host: host.unwrap_or_default(),
        port: port.unwrap_or(7890),
        username,
        password,
        custom_url,
    };

    proxy_config::save_proxy_config(&app, &config)
        .map_err(|e| format!("Failed to save proxy config: {}", e))
}

#[tauri::command]
async fn load_proxy_config(app: tauri::AppHandle) -> Result<proxy_config::ProxyConfig, String> {
    proxy_config::load_proxy_config(&app)
        .map_err(|e| format!("Failed to load proxy config: {}", e))
}

#[tauri::command]
async fn test_proxy_config(
    proxy_type: String,
    enabled: bool,
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    custom_url: Option<String>,
) -> Result<(), String> {
    let proxy_type = match proxy_type.as_str() {
        "system" => proxy_config::ProxyType::System,
        "http" => proxy_config::ProxyType::Http,
        "https" => proxy_config::ProxyType::Https,
        "socks5" => proxy_config::ProxyType::Socks5,
        "custom_url" => proxy_config::ProxyType::CustomUrl,
        _ => return Err(format!("Unknown proxy type: {}", proxy_type)),
    };

    let config = proxy_config::ProxyConfig {
        enabled,
        proxy_type,
        host: host.unwrap_or_default(),
        port: port.unwrap_or(7890),
        username,
        password,
        custom_url,
    };

    proxy_config::test_proxy_connection(&config).await
}

#[tauri::command]
async fn delete_proxy_config(app: tauri::AppHandle) -> Result<(), String> {
    proxy_config::delete_proxy_config(&app)
        .map_err(|e| format!("Failed to delete proxy config: {}", e))
}

#[tauri::command]
async fn proxy_config_exists(app: tauri::AppHandle) -> Result<bool, String> {
    proxy_config::proxy_config_exists(&app)
        .map_err(|e| format!("Failed to check proxy config: {}", e))
}

// 辅助函数：初始化存储管理器
async fn initialize_storage_manager(
    app: &tauri::AppHandle,
    state: &State<'_, AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建本地存储
    let local_storage = Arc::new(LocalFileStorage::new(app)?);

    // 尝试加载数据库配置并创建数据库存储
    let postgres_storage = {
        let db_manager_guard = state.database_manager.lock().unwrap();
        if let Some(db_manager) = db_manager_guard.as_ref() {
            Some(Arc::new(PostgreSQLStorage::new(db_manager.clone())))
        } else {
            None
        }
    };

    // 创建双重存储管理器
    let dual_storage = Arc::new(DualStorage::new(local_storage, postgres_storage));

    // 更新应用状态
    *state.storage_manager.lock().unwrap() = Some(dual_storage);

    Ok(())
}



fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_state = AppState {
                augment_oauth_state: Mutex::new(None),
                http_server: Mutex::new(None),
                storage_manager: Arc::new(Mutex::new(None)),
                database_manager: Arc::new(Mutex::new(None)),
                app_session_cache: Arc::new(Mutex::new(HashMap::new())),
                monitoring_email: Mutex::new(None),
                verification_code: Mutex::new(None),
                quick_get_mode: Mutex::new(None),
                current_login_window: Mutex::new(None),
                card_bin: Mutex::new("515462002040".to_string()), // 默认BIN,与 zhifu.js 一致
                card_address: Mutex::new(CardAddress::default()), // 默认空地址
            };

            app.manage(app_state);

            // 异步初始化存储管理器
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<AppState>();

                // 尝试加载数据库配置
                let mut should_sync = false;
                match DatabaseConfigManager::new(&app_handle) {
                    Ok(config_manager) => {
                        match config_manager.load_config() {
                            Ok(config) => {
                                if config.enabled {
                                    let mut db_manager = DatabaseManager::new(config);
                                    if db_manager.initialize().await.is_ok() {
                                        // 检查表是否存在
                                        should_sync = if let Some(pool) = db_manager.get_pool() {
                                            match pool.get().await {
                                                Ok(client) => {
                                                    match database::check_tables_exist(&client).await {
                                                        Ok(exists) => {
                                                            if !exists {
                                                                // 创建表
                                                                if let Err(e) = database::create_tables(&client).await {
                                                                    eprintln!("Failed to create tables on startup: {}", e);
                                                                }
                                                                false // 新创建的表不需要同步
                                                            } else {
                                                                // 表已存在，检查并添加新字段
                                                                if let Err(e) = database::add_new_fields_if_not_exist(&client).await {
                                                                    eprintln!("Failed to add new fields on startup: {}", e);
                                                                }
                                                                true // 表已存在，需要同步
                                                            }
                                                        }
                                                        Err(e) => {
                                                            eprintln!("Failed to check tables on startup: {}", e);
                                                            false
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("Failed to get database client on startup: {}", e);
                                                    false
                                                }
                                            }
                                        } else {
                                            false
                                        };

                                        *state.database_manager.lock().unwrap() = Some(Arc::new(db_manager));

                                        // 如果需要同步，在存储管理器初始化后执行
                                        if should_sync {
                                            // 初始化存储管理器
                                            if let Err(e) = initialize_storage_manager(&app_handle, &state).await {
                                                eprintln!("Failed to initialize storage manager on startup: {}", e);
                                            } else {
                                                // 执行初始同步
                                                let storage_manager = {
                                                    let storage_guard = state.storage_manager.lock().unwrap();
                                                    storage_guard.as_ref().cloned()
                                                };

                                                if let Some(storage_manager) = storage_manager {
                                                    match storage_manager.bidirectional_sync().await {
                                                        Ok(sync_result) => {
                                                            println!("Startup sync completed: {} tokens synced", sync_result.tokens_synced);
                                                        }
                                                        Err(e) => {
                                                            eprintln!("Startup sync failed: {}", e);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!("Failed to load database config: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to create config manager: {}", e),
                }

                // 如果没有执行同步（表不存在或数据库不可用），则初始化存储管理器
                if !should_sync {
                    if let Err(e) = initialize_storage_manager(&app_handle, &state).await {
                        eprintln!("Failed to initialize storage manager: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_auth_url,
            generate_augment_auth_url,
            get_token,
            get_augment_token,
            check_account_status,
            batch_check_tokens_status,
            fetch_batch_credit_consumption,
            add_token_from_session,
            open_url,
            // 新的简化命令
            save_tokens_json,
            load_tokens_json,
            // 书签管理命令
            add_bookmark,
            update_bookmark,
            delete_bookmark,
            get_bookmarks,
            get_all_bookmarks,
            // API 调用命令
            get_customer_info,
            get_ledger_summary,
            test_api_call,
            open_data_folder,
            open_editor_with_protocol,
            create_jetbrains_token_file,
            // 邮箱助手命令
            open_email_helper_window,
            set_monitoring_email,
            set_verification_code,
            get_current_verification_code,
            clear_verification_code,
            set_card_bin,
            get_card_bin,
            set_card_address,
            get_card_address,
            set_quick_get_mode,
            open_login_window,
            close_login_window,
            get_token_from_session_cookie,
            save_token_from_email_helper,
            // 数据库配置命令
            save_database_config,
            load_database_config,
            test_database_connection,
            delete_database_config,
            // 代理配置命令
            save_proxy_config,
            load_proxy_config,
            test_proxy_config,
            delete_proxy_config,
            proxy_config_exists,
            // 同步命令
            sync_tokens_to_database,
            sync_tokens_from_database,
            // 删除命令
            delete_token,
            bidirectional_sync_tokens,
            bidirectional_sync_tokens_with_data,
            get_storage_status,
            get_sync_status,

            open_internal_browser,
            close_window,
            check_for_updates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
