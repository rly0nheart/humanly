//! # Humanity
//!
//! A pure Rust crate to convert numbers, sizes, durations, times, and percentages
//! into human-readable formats.
//!
//! ## Quick Links
//! - [`HumanNumber`]: Convert large numbers into K, M, B, T or thousand/million/billion/trillion
//! - [`HumanSize`]: Convert bytes into KiB, MiB, GiB, etc.
//! - [`HumanDuration`]: Show elapsed time since a timestamp in human-readable format
//! - [`HumanTime`]: Format a `Duration` as H:M:S
//! - [`HumanPercent`]: Round and format floating-point numbers as percentages
//!
//! ## Output formats
//!
//! Each type provides `.concise()` and `.full()` methods for different output styles:
//!
//! ```rust
//! use humanity::HumanNumber;
//!
//! // Concise: "1.8k"
//! println!("{}", HumanNumber::from(1_800).concise());
//!
//! // Full: "1.8 thousand"
//! println!("{}", HumanNumber::from(1_800).full());
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use humanity::{HumanNumber, HumanSize, HumanDuration, HumanTime, HumanPercent};
//! use std::time::{Duration, SystemTime};
//!
//! // HumanNumber
//! assert_eq!(HumanNumber::from(1_200).concise(), "1.2k");
//! assert_eq!(HumanNumber::from(1_200).full(), "1.2 thousand");
//! assert_eq!(HumanNumber::from(1_800_000).concise(), "1.8M");
//! assert_eq!(HumanNumber::from(1_800_000).full(), "1.8 million");
//! assert_eq!(HumanNumber::from(2_500_000_000.0).concise(), "2.5B");
//! assert_eq!(HumanNumber::from(2_500_000_000.0).full(), "2.5 billion");
//! assert_eq!(HumanNumber::from(3_700_000_000_000.0).concise(), "3.7T");
//! assert_eq!(HumanNumber::from(3_700_000_000_000.0).full(), "3.7 trillion");
//!
//! // HumanSize
//! // Binary (default, 1024-based)
//! assert_eq!(HumanSize::from(5_242_880).concise(), "5 MiB");
//! assert_eq!(HumanSize::from(5_242_880).full(), "5 mebibytes");
//!
//! // Decimal (SI, 1000-based)
//! let human_size = HumanSize::from(5_000_000);
//! assert_eq!(human_size.decimal().concise(), "5 MB");
//! assert_eq!(human_size.decimal().full(), "5 megabytes");
//!
//! // Ensure chaining works
//! let human_size_2 = HumanSize::from(1_000_000);
//! assert_eq!(human_size_2.binary().concise(), "976.6 KiB");
//! assert_eq!(human_size_2.binary().full(), "976.6 kibibytes");
//!
//! // HumanDuration
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
//! ## Crate modules
//!
//! - [`HumanNumber`] — Convert numbers to K/M/B/T notation (concise) or word format (full).
//! - [`HumanSize`] — Convert bytes to human-readable units (KiB, MiB…).
//! - [`HumanDuration`] — Show how long ago a timestamp occurred in short or long format.
//! - [`HumanTime`] — Convert `Duration` into H:M:S strings.
//! - [`HumanPercent`] — Round floats and display as percentage string.
//!
//! [`HumanNumber`]: struct.HumanNumber.html
//! [`HumanSize`]: struct.HumanSize.html
//! [`HumanDuration`]: struct.HumanDuration.html
//! [`HumanTime`]: struct.HumanTime.html
//! [`HumanPercent`]: struct.HumanPercent.html

mod core;
pub use core::HumanNumber;
pub use core::HumanDuration;
pub use core::HumanPercent;
pub use core::HumanSize;
pub use core::HumanTime;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::core::{HumanNumber, HumanDuration, HumanPercent, HumanSize, HumanTime};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_human_number() {
        // Test full format (word format)
        assert_eq!(HumanNumber::from(500).full(), "500");
        assert_eq!(HumanNumber::from(1_000).full(), "1 thousand");
        assert_eq!(HumanNumber::from(1_500).full(), "1.5 thousand");
        assert_eq!(HumanNumber::from(1_700_700).full(), "1.7 million");
        assert_eq!(HumanNumber::from(1_000_000).full(), "1 million");
        assert_eq!(HumanNumber::from(1_500_000).full(), "1.5 million");
        assert_eq!(HumanNumber::from(1_000_000_000).full(), "1 billion");
        assert_eq!(HumanNumber::from(1_500_000_000).full(), "1.5 billion");
        assert_eq!(HumanNumber::from(1_000_000_000_000.0).full(), "1 trillion");
        assert_eq!(HumanNumber::from(2_500_000_000_000.0).full(), "2.5 trillion");

        // Test concise format (K/M/B/T notation)
        assert_eq!(HumanNumber::from(500).concise(), "500");
        assert_eq!(HumanNumber::from(999).concise(), "999");
        assert_eq!(HumanNumber::from(1_000).concise(), "1k");
        assert_eq!(HumanNumber::from(1_500).concise(), "1.5k");
        assert_eq!(HumanNumber::from(1_700_700).concise(), "1.7M");
        assert_eq!(HumanNumber::from(1_000_000).concise(), "1M");
        assert_eq!(HumanNumber::from(1_500_000).concise(), "1.5M");
        assert_eq!(HumanNumber::from(1_000_000_000).concise(), "1B");
        assert_eq!(HumanNumber::from(1_500_000_000).concise(), "1.5B");
        assert_eq!(HumanNumber::from(1_000_000_000_000.0).concise(), "1T");
        assert_eq!(HumanNumber::from(2_500_000_000_000.0).concise(), "2.5T");

        // Test that trailing .0 is removed
        assert_eq!(HumanNumber::from(100_000).concise(), "100k");
        assert_eq!(HumanNumber::from(5_000_000).concise(), "5M");

        // Test Display trait (should use full format)
        assert_eq!(HumanNumber::from(1_500).to_string(), "1.5 thousand");
        assert_eq!(HumanNumber::from(1_500_000).to_string(), "1.5 million");
    }

    #[test]
    fn test_human_size() {
        // Binary (default)
        assert_eq!(HumanSize::from(0).concise(), "0");
        assert_eq!(HumanSize::from(1).concise(), "1");
        assert_eq!(HumanSize::from(1).full(), "1 byte");
        assert_eq!(HumanSize::from(500).concise(), "500");
        assert_eq!(HumanSize::from(500).full(), "500 bytes");
        assert_eq!(HumanSize::from(1023).concise(), "1023");
        assert_eq!(HumanSize::from(1024).concise(), "1 KiB");
        assert_eq!(HumanSize::from(1024).to_string(), "1 kibibyte");
        assert_eq!(HumanSize::from(1_048_576).to_string(), "1 mebibyte");
        assert_eq!(HumanSize::from(1_500_000).concise(), "1.4 MiB");
        assert_eq!(HumanSize::from(1_073_741_824).to_string(), "1 gibibyte");

        // Decimal (SI)
        let hs = HumanSize::from(5_000_000);
        assert_eq!(hs.decimal().concise(), "5 MB");
        assert_eq!(hs.decimal().to_string(), "5 megabytes");

        // Ensure chaining works
        let hs2 = HumanSize::from(1_000_000);
        assert_eq!(hs2.binary().concise(), "976.6 KiB");
        assert_eq!(hs2.binary().to_string(), "976.6 kibibytes");
    }

    #[test]
    fn test_human_duration() {
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
    fn test_human_time() {
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