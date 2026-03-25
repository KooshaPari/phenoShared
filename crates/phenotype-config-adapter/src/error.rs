//! Error types for the config adapter.

use thiserror::Error;
use phenotype_port_interfaces::error::PortError;

/// Adapter-specific error type.
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Port error: {0}")]
    Port(#[from] PortError),
}

/// Result type alias for adapter operations.
pub type Result<T> = std::result::Result<T, AdapterError>;
