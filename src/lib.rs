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
//! - [`HumanPermissions`]: Show Unix or Windows-style permissions from mode bits
//!
//! ## Output formats
//!
//! Each type provides three formatting methods for flexibility:
//!
//! - `.concise()` — returns a short, compact format (e.g., `"1.2K"`, `"5 MiB"`, `"3h 12m 5s"`, `"12.3%"`, `"3d ago"`).
//! - `.full()` — returns a descriptive, fully written-out format (e.g., `"1.2 thousand"`, `"5 mebibytes"`, `"3 hours 12 minutes 5 seconds"`, `"12.3 percent"`, `"3 days ago"`).
//!
//! ## Examples
//!
//! ```rust
//! use humaniser::*;
//!
//! // HumanCount
//! assert_eq!(HumanCount::from(1_200).concise(), "1.2K");
//! assert_eq!(HumanCount::from(1_200).full(), "1.2 thousand");
//!
//! // HumanSize
//! assert_eq!(HumanSize::from(5_242_880).concise(), "5 MiB");
//! assert_eq!(HumanSize::from(5_242_880).full(), "5 mebibytes");
//!
//! // HumanDuration
//! use std::time::{Duration, SystemTime};
//! let now = SystemTime::now();
//! let result = HumanDuration::from(Some(now - Duration::from_secs(75))).concise();
//! assert!(result.contains("1m"));
//!
//! // HumanTime
//! assert_eq!(HumanTime::from(Duration::from_secs(3661)).concise(), "1h 1m 1s");
//! assert_eq!(HumanTime::from(Duration::from_secs(3661)).full(), "1 hour 1 minute 1 second");
//!
//! // HumanPercent
//! assert_eq!(HumanPercent::from(12.3456, 1).concise(), "12.3%");
//! assert_eq!(HumanPercent::from(12.3456, 1).full(), "12.3 percent");
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
//! - [`HumanPermissions`] — Convert numeric mode to readable permissions string.
//!
//! [`HumanCount`]: struct.HumanCount.html
//! [`HumanSize`]: struct.HumanSize.html
//! [`HumanDuration`]: struct.HumanDuration.html
//! [`HumanTime`]: struct.HumanTime.html
//! [`HumanPercent`]: struct.HumanPercent.html
//! [`HumanPermissions`]: struct.HumanPermissions.html

mod human;
pub use human::HumanCount;
pub use human::HumanDuration;
pub use human::HumanPercent;
pub use human::HumanSize;
pub use human::HumanTime;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::human::{
        HumanCount, HumanDuration, HumanPercent, HumanSize, HumanTime,
    };
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_human_count() {
        assert_eq!(HumanCount::from(500).full(), "500");
        assert_eq!(HumanCount::from(1_000).concise(), "1K");
        assert_eq!(HumanCount::from(1_500).full(), "1.5 thousand");
        assert_eq!(HumanCount::from(1_000_000).full(), "1 million");
        assert_eq!(HumanCount::from(1_500_000).concise(), "1.5M");
        assert_eq!(HumanCount::from(1_000_000_000).concise(), "1B");
        assert_eq!(HumanCount::from(1_500_000_000).full(), "1.5 billion");
    }

    #[test]
    fn test_human_size() {
        assert_eq!(HumanSize::from(500).concise(), "500 B");
        assert_eq!(HumanSize::from(1024).full(), "1 kibibyte");
        assert_eq!(HumanSize::from(1_048_576).full(), "1 mebibyte");
        assert_eq!(HumanSize::from(1_500_000).concise(), "1.4 MiB");
        assert_eq!(HumanSize::from(1_073_741_824).full(), "1 gibibyte");
    }

    #[test]
    fn test_human_time() {
        let now = SystemTime::now();
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(5))).full(),
            "just now"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(45))).concise(),
            "45s ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(120))).full(),
            "2 minutes ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(7200))).full(),
            "2 hours ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(172_800))).concise(),
            "2d ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(1_209_600))).concise(),
            "2wk ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(5_259_492))).full(),
            "2 months ago"
        );
        assert_eq!(
            HumanDuration::from(Some(now - Duration::from_secs(63_113_904))).concise(),
            "2yr ago"
        );
    }

    #[test]
    fn test_human_duration() {
        assert_eq!(HumanTime::from(Duration::from_secs(45)).concise(), "45s");
        assert_eq!(HumanTime::from(Duration::from_secs(90)).concise(), "1m 30s");
        assert_eq!(
            HumanTime::from(Duration::from_secs(3672)).full(),
            "1 hour 1 minute 12 seconds"
        );
    }

    #[test]
    fn test_human_percent() {
        assert_eq!(HumanPercent::from(12.3456, 0).concise(), "12%");
        assert_eq!(HumanPercent::from(12.3456, 1).concise(), "12.3%");
        assert_eq!(HumanPercent::from(12.3456, 2).full(), "12.35 percent");
        assert_eq!(HumanPercent::from(0.1234 * 100.0, 1).full(), "12.3 percent");
    }
}
