//! Repository entity for postgres adapter.

use async_trait::async_trait;
use phenotype_port_interfaces::domain::{Entity, Identifier};
use serde::{Deserialize, Serialize};

/// Base repository entity with ID and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryEntity {
    /// Unique identifier
    pub id: String,
    /// Entity type name
    pub entity_type: String,
    /// JSON payload
    pub payload: serde_json::Value,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Version for optimistic locking
    pub version: i64,
}

impl Entity for RepositoryEntity {
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Identifier for RepositoryEntity {
    fn new_id() -> Self::Id {
        uuid::Uuid::new_v4().to_string()
    }
}
