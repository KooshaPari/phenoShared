//! # Task Name
//!
//! Human-readable name for a task.

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Human-readable name for a task.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskName(String);

impl TaskName {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_string();
        if s.is_empty() {
            return Err(ValidationError::new("TaskName", "cannot be empty"));
        }
        if s.len() > 256 {
            return Err(ValidationError::new("TaskName", "cannot exceed 256 characters"));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for TaskName {
    fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        Self::new(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TaskName {
    fn default() -> Self {
        Self("Unnamed Task".to_string())
    }
}

impl From<TaskName> for String {
    fn from(name: TaskName) -> Self {
        name.0
    }
}

impl std::fmt::Display for TaskName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_name_creation() {
        let name = TaskName::new("Build Project").unwrap();
        assert_eq!(name.as_str(), "Build Project");
    }

    #[test]
    fn test_task_name_empty_fails() {
        assert!(TaskName::new("").is_err());
    }
}
