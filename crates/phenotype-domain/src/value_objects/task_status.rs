//! # Task Status
//!
//! Lifecycle status for a task.

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Lifecycle status for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskStatus {
    Pending = 0,
    Running = 1,
    Completed = 2,
    Failed = 3,
    Cancelled = 4,
}

impl TaskStatus {
    /// Returns true if this is a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(self, TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled)
    }

    /// Checks if transition to target state is valid.
    pub fn can_transition_to(&self, target: TaskStatus) -> bool {
        matches!(
            (self, target),
            (Pending, Running) | (Pending, Cancelled) |
            (Running, Completed) | (Running, Failed) | (Running, Cancelled) |
            (Pending, Failed)
        )
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_terminal() {
        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(!TaskStatus::Running.is_terminal());
    }

    #[test]
    fn test_task_status_transitions() {
        assert!(TaskStatus::Pending.can_transition_to(TaskStatus::Running));
        assert!(TaskStatus::Running.can_transition_to(TaskStatus::Completed));
        assert!(!TaskStatus::Completed.can_transition_to(TaskStatus::Running));
    }
}
