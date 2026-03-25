//! # Phenotype HTTP Adapter
//!
//! Simple HTTP client adapter using `reqwest`.

pub mod error;
pub mod http_client;

pub use error::HttpError;
pub use http_client::{HttpResponse, ReqwestHttpClient};

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_exists() {
        assert!(true);
    }
}
