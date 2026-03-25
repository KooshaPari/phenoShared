//! # Phenotype Redis Adapter
//!
//! Redis adapter for hexagonal architecture.

pub mod error;
pub mod redis_cache;
pub mod redis_config;

pub use error::RedisError;
pub use redis_cache::RedisCache;
pub use redis_config::RedisConfig;

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_exists() {
        assert!(true);
    }
}
