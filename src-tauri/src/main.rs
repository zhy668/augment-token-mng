// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod oauth;
mod augment_oauth;
mod storage;
mod bookmarks;
mod user;
mod http_server;

use oauth::{create_oauth_state, generate_authorize_url, complete_oauth_flow, OAuthState, LoginResult};
use augment_oauth::{create_augment_oauth_state, generate_augment_authorize_url, complete_augment_oauth_flow, AugmentOAuthState, AugmentTokenResponse};
use storage::{TokenManager, StoredToken};
use bookmarks::{BookmarkManager, Bookmark};
use user::{UserManager, UserMode, UserInfo, create_user_info_from_oauth};
use http_server::HttpServer;
use std::sync::Mutex;
use tauri::{State, Manager, Emitter, WebviewWindowBuilder, WebviewUrl};
use chrono;

// Global state to store OAuth state and user state
struct AppState {
    oauth_state: Mutex<Option<OAuthState>>,
    augment_oauth_state: Mutex<Option<AugmentOAuthState>>,
    user_manager: Mutex<UserManager>,
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
async fn generate_forum_auth_url(state: State<'_, AppState>) -> Result<String, String> {
    let oauth_state = create_oauth_state();
    let auth_url = generate_authorize_url(&oauth_state)
        .map_err(|e| format!("Failed to generate forum auth URL: {}", e))?;
    
    // Store the OAuth state
    *state.oauth_state.lock().unwrap() = Some(oauth_state);
    
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
async fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))
}

#[tauri::command]
async fn save_token(
    tenant_url: String,
    access_token: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    token_manager.add_token(tenant_url, access_token)
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
async fn cleanup_expired_tokens(
    app: tauri::AppHandle,
) -> Result<usize, String> {
    let token_manager = TokenManager::new(&app)
        .map_err(|e| format!("Failed to initialize token manager: {}", e))?;

    token_manager.cleanup_expired_tokens()
        .map_err(|e| format!("Failed to cleanup expired tokens: {}", e))
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
async fn get_user_mode(state: State<'_, AppState>) -> Result<UserMode, String> {
    let user_manager = state.user_manager.lock().unwrap();
    Ok(user_manager.get_current_mode().clone())
}

#[tauri::command]
async fn set_guest_mode(state: State<'_, AppState>) -> Result<(), String> {
    let mut user_manager = state.user_manager.lock().unwrap();
    user_manager.set_guest_mode();
    Ok(())
}

#[tauri::command]
async fn set_authenticated_mode(
    state: State<'_, AppState>,
    user_info: UserInfo,
) -> Result<(), String> {
    let mut user_manager = state.user_manager.lock().unwrap();
    user_manager.set_authenticated_mode(user_info);
    Ok(())
}

#[tauri::command]
async fn logout_user(state: State<'_, AppState>) -> Result<(), String> {
    let mut user_manager = state.user_manager.lock().unwrap();
    user_manager.logout();
    Ok(())
}

#[tauri::command]
async fn can_access_email_features(state: State<'_, AppState>) -> Result<bool, String> {
    let user_manager = state.user_manager.lock().unwrap();
    Ok(user_manager.can_access_email_features())
}

#[tauri::command]
async fn start_forum_oauth_login(state: State<'_, AppState>) -> Result<LoginResult, String> {
    // Create OAuth state
    let oauth_state = create_oauth_state();
    let auth_url = generate_authorize_url(&oauth_state)
        .map_err(|e| format!("Failed to generate auth URL: {}", e))?;

    // Store the OAuth state
    *state.oauth_state.lock().unwrap() = Some(oauth_state.clone());

    // Start HTTP server
    let mut http_server = HttpServer::new();

    // Open the authorization URL in browser
    if let Err(e) = open::that(&auth_url) {
        return Err(format!("Failed to open browser: {}", e));
    }

    // Wait for callback
    let callback_result = http_server.start_and_wait_for_callback().await
        .map_err(|e| format!("OAuth callback failed: {}", e))?;

    // Complete OAuth flow
    let login_result = complete_oauth_flow(&oauth_state, &callback_result.code, &callback_result.state)
        .await
        .map_err(|e| format!("Failed to complete OAuth flow: {}", e))?;

    // Update user state
    let user_info = create_user_info_from_oauth(
        login_result.user_info.id.to_string(),
        login_result.user_info.username.clone(),
        login_result.user_info.avatar_template.clone(),
        login_result.user_info.email.clone(),
    );

    {
        let mut user_manager = state.user_manager.lock().unwrap();
        user_manager.set_authenticated_mode(user_info);
    }

    Ok(login_result)
}

#[tauri::command]
async fn start_forum_oauth_login_internal(
    app: tauri::AppHandle,
    state: State<'_, AppState>
) -> Result<String, String> {
    // Create OAuth state
    let oauth_state = create_oauth_state();
    let auth_url = generate_authorize_url(&oauth_state)
        .map_err(|e| format!("Failed to generate auth URL: {}", e))?;

    // Store the OAuth state
    *state.oauth_state.lock().unwrap() = Some(oauth_state.clone());

    // Start HTTP server in background
    let mut http_server = HttpServer::new();
    let server_handle = tokio::spawn(async move {
        http_server.start_and_wait_for_callback().await
    });

    // Create internal browser window
    let window_label = format!("oauth_browser_{}", chrono::Utc::now().timestamp());

    let _window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::External(auth_url.parse().unwrap())
    )
    .title("OAuth 授权 - 论坛登录")
    .inner_size(800.0, 700.0)
    .center()
    .resizable(true)
    .build()
    .map_err(|e| format!("Failed to create OAuth window: {}", e))?;

    // Store server handle for cleanup
    {
        let mut http_server_guard = state.http_server.lock().unwrap();
        *http_server_guard = Some(HttpServer::new()); // Placeholder for cleanup
    }

    // Monitor for OAuth completion in background
    let app_handle = app.clone();
    let window_label_clone = window_label.clone();

    tokio::spawn(async move {
        // Wait for server to complete
        if let Ok(callback_result) = server_handle.await {
            match callback_result {
                Ok(callback_result) => {
                    // Get stored OAuth state from app state
                    let oauth_state = {
                        let app_state: tauri::State<AppState> = app_handle.state();
                        let guard = app_state.oauth_state.lock().unwrap();
                        guard.clone()
                    };

                    if let Some(oauth_state) = oauth_state {
                        // Complete OAuth flow
                        match complete_oauth_flow(&oauth_state, &callback_result.code, &callback_result.state).await {
                            Ok(login_result) => {
                                // Update user state
                                let user_info = create_user_info_from_oauth(
                                    login_result.user_info.id.to_string(),
                                    login_result.user_info.username.clone(),
                                    login_result.user_info.avatar_template.clone(),
                                    login_result.user_info.email.clone(),
                                );

                                {
                                    let app_state: tauri::State<AppState> = app_handle.state();
                                    let mut user_manager = app_state.user_manager.lock().unwrap();
                                    user_manager.set_authenticated_mode(user_info);
                                }

                                // Emit success event to frontend
                                let _ = app_handle.emit("oauth_completed", &login_result);

                                // Close OAuth window
                                if let Some(window) = app_handle.get_webview_window(&window_label_clone) {
                                    let _ = window.close();
                                }
                            }
                            Err(e) => {
                                // Emit error event to frontend
                                let _ = app_handle.emit("oauth_error", format!("OAuth flow failed: {}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    // Emit error event to frontend
                    let _ = app_handle.emit("oauth_error", format!("OAuth callback failed: {}", e));
                }
            }
        }
    });

    Ok(window_label)
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
            let user_manager = UserManager::new_with_storage(app.handle())
                .unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to initialize user manager with storage: {}", e);
                    UserManager::new()
                });

            app.manage(AppState {
                oauth_state: Mutex::new(None),
                augment_oauth_state: Mutex::new(None),
                user_manager: Mutex::new(user_manager),
                http_server: Mutex::new(None),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_auth_url,
            generate_augment_auth_url,
            generate_forum_auth_url,
            get_token,
            get_augment_token,
            open_url,
            save_token,
            get_all_tokens,
            delete_token,
            cleanup_expired_tokens,
            add_bookmark,
            update_bookmark,
            delete_bookmark,
            get_bookmarks,
            get_all_bookmarks,
            open_data_folder,
            get_user_mode,
            set_guest_mode,
            set_authenticated_mode,
            logout_user,
            can_access_email_features,
            start_forum_oauth_login,
            start_forum_oauth_login_internal,
            open_internal_browser,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
