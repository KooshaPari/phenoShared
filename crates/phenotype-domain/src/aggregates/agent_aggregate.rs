//! # Agent Aggregate
//!
//! Agent is the aggregate root for agent management.
//!
//! ## Aggregate Root Responsibilities
//!
//! 1. **Identity** - Holds and validates agent identity
//! 2. **Lifecycle** - Manages agent state transitions
//! 3. **Invariants** - Enforces business rules
//! 4. **Events** - Emits domain events for state changes
//!
//! ## State Machine
//!
//! ```text
//! Registered → Provisioning → Initializing → Ready
//!     ↓             ↓               ↓           ↓
//!   Deleted      Failed          Failed     Degraded
//!                                           ↓
//!                                        Retiring → Retired
//! ```
//!
//! ## Design Notes
//!
//! - Agent is the aggregate root - all access goes through here
//! - State transitions are validated and emit events
//! - Capabilities are anemic value objects with validation
//! - Metadata is a simple key-value store

use crate::errors::{DomainError, ValidationError};
use crate::events::{DomainEvent, EventEnvelope};
use crate::value_objects::{
    AgentId, AgentName, AgentStatus, Priority, TaskId, TaskName,
};

/// Agent aggregate root.
///
/// The Agent is the central entity in agent management. It encapsulates
/// all state and behavior related to an individual agent.
///
/// # Example
///
/// ```rust
/// use phenotype_domain::aggregates::{Agent, Command};
/// use phenotype_domain::value_objects::{AgentId, AgentName, AgentStatus};
///
/// let agent = Agent::new(
///     AgentId::new("agent-001"),
///     AgentName::new("Coding Assistant"),
/// );
///
/// let events = agent.handle_command(Command::Activate)?;
/// # Ok::<(), phenotype_domain::errors::DomainError>(())
/// ```
pub struct Agent {
    /// Unique identifier.
    id: AgentId,
    /// Human-readable name.
    name: AgentName,
    /// Current lifecycle status.
    status: AgentStatus,
    /// Agent capabilities (what it can do).
    capabilities: Vec<Capability>,
    /// Priority level.
    priority: Priority,
    /// Metadata key-value store.
    metadata: std::collections::BTreeMap<String, String>,
    /// Current task assignment.
    current_task: Option<TaskId>,
    /// Uncommitted domain events.
    uncommitted_events: Vec<EventEnvelope>,
}

impl Agent {
    // === Constructor ===
    
    /// Creates a new agent in `Registered` state.
    pub fn new(id: AgentId, name: AgentName) -> Self {
        let agent = Self {
            id,
            name,
            status: AgentStatus::Registered,
            capabilities: Vec::new(),
            priority: Priority::Normal,
            metadata: std::collections::BTreeMap::new(),
            current_task: None,
            uncommitted_events: Vec::new(),
        };
        agent
    }

    /// Reconstructs an agent from an event stream.
    pub fn from_events(events: &[EventEnvelope]) -> Result<Self, DomainError> {
        if events.is_empty() {
            return Err(DomainError::InvalidState(
                "Cannot reconstruct aggregate from empty event stream".into(),
            ));
        }

        let mut agent = Self {
            id: AgentId::default(),
            name: AgentName::default(),
            status: AgentStatus::Registered,
            capabilities: Vec::new(),
            priority: Priority::Normal,
            metadata: std::collections::BTreeMap::new(),
            current_task: None,
            uncommitted_events: Vec::new(),
        };

        for event in events {
            agent.apply(event)?;
        }
        Ok(agent)
    }

    // === Command Handlers ===

    /// Handles a command and returns resulting events.
    pub fn handle_command(&mut self, cmd: Command) -> Result<Vec<EventEnvelope>, DomainError> {
        match cmd {
            Command::Register { id, name } => self.register(id, name),
            Command::Provision { .. } => self.provision(),
            Command::Initialize { capabilities } => self.initialize(capabilities),
            Command::Activate => self.activate(),
            Command::Deactivate => self.deactivate(),
            Command::AssignTask { task_id } => self.assign_task(task_id),
            Command::CompleteTask { .. } => self.complete_task(),
            Command::FailTask { error } => self.fail_task(error),
            Command::Drain => self.drain(),
            Command::Retire => self.retire(),
            Command::Delete => self.delete(),
        }
    }

    fn register(&mut self, id: AgentId, name: AgentName) -> Result<Vec<EventEnvelope>, DomainError> {
        self.id = id;
        self.name = name;
        self.status = AgentStatus::Registered;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::Registered(AgentCreatedEvent {
                agent_id: self.id.clone(),
                name: self.name.clone(),
                registered_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn provision(&mut self) -> Result<Vec<EventEnvelope>, DomainError> {
        self.validate_transition(&AgentStatus::Provisioning)?;
        let old = self.status.clone();
        self.status = AgentStatus::Provisioning;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::ProvisioningStarted(ProvisioningStartedEvent {
                agent_id: self.id.clone(),
                started_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn initialize(&mut self, capabilities: Vec<Capability>) -> Result<Vec<EventEnvelope>, DomainError> {
        self.validate_transition(&AgentStatus::Initializing)?;
        self.capabilities = capabilities;
        let old = self.status.clone();
        self.status = AgentStatus::Initializing;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::Initialized(InitializedEvent {
                agent_id: self.id.clone(),
                initialized_at: chrono::Utc::now(),
                capabilities: self.capabilities.len() as u32,
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn activate(&mut self) -> Result<Vec<EventEnvelope>, DomainError> {
        self.validate_transition(&AgentStatus::Ready)?;
        let old = self.status.clone();
        self.status = AgentStatus::Ready;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::Activated(ActivatedEvent {
                agent_id: self.id.clone(),
                activated_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn deactivate(&mut self) -> Result<Vec<EventEnvelope>, DomainError> {
        if self.current_task.is_some() {
            return Err(DomainError::InvalidOperation(
                "Cannot deactivate while task is assigned".into(),
            ));
        }
        self.validate_transition(&AgentStatus::Draining)?;
        let old = self.status.clone();
        self.status = AgentStatus::Draining;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::Deactivated(DeactivatedEvent {
                agent_id: self.id.clone(),
                deactivated_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn assign_task(&mut self, task_id: TaskId) -> Result<Vec<EventEnvelope>, DomainError> {
        if self.current_task.is_some() {
            return Err(DomainError::InvalidOperation(
                "Task already assigned".into(),
            ));
        }
        if self.status != AgentStatus::Ready && self.status != AgentStatus::Degraded {
            return Err(DomainError::InvalidState(
                format!("Cannot assign task in status {:?}", self.status),
            ));
        }
        self.current_task = Some(task_id.clone());

        let event = EventEnvelope::new(
            crate::events::AgentEvent::TaskAssigned(TaskAssignedEvent {
                agent_id: self.id.clone(),
                task_id,
                assigned_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn complete_task(&mut self) -> Result<Vec<EventEnvelope>, DomainError> {
        let task_id = self.current_task.take()
            .ok_or_else(|| DomainError::InvalidOperation("No task assigned".into()))?;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::TaskCompleted(TaskCompletedEvent {
                agent_id: self.id.clone(),
                task_id,
                completed_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn fail_task(&mut self, error: String) -> Result<Vec<EventEnvelope>, DomainError> {
        let task_id = self.current_task.take()
            .ok_or_else(|| DomainError::InvalidOperation("No task assigned".into()))?;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::TaskFailed(TaskFailedEvent {
                agent_id: self.id.clone(),
                task_id,
                error,
                failed_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn drain(&mut self) -> Result<Vec<EventEnvelope>, DomainError> {
        self.validate_transition(&AgentStatus::Draining)?;
        let old = self.status.clone();
        self.status = AgentStatus::Draining;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::Drained(DrainedEvent {
                agent_id: self.id.clone(),
                drained_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn retire(&mut self) -> Result<Vec<EventEnvelope>, DomainError> {
        if self.current_task.is_some() {
            return Err(DomainError::InvalidOperation(
                "Cannot retire while task is assigned".into(),
            ));
        }
        self.validate_transition(&AgentStatus::Retired)?;
        let old = self.status.clone();
        self.status = AgentStatus::Retired;

        let event = EventEnvelope::new(
            crate::events::AgentEvent::Retired(RetiredEvent {
                agent_id: self.id.clone(),
                retired_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    fn delete(&mut self) -> Result<Vec<EventEnvelope>, DomainError> {
        if self.status != AgentStatus::Registered && self.status != AgentStatus::Retired {
            return Err(DomainError::InvalidState(
                format!("Cannot delete agent in status {:?}", self.status),
            ));
        }

        let event = EventEnvelope::new(
            crate::events::AgentEvent::Deleted(DeletedEvent {
                agent_id: self.id.clone(),
                deleted_at: chrono::Utc::now(),
            }),
            "system",
        );

        self.uncommitted_events.push(event.clone());
        Ok(vec![event])
    }

    // === Event Application ===

    fn apply(&mut self, event: &EventEnvelope) -> Result<(), DomainError> {
        match event.event() {
            crate::events::AgentEvent::Created(e) => {
                self.id = e.agent_id.clone();
                self.name = e.name.clone();
                self.status = AgentStatus::Registered;
            }
            crate::events::AgentEvent::Registered(_) => {
                self.status = AgentStatus::Registered;
            }
            crate::events::AgentEvent::ProvisioningStarted(_) => {
                self.status = AgentStatus::Provisioning;
            }
            crate::events::AgentEvent::Initialized(_) => {
                self.status = AgentStatus::Initializing;
            }
            crate::events::AgentEvent::Activated(_) => {
                self.status = AgentStatus::Ready;
            }
            crate::events::AgentEvent::Deactivated(_) => {
                self.status = AgentStatus::Draining;
            }
            crate::events::AgentEvent::TaskAssigned(e) => {
                self.current_task = Some(e.task_id.clone());
            }
            crate::events::AgentEvent::TaskCompleted(_) | 
            crate::events::AgentEvent::TaskFailed(_) => {
                self.current_task = None;
            }
            crate::events::AgentEvent::Drained(_) => {
                self.status = AgentStatus::Draining;
            }
            crate::events::AgentEvent::Retired(_) => {
                self.status = AgentStatus::Retired;
            }
            crate::events::AgentEvent::Deleted(_) => {
                self.status = AgentStatus::Deleted;
            }
            _ => {}
        }
        Ok(())
    }

    // === Helpers ===

    fn validate_transition(&self, target: &AgentStatus) -> Result<(), DomainError> {
        let allowed = match (&self.status, target) {
            // From Registered
            (AgentStatus::Registered, AgentStatus::Provisioning) => true,
            (AgentStatus::Registered, AgentStatus::Deleted) => true,
            
            // From Provisioning
            (AgentStatus::Provisioning, AgentStatus::Initializing) => true,
            (AgentStatus::Provisioning, AgentStatus::Failed) => true,
            
            // From Initializing
            (AgentStatus::Initializing, AgentStatus::Ready) => true,
            (AgentStatus::Initializing, AgentStatus::Failed) => true,
            
            // From Ready
            (AgentStatus::Ready, AgentStatus::Degraded) => true,
            (AgentStatus::Ready, AgentStatus::Draining) => true,
            
            // From Degraded
            (AgentStatus::Degraded, AgentStatus::Ready) => true,
            (AgentStatus::Degraded, AgentStatus::Draining) => true,
            (AgentStatus::Degraded, AgentStatus::Failed) => true,
            
            // From Draining
            (AgentStatus::Draining, AgentStatus::Ready) => true,
            (AgentStatus::Draining, AgentStatus::Retired) => true,
            
            // From Retired
            (AgentStatus::Retired, AgentStatus::Ready) => true,
            
            // Same state transitions (no-op)
            (a, b) if a == b => true,
            
            _ => false,
        };

        if !allowed {
            return Err(DomainError::InvalidState(format!(
                "Invalid transition from {:?} to {:?}",
                self.status, target
            )));
        }
        Ok(())
    }

    /// Returns uncommitted events and clears them.
    pub fn take_uncommitted_events(&mut self) -> Vec<EventEnvelope> {
        std::mem::take(&mut self.uncommitted_events)
    }

    // === Getters ===

    pub fn id(&self) -> &AgentId { &self.id }
    pub fn name(&self) -> &AgentName { &self.name }
    pub fn status(&self) -> &AgentStatus { &self.status }
    pub fn capabilities(&self) -> &[Capability] { &self.capabilities }
    pub fn priority(&self) -> &Priority { &self.priority }
    pub fn current_task(&self) -> Option<&TaskId> { self.current_task.as_ref() }
}

// === Value Objects ===

/// Agent capability descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    /// Capability identifier.
    pub id: String,
    /// Capability version.
    pub version: String,
    /// Whether the capability is currently active.
    pub enabled: bool,
}

impl Capability {
    pub fn new(id: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            enabled: true,
        }
    }
}

// === Commands ===

/// Commands that can be issued to the Agent aggregate.
#[derive(Debug, Clone)]
pub enum Command {
    Register { id: AgentId, name: AgentName },
    Provision,
    Initialize { capabilities: Vec<Capability> },
    Activate,
    Deactivate,
    AssignTask { task_id: TaskId },
    CompleteTask { output: Option<String> },
    FailTask { error: String },
    Drain,
    Retire,
    Delete,
}
