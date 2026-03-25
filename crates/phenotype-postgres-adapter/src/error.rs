//! # Error Types

/// Error type for Postgres adapter operations.
#[derive(Debug)]
pub enum PostgresError {
    Connection(String),
    Query(String),
    NotFound(String),
    AlreadyExists(String),
    Serialization(String),
    Pool(String),
}

impl std::fmt::Display for PostgresError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresError::Connection(msg) => write!(f, "Connection error: {}", msg),
            PostgresError::Query(msg) => write!(f, "Query error: {}", msg),
            PostgresError::NotFound(msg) => write!(f, "Not found: {}", msg),
            PostgresError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            PostgresError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            PostgresError::Pool(msg) => write!(f, "Pool error: {}", msg),
        }
    }
}

impl std::error::Error for PostgresError {}

impl From<serde_json::Error> for PostgresError {
    fn from(e: serde_json::Error) -> Self {
        PostgresError::Serialization(e.to_string())
    }
}
