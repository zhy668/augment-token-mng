// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod augment_oauth;
mod storage;
mod bookmarks;
mod http_server;

use augment_oauth::{create_augment_oauth_state, generate_augment_authorize_url, complete_augment_oauth_flow, check_account_ban_status, AugmentOAuthState, AugmentTokenResponse, AccountStatus};
use storage::{TokenManager, StoredToken, PortalInfo};
use bookmarks::{BookmarkManager, Bookmark};
use http_server::HttpServer;
use std::sync::Mutex;
use tauri::{State, Manager, WebviewWindowBuilder, WebviewUrl};
use chrono;

// Global state to store OAuth state
struct AppState {
    augment_oauth_state: Mutex<Option<AugmentOAuthState>>,
    http_server: Mutex<Option<HttpServer>>,
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
async fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))
}

#[tauri::command]
async fn save_token(
    tenant_url: String,
    access_token: String,
    portal_url: Option<String>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    token_manager.add_token_with_portal(tenant_url, access_token, portal_url)
        .map_err(|e| format!("Failed to save token: {}", e))
}

#[tauri::command]
async fn get_all_tokens(
    app: tauri::AppHandle,
) -> Result<Vec<StoredToken>, String> {
    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    token_manager.get_all_tokens()
        .map_err(|e| format!("Failed to load tokens: {}", e))
}

#[tauri::command]
async fn delete_token(
    id: String,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    token_manager.remove_token(&id)
        .map_err(|e| format!("Failed to delete token: {}", e))
}

#[tauri::command]
async fn update_token(
    id: String,
    tenant_url: String,
    access_token: String,
    portal_url: Option<String>,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    token_manager.update_token(&id, tenant_url, access_token, portal_url)
        .map_err(|e| format!("Failed to update token: {}", e))
}

#[tauri::command]
async fn update_token_ban_status(
    id: String,
    ban_status: Option<String>,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    token_manager.update_token_ban_status(&id, ban_status)
        .map_err(|e| format!("Failed to update token ban status: {}", e))
}

#[tauri::command]
async fn update_token_portal_info(
    id: String,
    credits_balance: i64,
    expiry_date: String,
    is_active: bool,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    println!("update_token_portal_info called with:");
    println!("  id: {}", id);
    println!("  credits_balance: {}", credits_balance);
    println!("  expiry_date: {}", expiry_date);
    println!("  is_active: {}", is_active);

    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    let portal_info = PortalInfo {
        credits_balance,
        expiry_date,
        is_active,
        last_updated: chrono::Utc::now(),
    };

    let result = token_manager.update_token_portal_info(&id, Some(portal_info))
        .map_err(|e| format!("Failed to update token portal info: {}", e))?;

    println!("Portal info update result: {}", result);
    Ok(result)
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
    title: Option<String>
) -> Result<String, String> {
    let window_label = format!("browser_{}", chrono::Utc::now().timestamp());

    let _window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
    )
    .title(&title.unwrap_or_else(|| "内置浏览器".to_string()))
    .inner_size(1000.0, 700.0)
    .center()
    .resizable(true)
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

    let client = reqwest::Client::new();
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

    let client = reqwest::Client::new();
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

    let client = reqwest::Client::new();
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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState {
                augment_oauth_state: Mutex::new(None),
                http_server: Mutex::new(None),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_auth_url,
            generate_augment_auth_url,
            get_token,
            get_augment_token,
            check_account_status,
            open_url,
            save_token,
            get_all_tokens,
            delete_token,
            update_token,
            update_token_ban_status,
            update_token_portal_info,
            add_bookmark,
            update_bookmark,
            delete_bookmark,
            get_bookmarks,
            get_all_bookmarks,
            get_customer_info,
            get_ledger_summary,
            test_api_call,
            open_data_folder,

            open_internal_browser,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
