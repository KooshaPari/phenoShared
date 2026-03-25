//! # Task ID
//!
//! Unique identifier for a task.

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Unique identifier for a task.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(String);

impl TaskId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_lowercase();
        if s.is_empty() {
            return Err(ValidationError::new("TaskId", "cannot be empty"));
        }
        if s.len() > 128 {
            return Err(ValidationError::new("TaskId", "cannot exceed 128 characters"));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for TaskId {
    fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        Self::new(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self("task-000".to_string())
    }
}

impl From<TaskId> for String {
    fn from(id: TaskId) -> Self {
        id.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_id_creation() {
        let id = TaskId::new("TASK-001").unwrap();
        assert_eq!(id.as_str(), "task-001");
    }

    #[test]
    fn test_task_id_empty_fails() {
        assert!(TaskId::new("").is_err());
    }
}
