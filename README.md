# Humaniser

A pure Rust crate to convert numbers, sizes, durations, times, and percentages
into human-readable formats.

## Features
- `HumanCount` — Convert numbers to readable short format (1K, 1M, 1B…).
- `HumanSize` — Convert bytes to human-readable units (KiB, MiB…).
- `HumanDuration` — Show how long ago a timestamp occurred in short format.
- `HumanTime` — Convert `Duration` into H:M:S strings.
- `HumanPercent` — Round floats and display as percentage string.

## Examples

```rust
use humaniser::*;

// HumanCount
assert_eq!(HumanCount::from(1_200), "1.2K");

// HumanSize
assert_eq!(HumanSize::from(5_242_880), "5 MiB");

// HumanDuration
use std::time::{Duration, SystemTime};
let now = SystemTime::now();
assert!(HumanDuration::from(Some(now - Duration::from_secs(75))).contains("1m"));

// HumanTime
assert_eq!(HumanTime::from(Duration::from_secs(3661)), "1h 1m 1s");

// HumanPercent
assert_eq!(HumanPercent::from(12.3456, 1), "12.3%");
```