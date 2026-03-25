//! # Task Entity
//!
//! A Task is a unit of work to be executed by an Agent.

use crate::errors::{DomainError, DomainResult};
use crate::value_objects::{Priority, TaskId, TaskName, TaskStatus};

/// A unit of work to be executed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Task {
    id: TaskId,
    name: TaskName,
    status: TaskStatus,
    priority: Priority,
    assigned_agent_id: Option<String>,
    version: u64,
}

impl Task {
    /// Create a new Task.
    pub fn new(id: TaskId, name: TaskName, priority: Priority) -> DomainResult<Self> {
        Ok(Self {
            id,
            name,
            status: TaskStatus::Pending,
            priority,
            assigned_agent_id: None,
            version: 1,
        })
    }

    /// Reconstruct a Task from stored data.
    pub(crate) fn reconstruct(
        id: TaskId,
        name: TaskName,
        status: TaskStatus,
        priority: Priority,
        assigned_agent_id: Option<String>,
        version: u64,
    ) -> Self {
        Self { id, name, status, priority, assigned_agent_id, version }
    }

    // === Accessors ===

    pub fn id(&self) -> &TaskId { &self.id }
    pub fn name(&self) -> &TaskName { &self.name }
    pub fn status(&self) -> TaskStatus { self.status }
    pub fn priority(&self) -> Priority { self.priority }
    pub fn assigned_agent_id(&self) -> Option<&str> { self.assigned_agent_id.as_deref() }
    pub fn version(&self) -> u64 { self.version }

    // === State Transitions ===

    /// Queue the task for execution.
    pub fn queue(&self) -> DomainResult<Self> {
        self.transition_to(TaskStatus::Queued)
    }

    /// Start executing the task.
    pub fn start(&self) -> DomainResult<Self> {
        self.transition_to(TaskStatus::Running)
    }

    /// Mark the task as completed.
    pub fn complete(&self) -> DomainResult<Self> {
        self.transition_to(TaskStatus::Completed)
    }

    /// Mark the task as failed.
    pub fn fail(&self) -> DomainResult<Self> {
        self.transition_to(TaskStatus::Failed)
    }

    /// Cancel the task.
    pub fn cancel(&self) -> DomainResult<Self> {
        if !self.status.can_cancel() {
            return Err(DomainError::InvalidStateTransition {
                current: format!("{:?}", self.status),
                target: "Cancelled".into(),
                reason: format!("Task in {:?} state cannot be cancelled", self.status),
            });
        }
        self.transition_to(TaskStatus::Cancelled)
    }

    /// Assign the task to an agent.
    pub fn assign_to(&self, agent_id: String) -> DomainResult<Self> {
        if self.assigned_agent_id.is_some() {
            return Err(DomainError::InvariantViolation(
                "Task is already assigned".into(),
            ));
        }
        Ok(Self {
            assigned_agent_id: Some(agent_id),
            ..self.clone()
        })
    }

    fn transition_to(&self, target: TaskStatus) -> DomainResult<Self> {
        if !self.status.can_transition_to(target) {
            return Err(DomainError::InvalidStateTransition {
                current: format!("{:?}", self.status),
                target: format!("{:?}", target),
                reason: format!("Cannot transition from {:?} to {:?}", self.status, target),
            });
        }
        Ok(Self {
            status: target,
            version: self.version + 1,
            ..self.clone()
        })
    }

    // === Queries ===

    pub fn is_terminal(&self) -> bool { self.status.is_terminal() }
    pub fn can_cancel(&self) -> bool { self.status.can_cancel() }
    pub fn is_assigned(&self) -> bool { self.assigned_agent_id.is_some() }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Task({}: {} [{:?}] {:?} v{})",
            self.id, self.name, self.status, self.priority, self.version
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_task() -> Task {
        let id = TaskId::new("task-001").unwrap();
        let name = TaskName::new("Build project").unwrap();
        Task::new(id, name, Priority::Normal).unwrap()
    }

    #[test]
    fn test_task_creation() {
        let task = make_task();
        assert_eq!(task.status(), TaskStatus::Pending);
        assert!(!task.is_terminal());
    }

    #[test]
    fn test_task_lifecycle() {
        let task = make_task();
        let queued = task.queue().unwrap();
        let running = queued.start().unwrap();
        let completed = running.complete().unwrap();
        assert_eq!(completed.status(), TaskStatus::Completed);
        assert!(completed.is_terminal());
    }

    #[test]
    fn test_task_assignment() {
        let task = make_task();
        let assigned = task.assign_to("agent-001".into()).unwrap();
        assert!(assigned.is_assigned());
        assert_eq!(assigned.assigned_agent_id(), Some("agent-001"));
    }

    #[test]
    fn test_task_cancel() {
        let task = make_task();
        let queued = task.queue().unwrap();
        let cancelled = queued.cancel().unwrap();
        assert_eq!(cancelled.status(), TaskStatus::Cancelled);
    }
}
