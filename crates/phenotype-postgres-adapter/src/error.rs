//! Error types for the postgres adapter.

use thiserror::Error;
use phenotype_port_interfaces::error::PortError;

/// Adapter-specific error type.
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Postgres error: {0}")]
<<<<<<< HEAD
    Postgres(#[from] tokio_postgres::Error),

    #[error("Pool error: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),
=======
    Postgres(String),

    #[error("Pool error: {0}")]
    Pool(String),
>>>>>>> main

    #[error("Serialization error: {0}")]
    Serialization(String),

<<<<<<< HEAD
    #[error("Deserialization error: {0}")]
    Deserialization(String),

=======
>>>>>>> main
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

<<<<<<< HEAD
    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Configuration error: {0}")]
    Config(String),

=======
>>>>>>> main
    #[error("Port error: {0}")]
    Port(#[from] PortError),
}

<<<<<<< HEAD
impl From<tokio_postgres::Error> for AdapterError {
    fn from(e: tokio_postgres::Error) -> Self {
        use tokio_postgres::Error as PgError;
        if e.to_string().contains("duplicate key") {
            AdapterError::AlreadyExists(e.to_string())
        } else if e.to_string().contains("no rows") {
            AdapterError::NotFound(e.to_string())
        } else {
            AdapterError::Postgres(e)
        }
    }
}

impl From<serde_json::Error> for AdapterError {
    fn from(e: serde_json::Error) -> Self {
        AdapterError::Serialization(e.to_string())
    }
}

=======
>>>>>>> main
/// Result type alias for adapter operations.
pub type Result<T> = std::result::Result<T, AdapterError>;
