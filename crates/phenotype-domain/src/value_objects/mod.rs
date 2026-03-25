//! # Value Objects
//!
//! Value Objects are immutable objects defined by their attributes rather than
//! a unique identity. Two value objects with the same attributes are equal.
//!
//! ## Design Principles
//!
//! | Principle | Description |
//! |-----------|-------------|
//! | **Immutability** | Value objects never change after creation |
//! | **Value Equality** | Equal by attribute value, not identity |
//! | **Self-Validation** | Validated at construction time |
//! | **Replace Instead of Modify** | Return new instance on change |
//!
//! ## When to Use Value Objects
//!
//! Use a Value Object when:
//! - It represents a concept defined by its attributes
//! - It is immutable
//! - It is compared by value, not identity
//! - It forms part of an Entity or Aggregate
//!
//! ## vs Entities
//!
//! | Value Object | Entity |
//! |--------------|--------|
//! | No identity | Has unique identity |
//! | Immutable | Mutable |
//! | Equal by value | Equal by identity |
//! | Discarded after use | Long-lived |
//!
//! ## Examples
//!
//! ```rust
//! use phenotype_domain::value_objects::{AgentId, Priority};
//!
//! let id1 = AgentId::new("agent-001");
//! let id2 = AgentId::new("agent-001");
//! assert_eq!(id1, id2); // Equal by value
//!
//! let priority = Priority::High;
//! assert_eq!(priority.as_i32(), 3);
//! ```

// === Public API ===

pub use self::agent_id::AgentId;
pub use self::agent_name::AgentName;
pub use self::agent_status::AgentStatus;
pub use self::task_id::TaskId;
pub use self::task_name::TaskName;
pub use self::task_status::TaskStatus;
pub use self::workflow_id::WorkflowId;
pub use self::workflow_name::WorkflowName;
pub use self::policy_id::PolicyId;
pub use self::priority::Priority;
pub use self::timestamp::Timestamp;

mod agent_id;
mod agent_name;
mod agent_status;
mod task_id;
mod task_name;
mod task_status;
mod workflow_id;
mod workflow_name;
mod policy_id;
mod priority;
mod timestamp;

// === Shared Trait ===

/// Base trait for all value objects.
///
/// Value objects are:
/// - Immutable
/// - Validated at construction
/// - Compared by value equality
pub trait ValueObject: Sized + Eq + Clone {
    /// Validates and creates a new instance.
    fn new(value: impl Into<String>) -> Result<Self, crate::errors::ValidationError>;

    /// Returns the inner string value.
    fn as_str(&self) -> &str;

    /// Returns the string representation.
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
