//! Data models for nanovms client.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistics for a sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxStats {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub memory_limit_mb: u64,
    pub disk_usage_mb: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub process_count: u32,
}

/// Log entry from a sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Resource limits for a sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_quota: Option<f64>,
    pub memory_limit_mb: Option<u64>,
    pub disk_limit_mb: Option<u64>,
    pub pids_limit: Option<u64>,
}

/// Security profile for a sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProfile {
    pub seccomp_enabled: bool,
    pub apparmor_profile: Option<String>,
    pub no_new_privileges: bool,
    pub read_only_rootfs: bool,
}

impl Default for SecurityProfile {
    fn default() -> Self {
        Self {
            seccomp_enabled: true,
            apparmor_profile: None,
            no_new_privileges: true,
            read_only_rootfs: false,
        }
    }
}

/// Build configuration for unikernels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Base image or package
    pub base: String,
    /// Entry point binary
    pub entry_point: Option<String>,
    /// Arguments to pass to entry point
    pub args: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Files to include
    pub files: Vec<String>,
    /// Directories to include
    pub directories: Vec<String>,
}

impl BuildConfig {
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            entry_point: None,
            args: Vec::new(),
            env: HashMap::new(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
}

/// Information about a built image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub name: String,
    pub size_bytes: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tier: super::Tier,
}

/// Event types for sandbox lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxEvent {
    Created { sandbox_id: String },
    Started { sandbox_id: String },
    Stopped { sandbox_id: String, exit_code: i32 },
    Deleted { sandbox_id: String },
    Error { sandbox_id: String, error: String },
}
