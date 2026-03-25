//! # Error Types

/// Error type for Redis adapter operations.
#[derive(Debug)]
pub enum RedisError {
    Connection(String),
    Query(String),
    NotFound(String),
    Serialization(String),
    Pool(String),
}

impl std::fmt::Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisError::Connection(msg) => write!(f, "Connection error: {}", msg),
            RedisError::Query(msg) => write!(f, "Query error: {}", msg),
            RedisError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RedisError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            RedisError::Pool(msg) => write!(f, "Pool error: {}", msg),
        }
    }
}

impl std::error::Error for RedisError {}

impl From<serde_json::Error> for RedisError {
    fn from(e: serde_json::Error) -> Self {
        RedisError::Serialization(e.to_string())
    }
}
