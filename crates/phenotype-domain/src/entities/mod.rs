//! # Entities
//!
//! Entities are objects defined by a unique identity rather than their attributes.
//! Unlike Value Objects, two entities with the same attributes but different IDs
//! are NOT equal.
//!
//! ## Entity vs Value Object
//!
//! | Aspect | Entity | Value Object |
//! |--------|--------|--------------|
//! | Identity | Has unique ID | No identity |
//! | Equality | By ID | By value |
//! | Mutability | Usually mutable | Immutable |
//! | Lifespan | Long-lived | Short-lived |
//!
//! ## Design Principles
//!
//! | Principle | Description |
//! |-----------|-------------|
//! | **Identity First** | Entity is defined by its ID |
//! | **Encapsulation** | State access through methods |
//! | **Defensive Copy** | Return copies, not references |
//!
//! ## Module Structure
//!
//! ```text
//! entities/
//! ├── mod.rs          # Public API
//! ├── agent.rs         # Agent entity
//! └── task.rs         # Task entity
//! ```

pub use self::agent::*;
pub use self::task::*;

mod agent;
mod task;
