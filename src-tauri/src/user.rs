use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub login_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum UserMode {
    Guest,
    Authenticated { user_info: UserInfo },
}

impl UserMode {
    pub fn is_authenticated(&self) -> bool {
        matches!(self, UserMode::Authenticated { .. })
    }

    pub fn is_guest(&self) -> bool {
        matches!(self, UserMode::Guest)
    }

    pub fn get_user_info(&self) -> Option<&UserInfo> {
        match self {
            UserMode::Authenticated { user_info } => Some(user_info),
            UserMode::Guest => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStateStorage {
    pub user_mode: UserMode,
    pub last_updated: u64,
}

impl UserStateStorage {
    pub fn new() -> Self {
        Self {
            user_mode: UserMode::Guest,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn update_mode(&mut self, mode: UserMode) {
        self.user_mode = mode;
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

#[derive(Debug, Clone)]
pub struct UserManager {
    current_mode: UserMode,
    storage_path: Option<PathBuf>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            current_mode: UserMode::Guest,
            storage_path: None,
        }
    }

    pub fn new_with_storage(app_handle: &tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        // Create app data directory if it doesn't exist
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;

        let storage_path = app_data_dir.join("user_state.json");

        let mut manager = Self {
            current_mode: UserMode::Guest,
            storage_path: Some(storage_path),
        };

        // Try to load existing state
        if let Err(e) = manager.load_state() {
            eprintln!("Warning: Failed to load user state: {}", e);
        }

        Ok(manager)
    }

    pub fn get_current_mode(&self) -> &UserMode {
        &self.current_mode
    }

    pub fn set_guest_mode(&mut self) {
        self.current_mode = UserMode::Guest;
        let _ = self.save_state();
    }

    pub fn set_authenticated_mode(&mut self, user_info: UserInfo) {
        self.current_mode = UserMode::Authenticated { user_info };
        let _ = self.save_state();
    }

    pub fn logout(&mut self) {
        self.current_mode = UserMode::Guest;
        let _ = self.save_state();
    }

    fn load_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let storage_path = self.storage_path.as_ref()
            .ok_or("No storage path configured")?;

        if !storage_path.exists() {
            return Ok(()); // No saved state, use default
        }

        let content = fs::read_to_string(storage_path)
            .map_err(|e| format!("Failed to read user state file: {}", e))?;

        let storage: UserStateStorage = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse user state file: {}", e))?;

        self.current_mode = storage.user_mode;
        Ok(())
    }

    fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let storage_path = self.storage_path.as_ref()
            .ok_or("No storage path configured")?;

        let mut storage = UserStateStorage::new();
        storage.update_mode(self.current_mode.clone());

        let content = serde_json::to_string_pretty(&storage)
            .map_err(|e| format!("Failed to serialize user state: {}", e))?;

        fs::write(storage_path, content)
            .map_err(|e| format!("Failed to write user state file: {}", e))?;

        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.current_mode.is_authenticated()
    }

    pub fn can_access_email_features(&self) -> bool {
        self.current_mode.is_authenticated()
    }

    pub fn get_user_id(&self) -> Option<String> {
        self.current_mode.get_user_info().map(|info| info.user_id.clone())
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to create UserInfo from OAuth response
pub fn create_user_info_from_oauth(
    user_id: String,
    username: String,
    avatar_template: Option<String>,
    email: Option<String>,
) -> UserInfo {
    let login_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Convert avatar_template to avatar_url
    let avatar_url = avatar_template.map(|template| {
        // Linux.do avatar template format: replace {size} with 32
        template.replace("{size}", "32")
    });

    UserInfo {
        user_id,
        username,
        avatar_url,
        email,
        login_time,
    }
}
