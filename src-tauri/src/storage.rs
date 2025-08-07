use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalInfo {
    pub credits_balance: i64,
    pub expiry_date: String,
    pub is_active: bool,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub id: String,
    pub tenant_url: String,
    pub access_token: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_info: Option<PortalInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_note: Option<String>,
}

impl StoredToken {
    pub fn new(tenant_url: String, access_token: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_url,
            access_token,
            created_at: Utc::now(),
            portal_url: None,
            ban_status: None,
            portal_info: None,
            email_note: None,
        }
    }

    pub fn new_with_portal(tenant_url: String, access_token: String, portal_url: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_url,
            access_token,
            created_at: Utc::now(),
            portal_url,
            ban_status: None,
            portal_info: None,
            email_note: None,
        }
    }

    pub fn new_with_details(tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_url,
            access_token,
            created_at: Utc::now(),
            portal_url,
            ban_status: None,
            portal_info: None,
            email_note,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStorage {
    pub tokens: Vec<StoredToken>,
}

impl TokenStorage {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn add_token(&mut self, tenant_url: String, access_token: String) -> String {
        let token = StoredToken::new(tenant_url, access_token);
        let id = token.id.clone();
        self.tokens.push(token);
        id
    }

    /// Add a new token with portal URL
    pub fn add_token_with_portal(&mut self, tenant_url: String, access_token: String, portal_url: Option<String>) -> String {
        let token = StoredToken::new_with_portal(tenant_url, access_token, portal_url);
        let id = token.id.clone();
        self.tokens.push(token);
        id
    }

    /// Add a new token with all details
    pub fn add_token_with_details(&mut self, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String>) -> String {
        let token = StoredToken::new_with_details(tenant_url, access_token, portal_url, email_note);
        let id = token.id.clone();
        self.tokens.push(token);
        id
    }

    pub fn remove_token(&mut self, id: &str) -> bool {
        let initial_len = self.tokens.len();
        self.tokens.retain(|token| token.id != id);
        self.tokens.len() < initial_len
    }

    /// Update an existing token
    pub fn update_token(&mut self, id: &str, tenant_url: String, access_token: String, portal_url: Option<String>) -> bool {
        if let Some(token) = self.tokens.iter_mut().find(|token| token.id == id) {
            token.tenant_url = tenant_url;
            token.access_token = access_token;
            token.portal_url = portal_url;
            true
        } else {
            false
        }
    }

    /// Update an existing token with all details
    pub fn update_token_with_details(&mut self, id: &str, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String>) -> bool {
        if let Some(token) = self.tokens.iter_mut().find(|token| token.id == id) {
            token.tenant_url = tenant_url;
            token.access_token = access_token;
            token.portal_url = portal_url;
            token.email_note = email_note;
            true
        } else {
            false
        }
    }

    /// Update token ban status
    pub fn update_token_ban_status(&mut self, id: &str, ban_status: Option<String>) -> bool {
        if let Some(token) = self.tokens.iter_mut().find(|token| token.id == id) {
            token.ban_status = ban_status;
            true
        } else {
            false
        }
    }

    /// Update token portal info
    pub fn update_token_portal_info(&mut self, id: &str, portal_info: Option<PortalInfo>) -> bool {
        if let Some(token) = self.tokens.iter_mut().find(|token| token.id == id) {
            token.portal_info = portal_info;
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TokenManager {
    storage_path: PathBuf,
}

impl TokenManager {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;
        
        // Create app data directory if it doesn't exist
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
        
        let storage_path = app_data_dir.join("tokens.json");
        
        Ok(Self { storage_path })
    }

    pub fn load_tokens(&self) -> Result<TokenStorage, Box<dyn std::error::Error>> {
        if !self.storage_path.exists() {
            return Ok(TokenStorage::new());
        }

        let content = fs::read_to_string(&self.storage_path)
            .map_err(|e| format!("Failed to read tokens file: {}", e))?;
        
        let storage: TokenStorage = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse tokens file: {}", e))?;
        
        Ok(storage)
    }

    pub fn save_tokens(&self, storage: &TokenStorage) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(storage)
            .map_err(|e| format!("Failed to serialize tokens: {}", e))?;
        
        fs::write(&self.storage_path, content)
            .map_err(|e| format!("Failed to write tokens file: {}", e))?;
        
        Ok(())
    }

    pub fn add_token(&self, tenant_url: String, access_token: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let id = storage.add_token(tenant_url, access_token);
        self.save_tokens(&storage)?;
        Ok(id)
    }

    pub fn add_token_with_portal(&self, tenant_url: String, access_token: String, portal_url: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let id = storage.add_token_with_portal(tenant_url, access_token, portal_url);
        self.save_tokens(&storage)?;
        Ok(id)
    }

    pub fn add_token_with_details(&self, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let id = storage.add_token_with_details(tenant_url, access_token, portal_url, email_note);
        self.save_tokens(&storage)?;
        Ok(id)
    }

    pub fn remove_token(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let removed = storage.remove_token(id);
        if removed {
            self.save_tokens(&storage)?;
        }
        Ok(removed)
    }

    pub fn get_all_tokens(&self) -> Result<Vec<StoredToken>, Box<dyn std::error::Error>> {
        let storage = self.load_tokens()?;
        Ok(storage.tokens)
    }

    pub fn update_token(&self, id: &str, tenant_url: String, access_token: String, portal_url: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let updated = storage.update_token(id, tenant_url, access_token, portal_url);
        if updated {
            self.save_tokens(&storage)?;
        }
        Ok(updated)
    }

    pub fn update_token_with_details(&self, id: &str, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let updated = storage.update_token_with_details(id, tenant_url, access_token, portal_url, email_note);
        if updated {
            self.save_tokens(&storage)?;
        }
        Ok(updated)
    }

    pub fn update_token_ban_status(&self, id: &str, ban_status: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let updated = storage.update_token_ban_status(id, ban_status);
        if updated {
            self.save_tokens(&storage)?;
        }
        Ok(updated)
    }

    pub fn update_token_portal_info(&self, id: &str, portal_info: Option<PortalInfo>) -> Result<bool, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let updated = storage.update_token_portal_info(id, portal_info);
        if updated {
            self.save_tokens(&storage)?;
        }
        Ok(updated)
    }
}
