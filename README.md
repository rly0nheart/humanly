# Humanly

A pure Rust crate to convert numbers, sizes, durations, times, and percentages
into human-readable formats.

## Crate Modules

- `HumanNumber` — Convert numbers to K/M/B/T notation (concise) or word format (full).
- `HumanSize` — Convert bytes to human-readable units (KiB, MiB…).
- `HumanDuration` — Show how long ago a timestamp occurred in short or long format.
- `HumanTime` — Convert `Duration` into H:M:S strings.
- `HumanPercent` — Round floats and display as percentage string.

## Output Formats

Each type provides `.concise()` and `.full()` methods for different output styles:

```rust
use humanly::HumanNumber;

// Concise: "1.8k"
println!("{}", HumanNumber::from(1_800).concise());

// Full: "1.8 thousand"
println!("{}", HumanNumber::from(1_800).full());
```

## Examples

```rust
use humanly::{HumanNumber, HumanSize, HumanDuration, HumanTime, HumanPercent};
use std::time::{Duration, SystemTime};

// HumanNumber
assert_eq!(HumanNumber::from(1_200).concise(), "1.2k");
assert_eq!(HumanNumber::from(1_200).full(), "1.2 thousand");
assert_eq!(HumanNumber::from(1_800_000).concise(), "1.8M");
assert_eq!(HumanNumber::from(1_800_000).full(), "1.8 million");
assert_eq!(HumanNumber::from(2_500_000_000.0).concise(), "2.5B");
assert_eq!(HumanNumber::from(2_500_000_000.0).full(), "2.5 billion");
assert_eq!(HumanNumber::from(3_700_000_000_000.0).concise(), "3.7T");
assert_eq!(HumanNumber::from(3_700_000_000_000.0).full(), "3.7 trillion");

// HumanSize
// Binary (default, 1024-based)
assert_eq!(HumanSize::from(5_242_880).concise(), "5 MiB");
assert_eq!(HumanSize::from(5_242_880).full(), "5 mebibytes");

// Decimal (SI, 1000-based)
let human_size = HumanSize::from(5_000_000);
assert_eq!(human_size.decimal().concise(), "5 MB");
assert_eq!(human_size.decimal().full(), "5 megabytes");

// HumanDuration
let now = SystemTime::now();
let result = HumanDuration::from(Some(now - Duration::from_secs(75))).concise();
assert!(result.contains("1m"));

// HumanTime
assert_eq!(HumanTime::from(Duration::from_secs(3661)).concise(), "1h 1m 1s");
assert_eq!(HumanTime::from(Duration::from_secs(3661)).to_string(), "1 hour 1 minute 1 second");

// HumanPercent
assert_eq!(HumanPercent::from(12.3456, 1).concise(), "12.3%");
assert_eq!(HumanPercent::from(12.3456, 1).to_string(), "12.3 percent");
```