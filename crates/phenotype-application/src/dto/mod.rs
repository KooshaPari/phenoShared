//!
//! # Data Transfer Objects (DTOs)
//!
//! DTOs are plain data structures used to transfer data between layers.
//! Following **Clean Architecture**, DTOs are used to:
//! - Decouple domain entities from API responses
//! - Hide internal structure from external consumers
//! - Shape data for specific use cases
//!
//! ## Naming Convention
//!
//! DTOs follow the pattern: `Noun+Response` or `Noun+Request`:
//! - `AgentResponse` - DTO for agent data
//! - `TaskListResponse` - DTO for paginated task list
//! - `CreateAgentRequest` - DTO for create command input

use serde::{Deserialize, Serialize};

/// Agent DTO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDto {
    pub id: String,
    pub name: String,
    pub status: String,
    pub capabilities: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Task DTO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub assigned_agent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

/// Workflow DTO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDto {
    pub id: String,
    pub name: String,
    pub status: String,
    pub current_step: u32,
    pub total_steps: u32,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

/// Paginated list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
    pub has_more: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: u64, offset: u64, limit: u64) -> Self {
        Self {
            has_more: offset + limit < total,
            data,
            total,
            offset,
            limit,
        }
    }
}

/// Agent metrics DTO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetricsDto {
    pub agent_id: String,
    pub tasks_completed: u64,
    pub tasks_in_progress: u64,
    pub avg_task_duration_ms: u64,
    pub utilization_percent: f64,
}

/// System health DTO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthDto {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub agents_online: u64,
    pub agents_busy: u64,
    pub tasks_pending: u64,
    pub tasks_in_progress: u64,
}

/// System metrics DTO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricsDto {
    pub timestamp: String,
    pub total_agents: u64,
    pub total_tasks: u64,
    pub tasks_completed_24h: u64,
    pub tasks_failed_24h: u64,
    pub avg_task_duration_ms: u64,
    pub throughput_tasks_per_minute: f64,
}

/// Command result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub event: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    pub fn ok(data: T, event: Option<serde_json::Value>) -> Self {
        Self {
            success: true,
            data: Some(data),
            event,
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            event: None,
            error: Some(error),
        }
    }
}

/// Query result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub cached: bool,
    pub error: Option<String>,
}

impl<T> QueryResult<T> {
    pub fn ok(data: T, cached: bool) -> Self {
        Self {
            success: true,
            data: Some(data),
            cached,
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            cached: false,
            error: Some(error),
        }
    }
}
