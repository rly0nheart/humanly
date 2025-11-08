# Humaniser

A small, ergonomic Rust crate to convert numbers, sizes, durations, times, percentages, and permissions
into human-readable formats.

## Features
- `HumanCount` — Convert numbers to readable short format (1K, 1M, 1B…).
- `HumanSize` — Convert bytes to human-readable units (KiB, MiB…).
- `HumanDuration` — Show how long ago a timestamp occurred in short format.
- `HumanTime` — Convert `Duration` into H:M:S strings.
- `HumanPercent` — Round floats and display as percentage string.
- `HumanPermissions` — Partially uses [unix_mode](https://github.com/sourcefrog/unix_mode) to convert numeric mode to a readable permissions string.

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

// HumanPermissions (Unix example)
assert_eq!(HumanPermissions::from(0o40755), "drwxr-xr-x");

// HumanPermissions (Windows-style)
#[cfg(windows)]
assert_eq!(
    HumanPermissions::from(0o40755),
    "User: Read, Write, Execute; Group: Read, Execute; Other: Read, Execute"
);
```

## Goals

- Provide a simple, consistent API to make numeric and system values readable.
- Cross-platform support for permissions (Unix and Windows).
- Small, dependency-light, and ergonomic for CLI, web, or general Rust applications.

## Platform-specific behavior

- **Unix**: Permissions use `unix_mode` crate to show `rwx` strings with file type prefix.
- **Windows**: Permissions are translated into descriptive text like `Read`, `Write`, `Execute` (this is untested)

