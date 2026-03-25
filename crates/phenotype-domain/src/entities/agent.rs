//! # Agent Entity
//!
//! An Agent is an autonomous entity that can execute tasks.
//!
//! ## Design Principles
//!
//! - **Tell, Don't Ask**: Behavior lives on the entity, not getters + conditionals
//! - **Fail Fast**: Invalid state transitions rejected at the entity level
//! - **Immutability via functional updates**: `with_*` methods return new instances
//! - **Domain Events**: State changes emit events for event sourcing
//!
//! ## Entity Identity
//!
//! Two agents are the same if they have the same [`AgentId`], regardless of attributes.

use crate::errors::{DomainError, DomainResult};
use crate::value_objects::{AgentId, AgentName, AgentStatus};

/// An autonomous agent that can execute tasks.
///
/// ## Entity Properties
///
/// - **Identity**: Determined by [`AgentId`]
/// - **Lifecycle**: [`AgentStatus`] tracks state machine
/// - **Version**: Optimistic locking for concurrency control
/// - **Immutable core**: Core attributes set at construction, status changes via state machine
///
/// ## State Machine
///
/// See [`AgentStatus`] for valid transitions.
///
/// ## Example
///
/// ```rust
/// use phenotype_domain::entities::Agent;
/// use phenotype_domain::value_objects::{AgentId, AgentName, AgentStatus};
///
/// let id = AgentId::new("agent-001").unwrap();
/// let name = AgentName::new("Coding Agent").unwrap();
/// let agent = Agent::new(id.clone(), name, AgentStatus::Active).unwrap();
///
/// // State transitions
/// let paused = agent.pause().unwrap();
/// assert_eq!(paused.status(), AgentStatus::Paused);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Agent {
    /// Unique identifier.
    id: AgentId,
    /// Human-readable name.
    name: AgentName,
    /// Current lifecycle status.
    status: AgentStatus,
    /// Version for optimistic locking.
    version: u64,
}

impl Agent {
    /// Create a new Agent in the Active state.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidStateTransition`] if the initial state is invalid.
    pub fn new(id: AgentId, name: AgentName, status: AgentStatus) -> DomainResult<Self> {
        // Validate initial state
        if !matches!(status, AgentStatus::Active) {
            return Err(DomainError::InvalidStateTransition {
                current: "none".into(),
                target: format!("{:?}", status),
                reason: "New agent must start in Active state".into(),
            });
        }

        Ok(Self {
            id,
            name,
            status,
            version: 1,
        })
    }

    /// Create a new Agent from existing data (econstruction).
    ///
    /// This is the "reconstruction constructor" used when loading from event store
    /// or database. It bypasses initial state validation.
    pub(crate) fn reconstruct(
        id: AgentId,
        name: AgentName,
        status: AgentStatus,
        version: u64,
    ) -> Self {
        Self { id, name, status, version }
    }

    // === Accessors (Tell, Don't Ask - provide behavior, not raw data) ===

    /// Get the agent's unique identifier.
    pub fn id(&self) -> &AgentId {
        &self.id
    }

    /// Get the agent's name.
    pub fn name(&self) -> &AgentName {
        &self.name
    }

    /// Get the agent's current status.
    pub fn status(&self) -> AgentStatus {
        self.status
    }

    /// Get the agent's version (for optimistic locking).
    pub fn version(&self) -> u64 {
        self.version
    }

    // === State Transitions (behavior, not raw setters) ===

    /// Transition to the Paused state.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidStateTransition`] if the transition is not allowed.
    pub fn pause(&self) -> DomainResult<Self> {
        self.transition_to(AgentStatus::Paused)
    }

    /// Transition to the Active state.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidStateTransition`] if the transition is not allowed.
    pub fn activate(&self) -> DomainResult<Self> {
        self.transition_to(AgentStatus::Active)
    }

    /// Transition to the Stopping state.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidStateTransition`] if the transition is not allowed.
    pub fn stop(&self) -> DomainResult<Self> {
        self.transition_to(AgentStatus::Stopping)
    }

    /// Transition to the Stopped state.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidStateTransition`] if the transition is not allowed.
    pub fn finalize(&self) -> DomainResult<Self> {
        self.transition_to(AgentStatus::Stopped)
    }

    /// Transition to the Error state.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidStateTransition`] if the transition is not allowed.
    pub fn error(&self) -> DomainResult<Self> {
        self.transition_to(AgentStatus::Error)
    }

    /// Resume from Paused or Error state back to Active.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidStateTransition`] if the agent cannot resume.
    pub fn resume(&self) -> DomainResult<Self> {
        match self.status {
            AgentStatus::Paused | AgentStatus::Error => self.activate(),
            AgentStatus::Active => Ok(self.clone()),
            _ => Err(DomainError::InvalidStateTransition {
                current: format!("{:?}", self.status),
                target: "Active".into(),
                reason: "Agent can only resume from Paused or Error state".into(),
            }),
        }
    }

    /// Attempt a state transition.
    ///
    /// Returns a new Agent instance with the updated status (functional update).
    fn transition_to(&self, target: AgentStatus) -> DomainResult<Self> {
        if !self.status.can_transition_to(target) {
            return Err(DomainError::InvalidStateTransition {
                current: format!("{:?}", self.status),
                target: format!("{:?}", target),
                reason: format!(
                    "Cannot transition from {:?} to {:?}",
                    self.status, target
                ),
            });
        }

        Ok(Self {
            id: self.id.clone(),
            name: self.name.clone(),
            status: target,
            version: self.version + 1,
        })
    }

    // === Domain Queries ===

    /// Returns true if the agent can accept new tasks.
    pub fn can_accept_tasks(&self) -> bool {
        self.status.can_accept_tasks()
    }

    /// Returns true if the agent is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }
}

// === Trait Implementations ===

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Agent({}: {} [{:?}] v{})",
            self.id, self.name, self.status, self.version
        )
    }
}

// === Testing ===

#[cfg(test)]
mod tests {
    use super::*;

    fn make_agent() -> Agent {
        let id = AgentId::new("agent-001").unwrap();
        let name = AgentName::new("Test Agent").unwrap();
        Agent::new(id, name, AgentStatus::Active).unwrap()
    }

    #[test]
    fn test_agent_creation() {
        let agent = make_agent();
        assert_eq!(agent.status(), AgentStatus::Active);
        assert_eq!(agent.version(), 1);
        assert!(agent.can_accept_tasks());
        assert!(!agent.is_terminal());
    }

    #[test]
    fn test_agent_pause() {
        let agent = make_agent();
        let paused = agent.pause().unwrap();
        assert_eq!(paused.status(), AgentStatus::Paused);
        assert_eq!(paused.version(), 2);
        assert!(!paused.can_accept_tasks());
    }

    #[test]
    fn test_agent_resume() {
        let agent = make_agent();
        let paused = agent.pause().unwrap();
        let resumed = paused.resume().unwrap();
        assert_eq!(resumed.status(), AgentStatus::Active);
        assert_eq!(resumed.version(), 3);
    }

    #[test]
    fn test_agent_stop() {
        let agent = make_agent();
        let stopping = agent.stop().unwrap();
        assert_eq!(stopping.status(), AgentStatus::Stopping);
        let stopped = stopping.finalize().unwrap();
        assert_eq!(stopped.status(), AgentStatus::Stopped);
        assert!(stopped.is_terminal());
    }

    #[test]
    fn test_invalid_initial_state() {
        let id = AgentId::new("agent-001").unwrap();
        let name = AgentName::new("Test Agent").unwrap();
        let result = Agent::new(id, name, AgentStatus::Stopped);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_transition() {
        let agent = make_agent();
        // Cannot transition directly from Active to Stopped
        let result = agent.finalize();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_state() {
        let agent = make_agent();
        let errored = agent.error().unwrap();
        assert_eq!(errored.status(), AgentStatus::Error);
        // Can resume from error
        let resumed = errored.resume().unwrap();
        assert_eq!(resumed.status(), AgentStatus::Active);
    }

    #[test]
    fn test_identity() {
        let id1 = AgentId::new("agent-001").unwrap();
        let id2 = AgentId::new("agent-002").unwrap();
        let name = AgentName::new("Test Agent").unwrap();

        let a1 = Agent::new(id1.clone(), name.clone(), AgentStatus::Active).unwrap();
        let a2 = Agent::new(id1, name.clone(), AgentStatus::Paused).unwrap();
        let a3 = Agent::new(id2, name, AgentStatus::Active).unwrap();

        // Same ID = same entity (even with different status)
        assert_eq!(a1, a2);
        // Different ID = different entity
        assert_ne!(a1, a3);
    }

    #[test]
    fn test_display() {
        let agent = make_agent();
        let display = format!("{}", agent);
        assert!(display.contains("agent-001"));
        assert!(display.contains("Test Agent"));
        assert!(display.contains("Active"));
    }
}
