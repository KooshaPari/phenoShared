//! # Timestamp
//!
//! Timestamp value object using Unix milliseconds.
//!
//! ## Zero External Dependencies
//!
//! Instead of using chrono, we use Unix milliseconds (i64).
//! This allows integration with any time library in adapters.
//!
//! ## Usage in Adapters
//!
//! ```rust
//! // In domain (no time library)
//! let ts = Timestamp::now();
//!
//! // In Rust adapter using chrono
//! let domain_ts = Timestamp::from_chrono(chrono::Utc::now());
//!
//! // In TypeScript adapter
//! const ts = Date.now();
//!
//! // In Go adapter
//! ts := time.Now().UnixMilli()
//! ```

use crate::errors::ValidationError;

/// Timestamp as Unix milliseconds since epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Creates a timestamp for the current time.
    pub fn now() -> Self {
        Self(current_time_millis())
    }

    /// Creates from Unix milliseconds.
    pub fn from_millis(ms: i64) -> Self {
        Self(ms)
    }

    /// Returns Unix milliseconds.
    pub fn as_i64(self) -> i64 {
        self.0
    }

    /// Creates from seconds.
    pub fn from_secs(secs: i64) -> Self {
        Self(secs * 1000)
    }

    /// Returns seconds since epoch.
    pub fn as_secs(self) -> i64 {
        self.0 / 1000
    }

    /// Creates an invalid/zero timestamp.
    pub fn zero() -> Self {
        Self(0)
    }

    /// Checks if this is zero.
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

// Platform-specific time implementation
#[cfg(not(feature = "std"))]
fn current_time_millis() -> i64 {
    // Embedded/no_std: return a fixed value for compilation
    0
}

#[cfg(feature = "std")]
fn current_time_millis() -> i64 {
    // When std is available, we use a simple approach
    // Adapters should use their own time libraries
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

impl From<i64> for Timestamp {
    fn from(ms: i64) -> Self {
        Self(ms)
    }
}

impl From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
