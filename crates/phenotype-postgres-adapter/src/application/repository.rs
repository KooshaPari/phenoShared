//! Repository service implementation.

use async_trait::async_trait;
use deadpool_postgres::{Pool, Manager, ManagerConfig, RecyclingMethod};
use tokio_postgres::NoTls;
use phenotype_port_interfaces::outbound::repository::{
    Repository, Entity, FindById, Save, Delete, Exists,
    FindAll, FindByField, Count, RepositoryExt,
};
use crate::error::{AdapterError, Result};
use crate::domain::entities::RepositoryEntity;

/// Postgres repository configuration.
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_size: usize,
    pub min_idle: Option<usize>,
    pub connect_timeout_secs: u64,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 5432,
            user: "postgres".into(),
            password: "postgres".into(),
            database: "phenotype".into(),
            max_size: 16,
            min_idle: None,
            connect_timeout_secs: 30,
        }
    }
}

impl From<PostgresConfig> for tokio_postgres::Config {
    fn from(config: PostgresConfig) -> Self {
        let mut cfg = tokio_postgres::Config::new();
        cfg.host(&config.host);
        cfg.port(config.port);
        cfg.user(&config.user);
        cfg.password(&config.password);
        cfg.dbname(&config.database);
        cfg.connect_timeout(std::time::Duration::from_secs(config.connect_timeout_secs));
        cfg
    }
}

/// Postgres connection pool.
pub type PostgresPool = deadpool_postgres::Pool;

/// Creates a new postgres connection pool from config.
pub async fn create_pool(config: PostgresConfig) -> Result<PostgresPool> {
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host(&config.host);
    pg_config.port(config.port);
    pg_config.user(&config.user);
    pg_config.password(&config.password);
    pg_config.dbname(&config.database);

    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let manager = Manager::from_config(pg_config, NoTls, manager_config);

    let pool = deadpool_postgres::Pool::builder(manager)
        .max_size(config.max_size)
        .build()
        .map_err(|e| AdapterError::Pool(e))?;

    Ok(pool)
}

/// Postgres repository adapter.
#[derive(Clone)]
pub struct PostgresRepository {
    pool: PostgresPool,
    table_name: String,
}

impl PostgresRepository {
    /// Creates a new PostgresRepository with the given pool and table name.
    pub fn new(pool: PostgresPool, table_name: impl Into<String>) -> Self {
        Self {
            pool,
            table_name: table_name.into(),
        }
    }

    /// Creates a new PostgresRepository with default table name "entities".
    pub fn with_default_table(pool: PostgresPool) -> Self {
        Self::new(pool, "entities")
    }

    /// Gets a connection from the pool.
    async fn get_client(&self) -> Result<deadpool_postgres::Client> {
        self.pool.get().await.map_err(AdapterError::Pool)
    }

    /// Initializes the repository schema.
    pub async fn initialize(&self) -> Result<()> {
        let client = self.get_client().await?;
        
        let query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                payload JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                version BIGINT NOT NULL DEFAULT 1
            )
            "#,
            self.table_name
        );
        
        client.execute(&query, &[]).await?;
        
        // Create index on entity_type for faster lookups
        let index_query = format!(
            "CREATE INDEX IF NOT EXISTS {}_entity_type_idx ON {}(entity_type)",
            self.table_name, self.table_name
        );
        client.execute(&index_query, &[]).await?;
        
        Ok(())
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    type Entity = RepositoryEntity;
    type Id = String;

    async fn save(&self, entity: &Self::Entity) -> Result<()> {
        let client = self.get_client().await?;
        
        let query = format!(
            r#"
            INSERT INTO {} (id, entity_type, payload, created_at, updated_at, version)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                payload = EXCLUDED.payload,
                updated_at = EXCLUDED.updated_at,
                version = EXCLUDED.version + 1
            "#,
            self.table_name
        );

        client.execute(
            &query,
            &[
                &entity.id,
                &entity.entity_type,
                &entity.payload,
                &entity.created_at,
                &entity.updated_at,
                &entity.version,
            ],
        ).await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Entity>> {
        let client = self.get_client().await?;
        
        let query = format!(
            "SELECT id, entity_type, payload, created_at, updated_at, version FROM {} WHERE id = $1",
            self.table_name
        );

        let row = client.query_opt(&query, &[&id]).await?;
        
        match row {
            Some(row) => Ok(Some(RepositoryEntity {
                id: row.get("id"),
                entity_type: row.get("entity_type"),
                payload: row.get("payload"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                version: row.get("version"),
            })),
            None => Ok(None),
        }
    }

    async fn delete(&self, id: &Self::Id) -> Result<bool> {
        let client = self.get_client().await?;
        
        let query = format!("DELETE FROM {} WHERE id = $1", self.table_name);
        let result = client.execute(&query, &[&id]).await?;

        Ok(result > 0)
    }
}

#[async_trait]
impl Exists<PostgresRepository> for PostgresRepository {
    async fn exists(&self, id: &Self::Id) -> Result<bool> {
        let client = self.get_client().await?;
        
        let query = format!(
            "SELECT 1 FROM {} WHERE id = $1 LIMIT 1",
            self.table_name
        );

        let row = client.query_opt(&query, &[&id]).await?;
        Ok(row.is_some())
    }
}

#[async_trait]
impl FindAll<PostgresRepository> for PostgresRepository {
    async fn find_all(&self, limit: usize, offset: usize) -> Result<Vec<Self::Entity>> {
        let client = self.get_client().await?;
        
        let query = format!(
            "SELECT id, entity_type, payload, created_at, updated_at, version FROM {} ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            self.table_name
        );

        let rows = client.query(&query, &[&(limit as i64), &(offset as i64)]).await?;

        Ok(rows.iter().map(|row| RepositoryEntity {
            id: row.get("id"),
            entity_type: row.get("entity_type"),
            payload: row.get("payload"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            version: row.get("version"),
        }).collect())
    }
}

#[async_trait]
impl FindByField<PostgresRepository> for PostgresRepository {
    async fn find_by_field(&self, field: &str, value: &str) -> Result<Vec<Self::Entity>> {
        let client = self.get_client().await?;
        
        let query = format!(
            "SELECT id, entity_type, payload, created_at, updated_at, version FROM {} WHERE {} = $1",
            self.table_name, field
        );

        let rows = client.query(&query, &[&value]).await?;

        Ok(rows.iter().map(|row| RepositoryEntity {
            id: row.get("id"),
            entity_type: row.get("entity_type"),
            payload: row.get("payload"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            version: row.get("version"),
        }).collect())
    }
}

#[async_trait]
impl Count<PostgresRepository> for PostgresRepository {
    async fn count(&self) -> Result<i64> {
        let client = self.get_client().await?;
        
        let query = format!("SELECT COUNT(*) FROM {}", self.table_name);
        let row = client.query_one(&query, &[]).await?;

        Ok(row.get(0))
    }
}

/// Extension trait for Repository with additional helper methods.
#[async_trait]
impl<T: Repository> RepositoryExt<T> for PostgresRepository where Self: FindById<T> + Save<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PostgresConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
    }
}
