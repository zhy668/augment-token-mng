use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub id: String,
    pub tenant_url: String,
    pub access_token: String,
    pub created_at: DateTime<Utc>,
}

impl StoredToken {
    pub fn new(tenant_url: String, access_token: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_url,
            access_token,
            created_at: Utc::now(),
        }
    }

    /// Calculate remaining days (14 days from creation)
    pub fn remaining_days(&self) -> i64 {
        let expiry_date = self.created_at + chrono::Duration::days(14);
        let now = Utc::now();
        let remaining = expiry_date - now;
        remaining.num_days().max(0)
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        self.remaining_days() <= 0
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

    pub fn remove_token(&mut self, id: &str) -> bool {
        let initial_len = self.tokens.len();
        self.tokens.retain(|token| token.id != id);
        self.tokens.len() < initial_len
    }



    /// Remove expired tokens
    pub fn cleanup_expired(&mut self) -> usize {
        let initial_len = self.tokens.len();
        self.tokens.retain(|token| !token.is_expired());
        initial_len - self.tokens.len()
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

    pub fn cleanup_expired_tokens(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let mut storage = self.load_tokens()?;
        let removed_count = storage.cleanup_expired();
        if removed_count > 0 {
            self.save_tokens(&storage)?;
        }
        Ok(removed_count)
    }
}
