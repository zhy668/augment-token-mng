use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::{NoTls, Error as PgError};
use std::sync::Arc;
use super::config::DatabaseConfig;

pub type DbPool = Pool;

#[derive(Debug)]
pub struct DatabaseManager {
    pool: Option<Arc<DbPool>>,
    config: DatabaseConfig,
}

impl DatabaseManager {
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            pool: None,
            config,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enabled {
            return Ok(());
        }

        let mut cfg = Config::new();
        cfg.host = Some(self.config.host.clone());
        cfg.port = Some(self.config.port);
        cfg.dbname = Some(self.config.database.clone());
        cfg.user = Some(self.config.username.clone());
        cfg.password = Some(self.config.password.clone());

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        
        // 测试连接
        let client = pool.get().await?;
        client.simple_query("SELECT 1").await?;
        
        self.pool = Some(Arc::new(pool));
        Ok(())
    }

    pub fn get_pool(&self) -> Option<Arc<DbPool>> {
        self.pool.clone()
    }

    pub fn is_connected(&self) -> bool {
        self.pool.is_some()
    }

    pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(pool) = &self.pool {
            let client = pool.get().await?;
            client.simple_query("SELECT 1").await?;
            Ok(())
        } else {
            Err("Database not connected".into())
        }
    }

    pub async fn close(&mut self) {
        if let Some(pool) = self.pool.take() {
            // deadpool会自动处理连接的关闭
            drop(pool);
        }
    }
}

pub async fn test_database_connection(config: &DatabaseConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut cfg = Config::new();
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database.clone());
    cfg.user = Some(config.username.clone());
    cfg.password = Some(config.password.clone());

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    let client = pool.get().await?;
    client.simple_query("SELECT 1").await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_manager_creation() {
        let config = DatabaseConfig::default();
        let manager = DatabaseManager::new(config);
        assert!(!manager.is_connected());
        assert!(manager.get_pool().is_none());
    }
}
