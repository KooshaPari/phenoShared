//! # Domain Events
//!
//! Immutable facts that happened in the domain.
//!
//! ## Event Sourcing Pattern
//!
//! In event sourcing, the state of an aggregate is reconstructed by replaying
//! all domain events. This provides:
//!
//! | Benefit | Description |
//! |----------|-------------|
//! | **Audit Trail** | Complete history of all state changes |
//! | **Temporal Queries** | Query state at any point in time |
//! | **Event Replay** | Rebuild state from scratch |
//! | **Causal Ordering** | Events capture causality |
//!
//! ## Event Structure
//!
//! Each event contains:
//! - **What** happened (event type)
//! - **When** it happened (timestamp)
//! - **Who** triggered it (actor)
//! - **What changed** (event data)
//!
//! ## Design Principles
//!
//! | Principle | Application |
//! |-----------|-------------|
//! | **Immutability** | Events cannot be modified after creation |
//! | **Append-only** | Events are never updated or deleted |
//! | **Tell, Don't Ask** | Events describe facts, not queries |
//! | **Fail Fast** | Invalid events rejected at construction |
//! | **DRY** | Shared event envelope fields in one place |
//!
//! ## Module Structure
//!
//! ```text
//! events/
//! ├── mod.rs                  # Public API, trait definitions
//! ├── agent_events.rs          # Agent lifecycle events
//! ├── task_events.rs           # Task lifecycle events
//! └── workflow_events.rs       # Workflow lifecycle events
//! ```
//!
//! ## Example Event
//!
//! ```rust
//! use phenotype_domain::events::{DomainEvent, EventEnvelope};
//!
//! let event = EventEnvelope::new(
//!     AgentCreatedEvent {
//!         agent_id: "agent-001".into(),
//!         name: "Coding Agent".into(),
//!         created_at: chrono::Utc::now(),
//!     },
//!     "system",
//! );
//!
//! // Events are immutable
//! assert_eq!(event.actor(), "system");
//! ```

// === Public API ===

pub use self::agent_events::*;
pub use self::task_events::*;
pub use self::workflow_events::*;

mod agent_events;
mod task_events;
mod workflow_events;
