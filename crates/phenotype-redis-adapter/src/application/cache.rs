//! Redis cache adapter implementing the Cache port.

use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use std::time::Duration;
use phenotype_port_interfaces::outbound::cache::{
    Cache, Get, Set, Delete, Exists, GetTtl, SetEx, CacheExt,
};
use crate::error::{AdapterError, Result};

/// Redis cache configuration.
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: usize,
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".into(),
            pool_size: 16,
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(3),
            write_timeout: Duration::from_secs(3),
        }
    }
}

/// Redis connection pool type.
pub type RedisPool = redis::aio::ConnectionManager;

/// Creates a new Redis connection manager from config.
pub async fn create_pool(config: RedisConfig) -> Result<RedisPool> {
    let client = Client::open(config.url.as_str())
        .map_err(|e| AdapterError::Connection(e.to_string()))?;
    
    let manager = client
        .get_tokio_connection_manager()
        .await
        .map_err(|e| AdapterError::Connection(e.to_string()))?;

    Ok(manager)
}

/// Redis cache adapter.
#[derive(Clone)]
pub struct RedisCache {
    pool: RedisPool,
    default_ttl: Option<Duration>,
}

impl RedisCache {
    /// Creates a new RedisCache with the given connection pool.
    pub fn new(pool: RedisPool) -> Self {
        Self { pool, default_ttl: None }
    }

    /// Creates a new RedisCache with a default TTL for all keys.
    pub fn with_default_ttl(pool: RedisPool, ttl: Duration) -> Self {
        Self { pool, default_ttl: Some(ttl) }
    }

    /// Gets a mutable reference to the connection.
    fn pool_mut(&mut self) -> &mut RedisPool {
        &mut self.pool
    }
}

#[async_trait]
impl Cache for RedisCache {
    type Value = Vec<u8>;

    async fn get(&self, key: &str) -> Result<Option<Self::Value>> {
        let mut conn = self.pool.clone();
        let result: Option<Vec<u8>> = conn.get(key).await?;
        Ok(result)
    }

    async fn set(&self, key: &str, value: Self::Value) -> Result<()> {
        let mut conn = self.pool.clone();
        conn.set(key, value).await?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        let mut conn = self.pool.clone();
        let deleted: i64 = conn.del(key).await?;
        Ok(deleted > 0)
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.pool.clone();
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }
}

#[async_trait]
impl Get for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Cache::get(self, key).await
    }
}

#[async_trait]
impl Set for RedisCache {
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<()> {
        Cache::set(self, key, value).await
    }
}

#[async_trait]
impl Delete for RedisCache {
    async fn delete(&self, key: &str) -> Result<bool> {
        Cache::delete(self, key).await
    }
}

#[async_trait]
impl Exists for RedisCache {
    async fn exists(&self, key: &str) -> Result<bool> {
        Cache::exists(self, key).await
    }
}

#[async_trait]
impl GetTtl for RedisCache {
    async fn get_ttl(&self, key: &str) -> Result<Option<Duration>> {
        let mut conn = self.pool.clone();
        let ttl: i64 = conn.ttl(key).await?;
        if ttl < 0 {
            Ok(None) // Key doesn't exist or has no TTL
        } else {
            Ok(Some(Duration::from_secs(ttl as u64)))
        }
    }
}

#[async_trait]
impl SetEx for RedisCache {
    async fn set_ex(&self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<()> {
        let mut conn = self.pool.clone();
        conn.set_ex(key, value, ttl.as_secs()).await?;
        Ok(())
    }
}

impl CacheExt for RedisCache {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RedisConfig::default();
        assert_eq!(config.url, "redis://127.0.0.1:6379");
        assert_eq!(config.pool_size, 16);
    }
}
