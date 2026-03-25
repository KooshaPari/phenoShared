//!
//! # Queries Module
//!
//! Queries represent **read operations** that do NOT modify state.
//! Following **CQRS**, queries are separated from commands.
//!
//! ## Query Properties
//!
//! - **Side-effect free**: Queries never modify state
//! - **Optimized for reads**: Can use read replicas, caches
//! - **Return DTOs**: Never return domain entities directly
//! - **Can be composed**: Multiple queries can be combined
//!
//! ## Query vs Command
//!
//! | Aspect | Query | Command |
//! |--------|-------|---------|
//! | Side effects | None | Yes |
//! | Return type | DTO | Events + Result |
//! | Caching | Yes | No |
//! | Concurrency | No locks | Optimistic/pessimistic locking |

use serde::{Deserialize, Serialize};

/// Base query trait.
pub trait Query: Send + Sync {
    fn query_type(&self) -> &'static str;
}

// =================================================================================================
// AGENT QUERIES
// =================================================================================================

/// Query to get a single agent by ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgent {
    /// Agent ID.
    pub agent_id: String,
}

impl Query for GetAgent {
    fn query_type(&self) -> &'static str {
        "GetAgent"
    }
}

/// Query to list all agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgents {
    /// Optional status filter.
    pub status_filter: Option<String>,

    /// Pagination offset.
    pub offset: Option<u64>,

    /// Pagination limit.
    pub limit: Option<u64>,
}

impl Query for ListAgents {
    fn query_type(&self) -> &'static str {
        "ListAgents"
    }
}

/// Query to search agents by capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAgentsByCapability {
    /// Capability to search for.
    pub capability: String,
}

impl Query for SearchAgentsByCapability {
    fn query_type(&self) -> &'static str {
        "SearchAgentsByCapability"
    }
}

/// Query to get agent metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentMetrics {
    /// Agent ID.
    pub agent_id: String,
}

impl Query for GetAgentMetrics {
    fn query_type(&self) -> &'static str {
        "GetAgentMetrics"
    }
}

// =================================================================================================
// TASK QUERIES
// =================================================================================================

/// Query to get a single task by ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTask {
    /// Task ID.
    pub task_id: String,
}

impl Query for GetTask {
    fn query_type(&self) -> &'static str {
        "GetTask"
    }
}

/// Query to list tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasks {
    /// Status filter.
    pub status_filter: Option<String>,

    /// Agent ID filter (tasks for specific agent).
    pub agent_id_filter: Option<String>,

    /// Pagination offset.
    pub offset: Option<u64>,

    /// Pagination limit.
    pub limit: Option<u64>,
}

impl Query for ListTasks {
    fn query_type(&self) -> &'static str {
        "ListTasks"
    }
}

/// Query to get task metrics/analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskMetrics {
    /// Time window (e.g., "24h", "7d", "30d").
    pub time_window: Option<String>,
}

impl Query for GetTaskMetrics {
    fn query_type(&self) -> &'static str {
        "GetTaskMetrics"
    }
}

/// Query to list tasks assigned to an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksByAgent {
    /// Agent ID.
    pub agent_id: String,
}

impl Query for ListTasksByAgent {
    fn query_type(&self) -> &'static str {
        "ListTasksByAgent"
    }
}

// =================================================================================================
// WORKFLOW QUERIES
// =================================================================================================

/// Query to get a workflow by ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorkflow {
    /// Workflow ID.
    pub workflow_id: String,
}

impl Query for GetWorkflow {
    fn query_type(&self) -> &'static str {
        "GetWorkflow"
    }
}

/// Query to list workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkflows {
    /// Status filter.
    pub status_filter: Option<String>,

    /// Pagination offset.
    pub offset: Option<u64>,

    /// Pagination limit.
    pub limit: Option<u64>,
}

impl Query for ListWorkflows {
    fn query_type(&self) -> &'static str {
        "ListWorkflows"
    }
}

// =================================================================================================
// SYSTEM QUERIES
// =================================================================================================

/// Query to get system health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemHealth {}

impl Query for GetSystemHealth {
    fn query_type(&self) -> &'static str {
        "GetSystemHealth"
    }
}

/// Query to get system metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemMetrics {}

impl Query for GetSystemMetrics {
    fn query_type(&self) -> &'static str {
        "GetSystemMetrics"
    }
}
