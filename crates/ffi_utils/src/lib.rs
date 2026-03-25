pub use parking_lot::Mutex as FfiMutex;

use once_cell::sync::Lazy;
use regex::Regex;

#[macro_export]
macro_rules! lazy_regex {
    ($name:ident, $pat:expr) => {
        static $name: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            Regex::new($pat).expect("invalid regex")
        });
    };
}
