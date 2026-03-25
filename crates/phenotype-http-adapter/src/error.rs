//! Error types for the HTTP adapter.

use thiserror::Error;
use phenotype_port_interfaces::error::PortError;

/// Adapter-specific error type.
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Response error: status={0}, body={1}")]
    Response(u16, String),

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
