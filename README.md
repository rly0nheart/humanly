# Humanly

A pure-Rust crate to convert numbers, sizes, durations, times, and percentages
into human-readable formats.

## Crate Modules

- `HumanNumber`: Convert numbers to K/M/B/T notation (short) or word format (long).
- `HumanSize`: Convert bytes to human-readable units (KiB, MiB…).
- `HumanDuration`: Convert a `Duration` into H:M:S strings.
- `HumanRelative`: Show how long ago (or until) a timestamp is, in short or long format.
- `HumanPercent`: Round floats and display as percentage string.

## Output Formats

Each type provides `.short()` and `.long()` methods. They return a
`HumanDisplay` view that implements `Display` and writes directly into the
formatter, so `println!`/`write!` allocate nothing. Call `.to_string()` when you
want an owned `String`.

```rust
use human::HumanNumber;

// Short: "1.8k"
println!("{}", HumanNumber::new(1_800).short());

// Long: "1.8 thousand"
println!("{}", HumanNumber::new(1_800).long());
```

## Examples

```rust
use human::{HumanNumber, HumanSize, HumanDuration, HumanRelative, HumanPercent};
use std::time::{Duration, SystemTime};

// HumanNumber
assert_eq!(HumanNumber::new(1_200).short().to_string(), "1.2k");
assert_eq!(HumanNumber::new(1_200).long().to_string(), "1.2 thousand");
assert_eq!(HumanNumber::new(1_800_000).short().to_string(), "1.8M");
assert_eq!(HumanNumber::new(1_800_000).long().to_string(), "1.8 million");
assert_eq!(HumanNumber::new(2_500_000_000.0).short().to_string(), "2.5B");
assert_eq!(HumanNumber::new(2_500_000_000.0).long().to_string(), "2.5 billion");
assert_eq!(HumanNumber::new(3_700_000_000_000.0).short().to_string(), "3.7T");
assert_eq!(HumanNumber::new(3_700_000_000_000.0).long().to_string(), "3.7 trillion");

// Concrete numeric types also convert via `From`/`Into`:
let number: HumanNumber = 1_200_i32.into();
assert_eq!(number.short().to_string(), "1.2k");

// HumanSize
// Binary (default, 1024-based)
assert_eq!(HumanSize::new(5_242_880).short().to_string(), "5 MiB");
assert_eq!(HumanSize::new(5_242_880).long().to_string(), "5 mebibytes");

// Decimal (SI, 1000-based)
let human_size = HumanSize::new(5_000_000);
assert_eq!(human_size.decimal().short().to_string(), "5 MB");
assert_eq!(human_size.decimal().long().to_string(), "5 megabytes");

// HumanRelative
let now = SystemTime::now();
let result = HumanRelative::new(now - Duration::from_secs(75)).short().to_string();
assert!(result.contains("1m"));

// HumanDuration
assert_eq!(HumanDuration::new(Duration::from_secs(3661)).short().to_string(), "1h 1m 1s");
assert_eq!(HumanDuration::new(Duration::from_secs(3661)).to_string(), "1 hour 1 minute 1 second");

// HumanPercent
assert_eq!(HumanPercent::new(12.3456, 1).short().to_string(), "12.3%");
assert_eq!(HumanPercent::new(12.3456, 1).to_string(), "12.3 percent");
```
