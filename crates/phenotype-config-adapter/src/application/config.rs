//! Config adapter implementing the Config port.

use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use phenotype_port_interfaces::outbound::config::{Config, ConfigExt, GetStr, GetBool, GetUsize, GetSection};
use crate::error::{AdapterError, Result};

/// Config source enum for priority ordering.
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// Environment variables (highest priority)
    Env,
    /// Command line arguments
    Args,
    /// File-based configuration
    File,
    /// Default values (lowest priority)
    Default,
}

/// Configuration adapter supporting multiple sources with priority.
#[derive(Clone)]
pub struct ConfigAdapter {
    sources: Arc<RwLock<HashMap<String, String>>>,
}

impl ConfigAdapter {
    /// Creates a new empty config adapter.
    pub fn new() -> Self {
        Self {
            sources: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a config adapter with initial values.
    pub fn with_values(values: HashMap<String, String>) -> Self {
        Self {
            sources: Arc::new(RwLock::new(values)),
        }
    }

    /// Loads configuration from environment variables with optional prefix.
    /// E.g., prefix "APP_" will load APP_DATABASE_URL as database_url.
    pub async fn load_env(&self, prefix: Option<&str>) {
        let prefix = prefix.unwrap_or("");
        for (key, value) in std::env::vars() {
            if prefix.is_empty() || key.starts_with(prefix) {
                let config_key = if prefix.is_empty() {
                    key.to_lowercase()
                } else {
                    key[prefix.len()..].trim_start_matches('_').to_lowercase()
                };
                self.sources.write().await.insert(config_key, value);
            }
        }
    }

    /// Loads configuration from a YAML file.
    pub async fn load_yaml(&self, path: &Path) -> Result<()> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| AdapterError::File(format!("Failed to read config file: {}", e)))?;

        let values: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&content)
            .map_err(|e| AdapterError::Parse(format!("Failed to parse YAML: {}", e)))?;

        fn flatten(prefix: &str, map: &serde_yaml::Value, result: &mut HashMap<String, String>) {
            if let Some(obj) = map.as_mapping() {
                for (k, v) in obj {
                    let key = k.as_str().unwrap_or("");
                    let full_key = if prefix.is_empty() {
                        key.to_string()
                    } else {
                        format!("{}_{}", prefix, key)
                    };
                    flatten(&full_key, v, result);
                }
            } else if let Some(s) = map.as_str() {
                result.insert(prefix.to_string(), s.to_string());
            } else if let Some(b) = map.as_bool() {
                result.insert(prefix.to_string(), b.to_string());
            } else if let Some(n) = map.as_i64() {
                result.insert(prefix.to_string(), n.to_string());
            } else if let Some(n) = map.as_f64() {
                result.insert(prefix.to_string(), n.to_string());
            }
        }

        let mut values_out = HashMap::new();
        flatten("", &serde_yaml::Value::Mapping(values.into_iter().collect()), &mut values_out);

        let mut sources = self.sources.write().await;
        sources.extend(values_out);

        Ok(())
    }

    /// Loads configuration from a JSON file.
    pub async fn load_json(&self, path: &Path) -> Result<()> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| AdapterError::File(format!("Failed to read config file: {}", e)))?;

        let values: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| AdapterError::Parse(format!("Failed to parse JSON: {}", e)))?;

        fn flatten(prefix: &str, value: &serde_json::Value, result: &mut HashMap<String, String>) {
            match value {
                serde_json::Value::Object(map) => {
                    for (k, v) in map {
                        let full_key = if prefix.is_empty() {
                            k.clone()
                        } else {
                            format!("{}_{}", prefix, k)
                        };
                        flatten(&full_key, v, result);
                    }
                }
                serde_json::Value::String(s) => {
                    result.insert(prefix.to_string(), s.clone());
                }
                serde_json::Value::Bool(b) => {
                    result.insert(prefix.to_string(), b.to_string());
                }
                serde_json::Value::Number(n) => {
                    result.insert(prefix.to_string(), n.to_string());
                }
                _ => {}
            }
        }

        let mut values_out = HashMap::new();
        flatten("", &values, &mut values_out);

        let mut sources = self.sources.write().await;
        sources.extend(values_out);

        Ok(())
    }

    /// Sets a configuration value.
    pub async fn set(&self, key: &str, value: &str) {
        self.sources.write().await.insert(key.to_string(), value.to_string());
    }

    /// Gets a configuration value by key.
    pub async fn get(&self, key: &str) -> Option<String> {
        self.sources.read().await.get(key).cloned()
    }

    /// Gets all configuration keys.
    pub async fn keys(&self) -> Vec<String> {
        self.sources.read().await.keys().cloned().collect()
    }
}

impl Default for ConfigAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Config for ConfigAdapter {
    type Value = String;

    async fn get(&self, key: &str) -> Result<Option<Self::Value>> {
        Ok(self.get(key).await)
    }

    async fn set(&self, key: &str, value: Self::Value) -> Result<()> {
        self.set(key, &value).await;
        Ok(())
    }
}

#[async_trait]
impl GetStr for ConfigAdapter {
    async fn get_str(&self, key: &str) -> Result<Option<String>> {
        Config::get(self, key).await
    }
}

#[async_trait]
impl GetBool for ConfigAdapter {
    async fn get_bool(&self, key: &str) -> Result<Option<bool>> {
        let value = self.get(key).await;
        Ok(value.map(|v| v.parse::<bool>().unwrap_or(false)))
    }
}

#[async_trait]
impl GetUsize for ConfigAdapter {
    async fn get_usize(&self, key: &str) -> Result<Option<usize>> {
        let value = self.get(key).await;
        Ok(value.and_then(|v| v.parse::<usize>().ok()))
    }
}

#[async_trait]
impl GetSection for ConfigAdapter {
    async fn get_section(&self, prefix: &str) -> Result<HashMap<String, String>> {
        let sources = self.sources.read().await;
        let section: HashMap<String, String> = sources
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(k, v)| (k[prefix.len()..].trim_start_matches('_').to_string(), v.clone()))
            .collect();
        Ok(section)
    }
}

impl ConfigExt for ConfigAdapter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_and_get() {
        let config = ConfigAdapter::new();
        config.set("database_url", "postgres://localhost/test").await;
        
        let value = config.get("database_url").await;
        assert_eq!(value, Some("postgres://localhost/test".to_string()));
    }

    #[tokio::test]
    async fn test_get_section() {
        let config = ConfigAdapter::with_values([
            ("database_host".to_string(), "localhost".to_string()),
            ("database_port".to_string(), "5432".to_string()),
            ("cache_host".to_string(), "redis.local".to_string()),
        ].iter().cloned().collect());

        let db_section = config.get_section("database").await.unwrap();
        assert_eq!(db_section.len(), 2);
        assert_eq!(db_section.get("host"), Some(&"localhost".to_string()));
    }
}
