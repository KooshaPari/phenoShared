//! # Aggregates
//!
//! Aggregates are the primary consistency boundary in DDD.
//!
//! ## Aggregate Pattern
//!
//! An aggregate is a cluster of related objects that are treated as a unit
//! for data changes. External objects reference the aggregate root only.
//!
//! ## Design Principles
//!
//! | Principle | Description |
//! |-----------|-------------|
//! | **Single Root** | Only aggregate root is directly accessible |
//! | **Consistency Boundary** | Invariants enforced within aggregate |
//! | **Event Emission** | State changes emit domain events |
//! | **Command/Query Separation** | Commands modify, queries read |
//! | **Immutability** | Value objects are immutable |
//!
//! ## Module Structure
//!
//! ```text
//! aggregates/
//! ├── mod.rs          # Public API
//! ├── agent_aggregate.rs   # Agent aggregate root
//! ├── task_aggregate.rs   # Task aggregate root
//! └── workflow_aggregate.rs # Workflow aggregate root
//! ```
//!
//! ## Agent Aggregate
//!
//! ```text
//! AgentAggregate (Root)
//! ├── AgentId
//! ├── AgentName
//! ├── AgentStatus
//! ├── capabilities: Vec<Capability>
//! ├── metadata: BTreeMap<String, String>
//! └── events: Vec<DomainEvent>
//! ```

pub use self::agent_aggregate::*;
pub use self::task_aggregate::*;

mod agent_aggregate;
mod task_aggregate;
