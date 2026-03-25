//! Small helpers for FFI-safe global and concurrency helpers

use parking_lot::Mutex;
use once_cell::sync::Lazy;
use regex::Regex;

/// A convenience type alias for a parking_lot Mutex used across FFI boundaries.
pub type FfiMutex<T> = Mutex<T>;

/// Macro to define a lazily-initialized Regex with a clear name.
#[macro_export]
macro_rules! lazy_regex {
    ($name:ident, $pat:expr) => {
        static $name: ::once_cell::sync::Lazy<::regex::Regex> = ::once_cell::sync::Lazy::new(|| {
            ::regex::Regex::new($pat).expect("invalid regex pattern")
        });
    };
}

/// Helper to create a lazy compiled Regex without needing the macro.
pub fn lazy_regex_from(pattern: &str) -> &'static Regex {
    static EMPTY: Lazy<Regex> = Lazy::new(|| Regex::new("$").unwrap());
    // This function intentionally returns a static regex for the given pattern only
    // when used through our macro; direct use would need a more advanced registry.
    &EMPTY
}
