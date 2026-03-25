//! # Agent Name
//!
//! Human-readable name for an agent.

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Human-readable name for an agent.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AgentName(String);

impl AgentName {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_string();
        if s.is_empty() {
            return Err(ValidationError::new("AgentName", "cannot be empty"));
        }
        if s.len() > 256 {
            return Err(ValidationError::new("AgentName", "cannot exceed 256 characters"));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for AgentName {
    fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        Self::new(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for AgentName {
    fn default() -> Self {
        Self("Unnamed Agent".to_string())
    }
}

impl From<AgentName> for String {
    fn from(name: AgentName) -> Self {
        name.0
    }
}

impl std::fmt::Display for AgentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_name_creation() {
        let name = AgentName::new("Coding Agent").unwrap();
        assert_eq!(name.as_str(), "Coding Agent");
    }

    #[test]
    fn test_agent_name_trimmed() {
        let name = AgentName::new("  Coding Agent  ").unwrap();
        assert_eq!(name.as_str(), "Coding Agent");
    }

    #[test]
    fn test_agent_name_empty_fails() {
        assert!(AgentName::new("").is_err());
    }
}
