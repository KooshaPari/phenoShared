//! Error types for the redis adapter.

use thiserror::Error;
use phenotype_port_interfaces::error::PortError;

/// Adapter-specific error type.
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Port error: {0}")]
    Port(#[from] PortError),
}

impl From<serde_json::Error> for AdapterError {
    fn from(e: serde_json::Error) -> Self {
        AdapterError::Serialization(e.to_string())
    }
}

/// Result type alias for adapter operations.
pub type Result<T> = std::result::Result<T, AdapterError>;
