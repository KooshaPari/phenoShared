//! # Workflow Domain Events
//!
//! Events emitted by workflow aggregates.
//!
//! ## Workflow Lifecycle Events
//!
//! ```text
//! WorkflowCreated
//!     ↓
//! WorkflowStarted → WorkflowPaused → WorkflowResumed
//!     ↓
//! WorkflowCompleted / WorkflowFailed
//!     ↓
//! WorkflowCancelled
//! ```
//!
//! ## Stage Events
//!
//! | Event | Triggered When |
//! |-------|----------------|
//! | `StageEntered` | A stage begins execution |
//! | `StageCompleted` | A stage finishes successfully |
//! | `StageFailed` | A stage encounters an error |
//! | `StageSkipped` | A stage is skipped (gating) |
//!
//! ## Design Notes
//!
//! - Workflow events include both high-level lifecycle events
//!   AND granular stage/step events for detailed tracing
//! - Events capture inputs/outputs for reproducibility
//! - Causal chain tracked via `correlation_id`

use crate::value_objects::{WorkflowId, WorkflowName, Priority};

/// Domain event types for workflow aggregates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowEventType {
    Created,
    Started,
    Paused,
    Resumed,
    Completed,
    Failed,
    Cancelled,
    StageEntered,
    StageCompleted,
    StageFailed,
    StageSkipped,
    StepEntered,
    StepCompleted,
    StepFailed,
    PolicyChecked,
    PolicyFailed,
}

/// Event: A new workflow was created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowCreatedEvent {
    pub workflow_id: WorkflowId,
    pub name: WorkflowName,
    pub priority: Priority,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
}

/// Event: Workflow execution started.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowStartedEvent {
    pub workflow_id: WorkflowId,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

/// Event: Workflow completed successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowCompletedEvent {
    pub workflow_id: WorkflowId,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: i64,
    pub stages_completed: u32,
    pub steps_completed: u32,
}

/// Event: Workflow failed with an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowFailedEvent {
    pub workflow_id: WorkflowId,
    pub failed_at: chrono::DateTime<chrono::Utc>,
    pub error_code: String,
    pub error_message: String,
    pub failed_at_stage: Option<String>,
}

/// Event: A workflow stage was entered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageEnteredEvent {
    pub workflow_id: WorkflowId,
    pub stage_name: String,
    pub stage_index: u32,
    pub entered_at: chrono::DateTime<chrono::Utc>,
}

/// Event: A workflow stage completed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageCompletedEvent {
    pub workflow_id: WorkflowId,
    pub stage_name: String,
    pub stage_index: u32,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub outputs: Option<String>,
}

/// Event: A policy was checked during workflow execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyCheckedEvent {
    pub workflow_id: WorkflowId,
    pub policy_id: String,
    pub policy_name: String,
    pub passed: bool,
    pub checked_at: chrono::DateTime<chrono::Utc>,
}
