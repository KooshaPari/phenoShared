//! # Aggregates
//!
//! Aggregates are clusters of related entities and value objects
//! that form a consistency boundary.
//!
//! ## Principles
//!
//! - **Single root**: Each aggregate has one root entity
//! - **Consistency**: Invariants enforced within aggregate boundary
//! - **Transactional**: Changes are atomic within aggregate
//! - **Event sourceable**: State changes produce domain events

// Re-export for convenience
pub use crate::entities::Agent;
