use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use tauri::Manager;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

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

// 文件操作队列项
#[derive(Debug)]
enum FileOperation {
    UpdateBanStatus { id: String, ban_status: Option<String> },
    UpdatePortalInfo { id: String, portal_info: Option<PortalInfo> },
    UpdateToken { id: String, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String> },
    AddToken { id: String, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String> },
    RemoveToken { id: String },
}

// 文件操作队列管理器
#[derive(Clone)]
pub struct FileOperationQueue {
    queue: Arc<Mutex<VecDeque<FileOperation>>>,
    is_processing: Arc<Mutex<bool>>,
}

impl FileOperationQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            is_processing: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn enqueue_and_process(&self, operation: FileOperation, token_manager: &TokenManager) -> Result<bool, Box<dyn std::error::Error>> {
        // 将操作加入队列
        {
            let mut queue = self.queue.lock().unwrap();
            queue.push_back(operation);
        }

        // 如果当前没有在处理，开始处理队列
        {
            let mut is_processing = self.is_processing.lock().unwrap();
            if *is_processing {
                return Ok(true); // 已经有其他线程在处理
            }
            *is_processing = true;
        }

        // 处理队列中的所有操作
        let result = self.process_queue(token_manager).await;

        // 标记处理完成
        {
            let mut is_processing = self.is_processing.lock().unwrap();
            *is_processing = false;
        }

        result
    }

    async fn process_queue(&self, token_manager: &TokenManager) -> Result<bool, Box<dyn std::error::Error>> {
        loop {
            let operation = {
                let mut queue = self.queue.lock().unwrap();
                queue.pop_front()
            };

            match operation {
                Some(op) => {
                    if let Err(e) = self.execute_operation(op, token_manager).await {
                        eprintln!("Failed to execute file operation: {}", e);
                        return Err(e);
                    }
                }
                None => break, // 队列为空
            }
        }
        Ok(true)
    }

    async fn execute_operation(&self, operation: FileOperation, token_manager: &TokenManager) -> Result<bool, Box<dyn std::error::Error>> {
        // 加载当前状态
        let mut storage = token_manager.load_tokens_direct()?;
        let mut needs_save = false;

        match operation {
            FileOperation::UpdateBanStatus { id, ban_status } => {
                if storage.update_token_ban_status(&id, ban_status) {
                    needs_save = true;
                }
            }
            FileOperation::UpdatePortalInfo { id, portal_info } => {
                if storage.update_token_portal_info(&id, portal_info) {
                    needs_save = true;
                }
            }
            FileOperation::UpdateToken { id, tenant_url, access_token, portal_url, email_note } => {
                if storage.update_token_with_details(&id, tenant_url, access_token, portal_url, email_note) {
                    needs_save = true;
                }
            }
            FileOperation::AddToken { id, tenant_url, access_token, portal_url, email_note } => {
                let token = StoredToken::new_with_details(tenant_url, access_token, portal_url, email_note);
                // 使用预生成的ID
                let mut token = token;
                token.id = id;
                storage.tokens.push(token);
                needs_save = true;
            }
            FileOperation::RemoveToken { id } => {
                if storage.remove_token(&id) {
                    needs_save = true;
                }
            }
        }

        if needs_save {
            token_manager.save_tokens_direct(&storage)?;
        }

        Ok(true)
    }
}

#[derive(Clone)]
pub struct TokenManager {
    storage_path: PathBuf,
    operation_queue: FileOperationQueue,
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

        Ok(Self {
            storage_path,
            operation_queue: FileOperationQueue::new(),
        })
    }

    // 直接文件操作方法（用于队列处理）
    pub fn load_tokens_direct(&self) -> Result<TokenStorage, Box<dyn std::error::Error>> {
        if !self.storage_path.exists() {
            return Ok(TokenStorage::new());
        }

        let content = fs::read_to_string(&self.storage_path)
            .map_err(|e| format!("Failed to read tokens file: {}", e))?;

        let storage: TokenStorage = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse tokens file: {}", e))?;

        Ok(storage)
    }

    pub fn save_tokens_direct(&self, storage: &TokenStorage) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(storage)
            .map_err(|e| format!("Failed to serialize tokens: {}", e))?;

        fs::write(&self.storage_path, content)
            .map_err(|e| format!("Failed to write tokens file: {}", e))?;

        Ok(())
    }

    // 公共接口方法（只读操作）
    pub fn load_tokens(&self) -> Result<TokenStorage, Box<dyn std::error::Error>> {
        self.load_tokens_direct()
    }

    pub async fn add_token(&self, tenant_url: String, access_token: String) -> Result<String, Box<dyn std::error::Error>> {
        // 生成ID（在队列处理前）
        let id = uuid::Uuid::new_v4().to_string();

        let operation = FileOperation::AddToken {
            id: id.clone(),
            tenant_url,
            access_token,
            portal_url: None,
            email_note: None,
        };

        self.operation_queue.enqueue_and_process(operation, self).await?;
        Ok(id)
    }

    pub async fn add_token_with_portal(&self, tenant_url: String, access_token: String, portal_url: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        // 生成ID（在队列处理前）
        let id = uuid::Uuid::new_v4().to_string();

        let operation = FileOperation::AddToken {
            id: id.clone(),
            tenant_url,
            access_token,
            portal_url,
            email_note: None,
        };

        self.operation_queue.enqueue_and_process(operation, self).await?;
        Ok(id)
    }

    pub async fn add_token_with_details(&self, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        // 生成ID（在队列处理前）
        let id = uuid::Uuid::new_v4().to_string();

        let operation = FileOperation::AddToken {
            id: id.clone(),
            tenant_url,
            access_token,
            portal_url,
            email_note,
        };

        self.operation_queue.enqueue_and_process(operation, self).await?;
        Ok(id)
    }

    pub async fn remove_token(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let operation = FileOperation::RemoveToken {
            id: id.to_string(),
        };

        self.operation_queue.enqueue_and_process(operation, self).await
    }

    pub fn get_all_tokens(&self) -> Result<Vec<StoredToken>, Box<dyn std::error::Error>> {
        let storage = self.load_tokens()?;
        Ok(storage.tokens)
    }

    pub async fn update_token(&self, id: &str, tenant_url: String, access_token: String, portal_url: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let operation = FileOperation::UpdateToken {
            id: id.to_string(),
            tenant_url,
            access_token,
            portal_url,
            email_note: None,
        };

        self.operation_queue.enqueue_and_process(operation, self).await
    }

    pub async fn update_token_with_details(&self, id: &str, tenant_url: String, access_token: String, portal_url: Option<String>, email_note: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let operation = FileOperation::UpdateToken {
            id: id.to_string(),
            tenant_url,
            access_token,
            portal_url,
            email_note,
        };

        self.operation_queue.enqueue_and_process(operation, self).await
    }

    pub async fn update_token_ban_status(&self, id: &str, ban_status: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let operation = FileOperation::UpdateBanStatus {
            id: id.to_string(),
            ban_status,
        };

        self.operation_queue.enqueue_and_process(operation, self).await
    }

    pub async fn update_token_portal_info(&self, id: &str, portal_info: Option<PortalInfo>) -> Result<bool, Box<dyn std::error::Error>> {
        let operation = FileOperation::UpdatePortalInfo {
            id: id.to_string(),
            portal_info,
        };

        self.operation_queue.enqueue_and_process(operation, self).await
    }
}
