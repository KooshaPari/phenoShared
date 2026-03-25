//! # Error Types

/// Error type for HTTP adapter operations.
#[derive(Debug)]
pub enum HttpError {
    Request(String),
    Response(String),
    Serialization(String),
    Timeout(String),
    Network(String),
    Status { code: u16, body: String },
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::Request(msg) => write!(f, "Request error: {}", msg),
            HttpError::Response(msg) => write!(f, "Response error: {}", msg),
            HttpError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            HttpError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            HttpError::Network(msg) => write!(f, "Network error: {}", msg),
            HttpError::Status { code, body } => write!(f, "Status {}: {}", code, body),
        }
    }
}

impl std::error::Error for HttpError {}

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            HttpError::Timeout(e.to_string())
        } else if e.is_connect() {
            HttpError::Network(e.to_string())
        } else {
            HttpError::Request(e.to_string())
        }
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(e: serde_json::Error) -> Self {
        HttpError::Serialization(e.to_string())
    }
}
