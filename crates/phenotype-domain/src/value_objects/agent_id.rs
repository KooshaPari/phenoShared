//! # Agent ID
//!
//! Unique identifier for an agent.
//!
//! ## Design
//!
//! - Non-empty string
//! - Normalized to lowercase
//! - Validated at construction

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Unique identifier for an agent.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentId(String);

impl AgentId {
    /// Creates a new AgentId, validating the input.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_lowercase();
        if s.is_empty() {
            return Err(ValidationError::new("AgentId", "cannot be empty"));
        }
        if s.len() > 128 {
            return Err(ValidationError::new("AgentId", "cannot exceed 128 characters"));
        }
        Ok(Self(s))
    }

    /// Creates without validation (for deserialization/reconstruction).
    pub fn from_unchecked(value: String) -> Self {
        Self(value.to_lowercase())
    }

    /// Returns the string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for AgentId {
    fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        Self::new(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self("agent-000".to_string())
    }
}

impl From<AgentId> for String {
    fn from(id: AgentId) -> Self {
        id.0
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id_creation() {
        let id = AgentId::new("AGENT-001").unwrap();
        assert_eq!(id.as_str(), "agent-001"); // normalized
    }

    #[test]
    fn test_agent_id_empty_fails() {
        assert!(AgentId::new("").is_err());
        assert!(AgentId::new("   ").is_err());
    }

    #[test]
    fn test_agent_id_too_long() {
        let long = "a".repeat(129);
        assert!(AgentId::new(long).is_err());
    }

    #[test]
    fn test_agent_id_equality() {
        let id1 = AgentId::new("agent-001").unwrap();
        let id2 = AgentId::new("AGENT-001").unwrap();
        assert_eq!(id1, id2); // Case insensitive equality
    }
}
