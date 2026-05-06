//! # PostgreSQL Repository
//!
//! Simple PostgreSQL repository implementation.

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

use crate::error::PostgresError;
use crate::postgres_config::PostgresConfig;

/// Validates that a table name is safe for use in SQL statements.
/// Only allows lowercase ASCII letters, digits, and underscores.
/// Returns the name if valid, or an error if it contains dangerous characters.
fn validate_table_name(name: &str) -> Result<&str, PostgresError> {
    if name.is_empty() || name.len() > 63 {
        return Err(PostgresError::Query(format!(
            "Invalid table name '{}': length must be 1-63",
            name
        )));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(PostgresError::Query(format!(
            "Invalid table name '{}': must contain only lowercase letters, digits, and underscores",
            name
        )));
    }
    // Must not start with a digit
    if name.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        return Err(PostgresError::Query(format!(
            "Invalid table name '{}': must not start with a digit",
            name
        )));
    }
    // Reserved words
    let reserved = [
        "select", "insert", "update", "delete", "drop", "create", "alter",
        "table", "from", "where", "union", "order", "group", "having",
    ];
    if reserved.contains(&name) {
        return Err(PostgresError::Query(format!(
            "Invalid table name '{}': reserved SQL keyword",
            name
        )));
    }
    Ok(name)
}

/// PostgreSQL repository for storing entities.
#[derive(Clone)]
pub struct PostgresRepository {
    pool: Pool,
    table_name: String,
}

impl PostgresRepository {
    /// Create a new PostgresRepository with the given pool.
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            table_name: "entities".to_string(),
        }
    }

    /// Create with a custom table name. Name is validated for safety.
    pub fn with_table(pool: Pool, table_name: impl Into<String>) -> Result<Self, PostgresError> {
        let name = table_name.into();
        validate_table_name(&name)?;
        Ok(Self {
            pool,
            table_name: name,
        })
    }

    /// Create a connection pool from config.
    pub async fn from_config(config: &PostgresConfig) -> Result<Self, PostgresError> {
        let pool = create_pool(config).await?;
        Ok(Self::new(pool))
    }

    /// Get the connection pool.
    pub fn pool(&self) -> &Pool {
        &self.pool
    }

    /// Get the table name.
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// Initialize the database schema.
    pub async fn initialize(&self) -> Result<(), PostgresError> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| PostgresError::Pool(e.to_string()))?;

        let query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                data JSONB NOT NULL,
                version INTEGER NOT NULL DEFAULT 1,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
            self.table_name
        );

        client
            .execute(&query, &[])
            .await
            .map_err(|e| PostgresError::Query(e.to_string()))?;

        Ok(())
    }
}

/// Create a connection pool from config.
pub async fn create_pool(config: &PostgresConfig) -> Result<Pool, PostgresError> {
    let mut cfg = Config::new();
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.user = Some(config.user.clone());
    cfg.password = Some(config.password.clone());
    cfg.dbname = Some(config.database.clone());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| PostgresError::Pool(e.to_string()))
}
