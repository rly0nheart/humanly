//! # libhuman
//!
//! A pure Rust crate to convert numbers, sizes, durations, times, and percentages
//! into human-readable formats.
//!
//! ## Quick Links
//! - [`HumanNumber`]: Convert large numbers into k, M, B, T or thousand/million/billion/trillion
//! - [`HumanSize`]: Convert bytes into KiB, MiB, GiB, etc.
//! - [`HumanDuration`]: Format a `Duration` as H:M:S
//! - [`HumanRelative`]: Show how long ago (or until) a timestamp is
//! - [`HumanPercent`]: Round and format floating-point numbers as percentages
//!
//! ## Output formats
//!
//! Each type provides `.short()` and `.long()` methods. They return a
//! [`HumanDisplay`] view that implements [`Display`](std::fmt::Display) and
//! formats directly into the writer — `println!`/`write!` allocate nothing.
//! Call `.to_string()` when you want an owned `String`.
//!
//! ```rust
//! use human::HumanNumber;
//!
//! // Short: "1.8k" — printed with no intermediate String
//! println!("{}", HumanNumber::new(1_800).short());
//!
//! // Long: "1.8 thousand"
//! println!("{}", HumanNumber::new(1_800).long());
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use human::{HumanNumber, HumanSize, HumanDuration, HumanRelative, HumanPercent};
//! use std::time::{Duration, SystemTime};
//!
//! // HumanNumber
//! assert_eq!(HumanNumber::new(1_200).short().to_string(), "1.2k");
//! assert_eq!(HumanNumber::new(1_200).long().to_string(), "1.2 thousand");
//! assert_eq!(HumanNumber::new(1_800_000).short().to_string(), "1.8M");
//! assert_eq!(HumanNumber::new(1_800_000).long().to_string(), "1.8 million");
//! assert_eq!(HumanNumber::new(2_500_000_000.0).short().to_string(), "2.5B");
//! assert_eq!(HumanNumber::new(2_500_000_000.0).long().to_string(), "2.5 billion");
//! assert_eq!(HumanNumber::new(3_700_000_000_000.0).short().to_string(), "3.7T");
//! assert_eq!(HumanNumber::new(3_700_000_000_000.0).long().to_string(), "3.7 trillion");
//!
//! // HumanSize
//! // Binary (default, 1024-based)
//! assert_eq!(HumanSize::new(5_242_880).short().to_string(), "5 MiB");
//! assert_eq!(HumanSize::new(5_242_880).long().to_string(), "5 mebibytes");
//!
//! // Decimal (SI, 1000-based)
//! let human_size = HumanSize::new(5_000_000);
//! assert_eq!(human_size.decimal().short().to_string(), "5 MB");
//! assert_eq!(human_size.decimal().long().to_string(), "5 megabytes");
//!
//! // Ensure chaining works
//! let human_size_2 = HumanSize::new(1_000_000);
//! assert_eq!(human_size_2.binary().short().to_string(), "976.6 KiB");
//! assert_eq!(human_size_2.binary().long().to_string(), "976.6 kibibytes");
//!
//! // HumanRelative
//! let now = SystemTime::now();
//! let result = HumanRelative::new(now - Duration::from_secs(75)).short().to_string();
//! assert!(result.contains("1m"));
//!
//! // HumanDuration
//! assert_eq!(HumanDuration::new(Duration::from_secs(3661)).short().to_string(), "1h 1m 1s");
//! assert_eq!(HumanDuration::new(Duration::from_secs(3661)).to_string(), "1 hour 1 minute 1 second");
//!
//! // HumanPercent
//! assert_eq!(HumanPercent::new(12.3456, 1).short().to_string(), "12.3%");
//! assert_eq!(HumanPercent::new(12.3456, 1).to_string(), "12.3 percent");
//! ```
//!
//! ## Crate modules
//!
//! - [`HumanNumber`] — Convert numbers to K/M/B/T notation (short) or word format (long).
//! - [`HumanSize`] — Convert bytes to human-readable units (KiB, MiB…).
//! - [`HumanDuration`] — Convert a `Duration` into H:M:S strings.
//! - [`HumanRelative`] — Show how long ago (or until) a timestamp is, in short or long format.
//! - [`HumanPercent`] — Round floats and display as percentage string.
//!
//! [`HumanNumber`]: struct.HumanNumber.html
//! [`HumanSize`]: struct.HumanSize.html
//! [`HumanDuration`]: struct.HumanDuration.html
//! [`HumanRelative`]: struct.HumanRelative.html
//! [`HumanPercent`]: struct.HumanPercent.html
//! [`HumanDisplay`]: struct.HumanDisplay.html

mod libhuman;
pub use libhuman::HumanDisplay;
pub use libhuman::HumanDuration;
pub use libhuman::HumanNumber;
pub use libhuman::HumanPercent;
pub use libhuman::HumanRelative;
pub use libhuman::HumanSize;

#[cfg(test)]
mod tests {
    use crate::libhuman::{
        HumanDuration, HumanNumber, HumanPercent, HumanRelative, HumanSize,
    };
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_human_number() {
        // Test long format (word format)
        assert_eq!(HumanNumber::new(500).long().to_string(), "500");
        assert_eq!(HumanNumber::new(1_000).long().to_string(), "1 thousand");
        assert_eq!(HumanNumber::new(1_500).long().to_string(), "1.5 thousand");
        assert_eq!(HumanNumber::new(1_700_700).long().to_string(), "1.7 million");
        assert_eq!(HumanNumber::new(1_000_000).long().to_string(), "1 million");
        assert_eq!(HumanNumber::new(1_500_000).long().to_string(), "1.5 million");
        assert_eq!(HumanNumber::new(1_000_000_000).long().to_string(), "1 billion");
        assert_eq!(HumanNumber::new(1_500_000_000).long().to_string(), "1.5 billion");
        assert_eq!(HumanNumber::new(1_000_000_000_000.0).long().to_string(), "1 trillion");
        assert_eq!(HumanNumber::new(2_500_000_000_000.0).long().to_string(), "2.5 trillion");

        // Test short format (k/M/B/T notation)
        assert_eq!(HumanNumber::new(500).short().to_string(), "500");
        assert_eq!(HumanNumber::new(999).short().to_string(), "999");
        assert_eq!(HumanNumber::new(1_000).short().to_string(), "1k");
        assert_eq!(HumanNumber::new(1_500).short().to_string(), "1.5k");
        assert_eq!(HumanNumber::new(1_700_700).short().to_string(), "1.7M");
        assert_eq!(HumanNumber::new(1_000_000).short().to_string(), "1M");
        assert_eq!(HumanNumber::new(1_500_000).short().to_string(), "1.5M");
        assert_eq!(HumanNumber::new(1_000_000_000).short().to_string(), "1B");
        assert_eq!(HumanNumber::new(1_500_000_000).short().to_string(), "1.5B");
        assert_eq!(HumanNumber::new(1_000_000_000_000.0).short().to_string(), "1T");
        assert_eq!(HumanNumber::new(2_500_000_000_000.0).short().to_string(), "2.5T");

        // Test Display trait (should use long format)
        assert_eq!(HumanNumber::new(1_500).to_string(), "1.5 thousand");
        assert_eq!(HumanNumber::new(1_500_000).to_string(), "1.5 million");

        // From / Into for concrete numeric types
        let via_into: HumanNumber = 1_500_i32.into();
        assert_eq!(via_into.short().to_string(), "1.5k");
    }

    #[test]
    fn test_human_size() {
        // Binary (default)
        assert_eq!(HumanSize::new(0).short().to_string(), "0 B");
        assert_eq!(HumanSize::new(1).short().to_string(), "1 B");
        assert_eq!(HumanSize::new(1).long().to_string(), "1 byte");
        assert_eq!(HumanSize::new(500).short().to_string(), "500 B");
        assert_eq!(HumanSize::new(500).long().to_string(), "500 bytes");
        assert_eq!(HumanSize::new(1023).short().to_string(), "1023 B");
        assert_eq!(HumanSize::new(1024).short().to_string(), "1 KiB");
        assert_eq!(HumanSize::new(1024).to_string(), "1 kibibyte");
        assert_eq!(HumanSize::new(1_048_576).to_string(), "1 mebibyte");
        assert_eq!(HumanSize::new(1_500_000).short().to_string(), "1.4 MiB");
        assert_eq!(HumanSize::new(1_073_741_824).to_string(), "1 gibibyte");

        // Decimal (SI)
        let hs = HumanSize::new(5_000_000);
        assert_eq!(hs.decimal().short().to_string(), "5 MB");
        assert_eq!(hs.decimal().to_string(), "5 megabytes");

        // Ensure chaining works
        let hs2 = HumanSize::new(1_000_000);
        assert_eq!(hs2.binary().short().to_string(), "976.6 KiB");
        assert_eq!(hs2.binary().to_string(), "976.6 kibibytes");
    }

    #[test]
    fn test_human_relative() {
        let now = SystemTime::now();

        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(0)).to_string(),
            "just now"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(45)).short().to_string(),
            "45s ago"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(120)).to_string(),
            "2 minutes ago"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(7200)).to_string(),
            "2 hours ago"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(172_800)).short().to_string(),
            "2d ago"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(1_209_600)).short().to_string(),
            "2w ago"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(5_259_492)).to_string(),
            "2 months ago"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(63_113_904)).short().to_string(),
            "2y ago"
        );
        assert_eq!(
            HumanRelative::new(now - Duration::from_secs(86_400)).to_string(),
            "yesterday"
        );

        // Future timestamps: long format says "from now", not "ago"
        assert_eq!(
            HumanRelative::new(now + Duration::from_secs(150)).short().to_string(),
            "2m from now"
        );
        assert_eq!(
            HumanRelative::new(now + Duration::from_secs(150)).long().to_string(),
            "2 minutes from now"
        );
        assert_eq!(
            HumanRelative::new(now + Duration::from_secs(5400)).long().to_string(),
            "1 hour from now"
        );
        assert_eq!(
            HumanRelative::new(now + Duration::from_secs(90_000)).long().to_string(),
            "tomorrow"
        );
    }

    #[test]
    fn test_human_duration() {
        assert_eq!(
            HumanDuration::new(Duration::from_secs(45)).short().to_string(),
            "45s"
        );
        assert_eq!(
            HumanDuration::new(Duration::from_secs(90)).short().to_string(),
            "1m 30s"
        );
        assert_eq!(
            HumanDuration::new(Duration::from_secs(3672)).to_string(),
            "1 hour 1 minute 12 seconds"
        );
    }

    #[test]
    fn test_human_percent() {
        assert_eq!(HumanPercent::new(12.3456, 0).short().to_string(), "12%");
        assert_eq!(HumanPercent::new(12.3456, 1).short().to_string(), "12.3%");
        assert_eq!(HumanPercent::new(12.3456, 2).to_string(), "12.35 percent");
        assert_eq!(
            HumanPercent::new(0.1234 * 100.0, 1).to_string(),
            "12.3 percent"
        );
    }
}
