//! # Humaniser
//!
//! A small, ergonomic Rust crate to convert numbers, sizes, durations, times, percentages, and permissions
//! into human-readable formats.
//!
//! ## Quick Links
//! - [`HumanCount`]: Convert large numbers into K, M, B, T, etc.
//! - [`HumanSize`]: Convert bytes into KiB, MiB, GiB, etc.
//! - [`HumanDuration`]: Show elapsed time since a timestamp in human-readable format
//! - [`HumanTime`]: Format a `Duration` as H:M:S
//! - [`HumanPercent`]: Round and format floating-point numbers as percentages
//!
//! ## Output formats
//!
//! Each type provides `.concise()`, a formatting method that returns a concise version of the output:
//! ```rust
//! use humaniser::HumanCount;
//!
//! fn main() {
//! // This will print "1.8 thousand"
//! println!("{}", HumanCount::from(1_800).to_string());
//!
//! // This will print "1.8K"
//! println!("{}", HumanCount::from(1_800).concise());
//! }
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use humaniser::*;
//!
//! // HumanCount
//! assert_eq!(HumanCount::from(1_200).concise(), "1.2K");
//! assert_eq!(HumanCount::from(1_200).to_string(), "1.2 thousand");
//!
//! # HumanSize
//!
//! Default is **binary (IEC)** (1024-based):
//! ```rust
//! use humaniser::HumanSize;
//!
//! assert_eq!(HumanSize::from(5_242_880).concise(), "5 MiB");       // Binary concise
//! assert_eq!(HumanSize::from(5_242_880).full(), "5 mebibytes");    // Binary full
//! ```
//!
//! You can also explicitly choose **decimal (SI)** (1000-based):
//! ```rust
//! use humaniser::{HumanSize, UnitSystem};
//!
//! let hs = HumanSize::from_with_system(5_000_000, UnitSystem::Decimal);
//! assert_eq!(hs.concise(), "5 MB");       // Decimal concise
//! assert_eq!(hs.full(), "5 megabytes");   // Decimal full
//! ```
//!
//! // HumanDuration
//! ```rust
//! use std::time::{Duration, SystemTime};
//! let now = SystemTime::now();
//! let result = HumanDuration::from(Some(now - Duration::from_secs(75))).concise();
//! assert!(result.contains("1m"));
//!
//! // HumanTime
//! assert_eq!(HumanTime::from(Duration::from_secs(3661)).concise(), "1h 1m 1s");
//! assert_eq!(HumanTime::from(Duration::from_secs(3661)).to_string(), "1 hour 1 minute 1 second");
//!
//! // HumanPercent
//! assert_eq!(HumanPercent::from(12.3456, 1).concise(), "12.3%");
//! assert_eq!(HumanPercent::from(12.3456, 1).to_string(), "12.3 percent");
//! ```
//!
//! ## Goals
//!
//! - Provide a simple, consistent API to make numeric and system values readable.
//! - Cross-platform support for permissions (Unix and Windows).
//! - Small, dependency-light, and ergonomic for CLI, web, or general Rust applications.
//!
//! ## Platform-specific behavior
//!
//! - **Unix**: Permissions use `unix_mode` crate to show `rwx` strings with file type prefix.
//! - **Windows**: Permissions are translated into descriptive text like `Read`, `Write`, `Execute`.
//!
//! ## Crate modules
//!
//! - [`HumanCount`] — Convert numbers to readable short format (1K, 1M, 1B…).
//! - [`HumanSize`] — Convert bytes to human-readable units (KiB, MiB…).
//! - [`HumanDuration`] — Show how long ago a timestamp occurred in short or long format.
//! - [`HumanTime`] — Convert `Duration` into H:M:S strings.
//! - [`HumanPercent`] — Round floats and display as percentage string.
//!
//! [`HumanCount`]: struct.HumanCount.html
//! [`HumanSize`]: struct.HumanSize.html
//! [`HumanDuration`]: struct.HumanDuration.html
//! [`HumanTime`]: struct.HumanTime.html
//! [`HumanPercent`]: struct.HumanPercent.html

mod core;
pub use core::HumanCount;
pub use core::HumanDuration;
pub use core::HumanPercent;
pub use core::HumanSize;
pub use core::HumanTime;
pub use core::UnitSystem;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::core::{HumanCount, HumanDuration, HumanPercent, HumanSize, HumanTime};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_human_count() {
        assert_eq!(HumanCount::from(1_700_700).to_string(), "1.7 million");
        assert_eq!(HumanCount::from(500).to_string(), "500");
        assert_eq!(HumanCount::from(1_000).concise(), "1K");
        assert_eq!(HumanCount::from(1_500).to_string(), "1.5 thousand");
        assert_eq!(HumanCount::from(1_000_000).to_string(), "1 million");
        assert_eq!(HumanCount::from(1_500_000).concise(), "1.5M");
        assert_eq!(HumanCount::from(1_000_000_000).concise(), "1B");
        assert_eq!(HumanCount::from(1_500_000_000).to_string(), "1.5 billion");
    }

    #[test]
    fn test_human_size() {
        assert_eq!(HumanSize::from(500).concise(), "500 B");
        assert_eq!(HumanSize::from(1024).to_string(), "1 kibibyte");
        assert_eq!(HumanSize::from(1_048_576).to_string(), "1 mebibyte");
        assert_eq!(HumanSize::from(1_500_000).concise(), "1.4 MiB");
        assert_eq!(HumanSize::from(1_073_741_824).to_string(), "1 gibibyte");
    }

    #[test]
    fn test_human_time() {
        let now = SystemTime::now();
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(0))).to_string(),
            "just now"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(45))).concise(),
            "45s ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(120))).to_string(),
            "2 minutes ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(7200))).to_string(),
            "2 hours ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(172_800))).concise(),
            "2d ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(1_209_600))).concise(),
            "2w ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(5_259_492))).to_string(),
            "2 months ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(63_113_904))).concise(),
            "2y ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(86_400))).to_string(),
            "yesterday"
        );
    }

    #[test]
    fn test_human_duration() {
        assert_eq!(HumanTime::from(Duration::from_secs(45)).concise(), "45s");
        assert_eq!(HumanTime::from(Duration::from_secs(90)).concise(), "1m 30s");
        assert_eq!(
            HumanTime::from(Duration::from_secs(3672)).to_string(),
            "1 hour 1 minute 12 seconds"
        );
    }

    #[test]
    fn test_human_percent() {
        assert_eq!(HumanPercent::from(12.3456, 0).concise(), "12%");
        assert_eq!(HumanPercent::from(12.3456, 1).concise(), "12.3%");
        assert_eq!(HumanPercent::from(12.3456, 2).to_string(), "12.35 percent");
        assert_eq!(HumanPercent::from(0.1234 * 100.0, 1).to_string(), "12.3 percent");
    }
}
