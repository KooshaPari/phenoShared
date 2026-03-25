//! # Agent Status
//!
//! Lifecycle status for an agent.

/// Lifecycle status for an agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AgentStatus {
    /// Agent is registered but not yet active.
    Registered = 0,
    /// Agent is being provisioned.
    Provisioning = 1,
    /// Agent is initializing.
    Initializing = 2,
    /// Agent is active and ready.
    Active = 3,
    /// Agent is paused.
    Paused = 4,
    /// Agent is stopping.
    Stopping = 5,
    /// Agent has stopped.
    Stopped = 6,
    /// Agent encountered an error.
    Error = 7,
}

impl AgentStatus {
    /// Returns true if the agent can accept tasks.
    pub fn can_accept_tasks(&self) -> bool {
        matches!(self, AgentStatus::Active)
    }

    /// Returns true if this is a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(self, AgentStatus::Stopped)
    }

    /// Checks if transition to target state is valid.
    pub fn can_transition_to(&self, target: AgentStatus) -> bool {
        matches!(
            (self, target),
            (Active, Paused) | (Active, Stopping) |
            (Paused, Active) | (Paused, Stopping) |
            (Stopping, Stopped) |
            (Error, Active) | (Error, Stopping)
        )
    }
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self::Registered
    }
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_can_accept_tasks() {
        assert!(AgentStatus::Active.can_accept_tasks());
        assert!(!AgentStatus::Paused.can_accept_tasks());
    }

    #[test]
    fn test_terminal_states() {
        assert!(AgentStatus::Stopped.is_terminal());
        assert!(!AgentStatus::Active.is_terminal());
    }

    #[test]
    fn test_valid_transitions() {
        assert!(AgentStatus::Active.can_transition_to(AgentStatus::Paused));
        assert!(AgentStatus::Paused.can_transition_to(AgentStatus::Active));
        assert!(!AgentStatus::Active.can_transition_to(AgentStatus::Stopped));
    }
}
