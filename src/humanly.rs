use std::fmt;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy)]
enum HumanFormat {
    Concise,
    Full,
}

macro_rules! human_display {
    ($t:ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.full())
            }
        }
    };
}

human_display!(HumanNumber);
human_display!(HumanSize);
human_display!(HumanDuration);
human_display!(HumanTime);
human_display!(HumanPercent);

/* -------------------- HumanNumber -------------------- */

/// Humanises large numbers into readable abbreviated or word formats.
///
/// Converts numeric values into concise notation (e.g. `"1.8k"`, `"2.5B"`) or
/// full word format (e.g. `"1.8 thousand"`, `"2.5 billion"`).
///
/// The [`Display`](fmt::Display) implementation outputs the [`full`](HumanNumber::full) format.
///
/// # Examples
///
/// ```
/// use humanly::HumanNumber;
///
/// assert_eq!(HumanNumber::from(1_500).concise(), "1.5k");
/// assert_eq!(HumanNumber::from(1_500).full(), "1.5 thousand");
/// assert_eq!(HumanNumber::from(2_500_000_000.0).to_string(), "2.5 billion");
/// ```
pub struct HumanNumber {
    number: f64,
}

impl HumanNumber {
    /// Creates a new `HumanNumber` from any value convertible to `f64`.
    ///
    /// # Parameters
    ///
    /// * `number` - The numeric value to humanise. Accepts any type implementing
    ///   `Into<f64>` (e.g. `i32`, `u64`, `f64`).
    ///
    /// # Returns
    ///
    /// A new [`HumanNumber`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanNumber;
    ///
    /// let from_int = HumanNumber::from(1_000);
    /// let from_float = HumanNumber::from(2_500_000_000.0);
    /// ```
    pub fn from(number: impl Into<f64>) -> Self {
        Self {
            number: number.into(),
        }
    }

    /// Returns the number in concise abbreviated format.
    ///
    /// Uses short suffixes: `k` (thousand), `M` (million), `B` (billion), `T` (trillion).
    /// Numbers below 1,000 are returned without a suffix.
    ///
    /// # Returns
    ///
    /// A [`String`] with the abbreviated representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanNumber;
    ///
    /// assert_eq!(HumanNumber::from(500).concise(), "500");
    /// assert_eq!(HumanNumber::from(1_200).concise(), "1.2k");
    /// assert_eq!(HumanNumber::from(1_800_000).concise(), "1.8M");
    /// assert_eq!(HumanNumber::from(1_000_000_000).concise(), "1B");
    /// assert_eq!(HumanNumber::from(3_700_000_000_000.0).concise(), "3.7T");
    /// ```
    pub fn concise(&self) -> String {
        self.format(HumanFormat::Concise)
    }

    /// Returns the number in full word format.
    ///
    /// Uses word suffixes: `thousand`, `million`, `billion`, `trillion`.
    /// Numbers below 1,000 are returned without a suffix.
    ///
    /// # Returns
    ///
    /// A [`String`] with the full word representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanNumber;
    ///
    /// assert_eq!(HumanNumber::from(500).full(), "500");
    /// assert_eq!(HumanNumber::from(1_200).full(), "1.2 thousand");
    /// assert_eq!(HumanNumber::from(1_800_000).full(), "1.8 million");
    /// assert_eq!(HumanNumber::from(1_000_000_000).full(), "1 billion");
    /// assert_eq!(HumanNumber::from(3_700_000_000_000.0).full(), "3.7 trillion");
    /// ```
    pub fn full(&self) -> String {
        self.format(HumanFormat::Full)
    }

    fn format(&self, format: HumanFormat) -> String {
        let number = self.number;
        let abs_number = number.abs();

        let (divisor, short_suffix, long_suffix) = if abs_number < 1_000.0 {
            (1.0, "", "")
        } else if abs_number < 1_000_000.0 {
            (1_000.0, "k", " thousand")
        } else if abs_number < 1_000_000_000.0 {
            (1_000_000.0, "M", " million")
        } else if abs_number < 1_000_000_000_000.0 {
            (1_000_000_000.0, "B", " billion")
        } else {
            (1_000_000_000_000.0, "T", " trillion")
        };

        let value = number / divisor;
        let formatted = if value.fract() == 0.0 {
            format!("{}", value as i64)
        } else {
            let s = format!("{:.1}", value);
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        };

        match format {
            HumanFormat::Concise => format!("{}{}", formatted, short_suffix),
            HumanFormat::Full => format!("{}{}", formatted, long_suffix),
        }
    }
}
/* -------------------- HumanSize -------------------- */

#[derive(Clone, Copy, Debug)]
enum UnitSystem {
    Binary,  // IEC, 1024-based
    Decimal, // SI, 1000-based
}

/// Humanises byte counts into readable file-size strings.
///
/// Supports both binary (IEC, 1024-based: KiB, MiB, GiB…) and decimal
/// (SI, 1000-based: kB, MB, GB…) unit systems. Defaults to binary.
///
/// Use the [`decimal`](HumanSize::decimal) and [`binary`](HumanSize::binary) methods
/// to switch between unit systems via method chaining.
///
/// The [`Display`](fmt::Display) implementation outputs the [`full`](HumanSize::full) format.
///
/// # Examples
///
/// ```
/// use humanly::HumanSize;
///
/// // Binary (default)
/// assert_eq!(HumanSize::from(5_242_880).concise(), "5 MiB");
/// assert_eq!(HumanSize::from(5_242_880).full(), "5 mebibytes");
///
/// // Decimal
/// assert_eq!(HumanSize::from(5_000_000).decimal().concise(), "5 MB");
/// assert_eq!(HumanSize::from(5_000_000).decimal().full(), "5 megabytes");
/// ```
#[derive(Clone, Copy, Debug)]
pub struct HumanSize {
    bytes: u64,
    system: UnitSystem,
}

impl HumanSize {
    /// Creates a new `HumanSize` from a byte count, defaulting to binary (IEC) units.
    ///
    /// # Parameters
    ///
    /// * `bytes` - The number of bytes to humanise.
    ///
    /// # Returns
    ///
    /// A new [`HumanSize`] instance using the binary unit system.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanSize;
    ///
    /// let size = HumanSize::from(1_048_576);
    /// assert_eq!(size.concise(), "1 MiB");
    /// ```
    pub fn from(bytes: u64) -> Self {
        Self {
            bytes,
            system: UnitSystem::Binary,
        }
    }

    /// Switches to the decimal (SI, 1000-based) unit system.
    ///
    /// Units used: kB, MB, GB, TB, PB, EB, ZB, YB.
    ///
    /// # Returns
    ///
    /// The same [`HumanSize`] instance with the unit system set to decimal,
    /// enabling method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanSize;
    ///
    /// assert_eq!(HumanSize::from(5_000_000).decimal().concise(), "5 MB");
    /// assert_eq!(HumanSize::from(5_000_000).decimal().full(), "5 megabytes");
    /// ```
    pub fn decimal(mut self) -> Self {
        self.system = UnitSystem::Decimal;
        self
    }

    /// Switches to the binary (IEC, 1024-based) unit system.
    ///
    /// This is the default, but can be useful when chaining after a previous
    /// call to [`decimal`](HumanSize::decimal). Units used: KiB, MiB, GiB, TiB, PiB, EiB, ZiB, YiB.
    ///
    /// # Returns
    ///
    /// The same [`HumanSize`] instance with the unit system set to binary,
    /// enabling method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanSize;
    ///
    /// assert_eq!(HumanSize::from(1_000_000).binary().concise(), "976.6 KiB");
    /// ```
    pub fn binary(mut self) -> Self {
        self.system = UnitSystem::Binary;
        self
    }

    /// Returns the size in concise format with abbreviated unit suffixes.
    ///
    /// Values below 1,024 bytes are returned with the `B` unit.
    ///
    /// # Returns
    ///
    /// A [`String`] with the concise size representation (e.g. `"5 MiB"`, `"1.4 GB"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanSize;
    ///
    /// assert_eq!(HumanSize::from(500).concise(), "500 B");
    /// assert_eq!(HumanSize::from(1024).concise(), "1 KiB");
    /// assert_eq!(HumanSize::from(5_242_880).concise(), "5 MiB");
    /// assert_eq!(HumanSize::from(5_000_000).decimal().concise(), "5 MB");
    /// ```
    pub fn concise(&self) -> String {
        self.format(HumanFormat::Concise)
    }

    /// Returns the size in full format with pluralised unit names.
    ///
    /// Values below 1,024 bytes are returned as `"N byte"` or `"N bytes"`.
    ///
    /// # Returns
    ///
    /// A [`String`] with the full size representation (e.g. `"5 mebibytes"`, `"1 gigabyte"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanSize;
    ///
    /// assert_eq!(HumanSize::from(1).full(), "1 byte");
    /// assert_eq!(HumanSize::from(500).full(), "500 bytes");
    /// assert_eq!(HumanSize::from(1024).full(), "1 kibibyte");
    /// assert_eq!(HumanSize::from(5_242_880).full(), "5 mebibytes");
    /// assert_eq!(HumanSize::from(5_000_000).decimal().full(), "5 megabytes");
    /// ```
    pub fn full(&self) -> String {
        self.format(HumanFormat::Full)
    }

    fn format(&self, format: HumanFormat) -> String {
        // If bytes, return the number with the `B` unit
        if self.bytes < 1024 {
            return match format {
                HumanFormat::Concise => format!("{} B", self.bytes),
                HumanFormat::Full => {
                    if self.bytes == 1 {
                        "1 byte".to_string()
                    } else {
                        format!("{} bytes", self.bytes)
                    }
                }
            };
        }

        // Unit arrays
        let (units_short, units_full, step) = match self.system {
            UnitSystem::Binary => (
                ["KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"],
                [
                    "kibibyte", "mebibyte", "gibibyte", "tebibyte", "pebibyte", "exbibyte",
                    "zebibyte", "yobibyte",
                ],
                1024.0,
            ),
            UnitSystem::Decimal => (
                ["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"],
                [
                    "kilobyte",
                    "megabyte",
                    "gigabyte",
                    "terabyte",
                    "petabyte",
                    "exabyte",
                    "zettabyte",
                    "yottabyte",
                ],
                1000.0,
            ),
        };

        let mut size = self.bytes as f64;
        let mut idx = 0;

        // First division to get to first unit (KiB or kB)
        size /= step;

        // Continue dividing if needed
        while size >= step && idx < units_short.len() - 1 {
            size /= step;
            idx += 1;
        }

        let rounded = (size * 10.0).round() / 10.0;
        let formatted = if rounded.fract() == 0.0 {
            format!("{}", rounded as u64)
        } else {
            format!("{:.1}", rounded)
        };

        match format {
            HumanFormat::Concise => format!("{} {}", formatted, units_short[idx]),
            HumanFormat::Full => {
                let unit = units_full[idx];
                let pluralized = if rounded == 1.0 {
                    unit.to_string()
                } else {
                    format!("{}s", unit)
                };
                format!("{} {}", formatted, pluralized)
            }
        }
    }
}

/* -------------------- HumanDuration -------------------- */

/// Humanises a [`SystemTime`] timestamp into a relative time string.
///
/// Computes the elapsed time between the given timestamp and the current time,
/// then formats it as a human-readable duration (e.g. `"45s ago"`, `"2 hours ago"`,
/// `"yesterday"`, `"3m from now"`). Handles both past and future timestamps.
///
/// If the timestamp is `None`, all output methods return `"-"`.
///
/// The [`Display`](fmt::Display) implementation outputs the [`full`](HumanDuration::full) format.
///
/// # Examples
///
/// ```
/// use humanly::HumanDuration;
/// use std::time::{Duration, SystemTime};
///
/// let past = Some(SystemTime::now() - Duration::from_secs(120));
/// assert_eq!(HumanDuration::from(past).concise(), "2m ago");
///
/// assert_eq!(HumanDuration::from(None).concise(), "-");
/// ```
pub struct HumanDuration {
    system_time: Option<SystemTime>,
}

impl HumanDuration {
    /// Creates a new `HumanDuration` from an optional [`SystemTime`].
    ///
    /// # Parameters
    ///
    /// * `system_time` - The timestamp to compute relative duration from. Pass `None`
    ///   to represent an absent or unknown time.
    ///
    /// # Returns
    ///
    /// A new [`HumanDuration`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanDuration;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let duration = HumanDuration::from(Some(SystemTime::now() - Duration::from_secs(60)));
    /// let unknown = HumanDuration::from(None);
    /// ```
    pub fn from(system_time: Option<SystemTime>) -> Self {
        Self { system_time }
    }

    /// Returns the relative duration in concise format.
    ///
    /// Uses short suffixes: `s` (seconds), `m` (minutes), `h` (hours), `d` (days),
    /// `w`/`wk` (weeks), `mo` (months), `y`/`yr` (years), followed by `ago` or `from now`.
    ///
    /// # Returns
    ///
    /// A [`String`] with the concise relative time (e.g. `"45s ago"`, `"2h from now"`),
    /// `"just now"` if less than one second, or `"-"` if the timestamp is `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanDuration;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let past = Some(SystemTime::now() - Duration::from_secs(90));
    /// assert_eq!(HumanDuration::from(past).concise(), "1m ago");
    ///
    /// assert_eq!(HumanDuration::from(None).concise(), "-");
    /// ```
    pub fn concise(&self) -> String {
        self.format(HumanFormat::Concise)
    }

    fn full(&self) -> String {
        self.format(HumanFormat::Full)
    }

    fn format(&self, format: HumanFormat) -> String {
        let now = SystemTime::now();
        if let Some(st) = self.system_time {
            let elapsed = match now.duration_since(st) {
                Ok(dur) => dur.as_secs() as i64,
                Err(err) => -(err.duration().as_secs() as i64),
            };

            if elapsed.abs() < 1 {
                return "just now".to_string();
            }

            let (count, concise_suffix, singular, plural) = if elapsed < 0 {
                // future
                let secs = -elapsed as u64;
                if secs < 60 {
                    (secs, "s from now", "second", "seconds")
                } else if secs < 3600 {
                    (secs / 60, "m from now", "minute", "minutes")
                } else if secs < 86_400 {
                    (secs / 3600, "h from now", "hour", "hours")
                } else if secs < 604_800 {
                    (secs / 86_400, "d from now", "day", "days")
                } else if secs < 2_592_000 {
                    (secs / 604_800, "wk from now", "week", "weeks")
                } else if secs < 31_536_000 {
                    (secs / 2_592_000, "mo from now", "month", "months")
                } else {
                    (secs / 31_536_000, "yr from now", "year", "years")
                }
            } else {
                let secs = elapsed as u64;
                if secs < 60 {
                    (secs, "s ago", "second", "seconds")
                } else if secs < 3600 {
                    (secs / 60, "m ago", "minute", "minutes")
                } else if secs < 86_400 {
                    (secs / 3600, "h ago", "hour", "hours")
                } else if secs < 604_800 {
                    (secs / 86_400, "d ago", "day", "days")
                } else if secs < 2_592_000 {
                    (secs / 604_800, "w ago", "week", "weeks")
                } else if secs < 31_536_000 {
                    (secs / 2_592_000, "mo ago", "month", "months")
                } else {
                    (secs / 31_536_000, "y ago", "year", "years")
                }
            };

            match format {
                HumanFormat::Concise => {
                    format!("{}{}", count, concise_suffix)
                }
                HumanFormat::Full => {
                    if count == 1 && singular == "day" && elapsed >= 0 {
                        "yesterday".to_string()
                    } else if count == 1 && singular == "day" && elapsed < 0 {
                        "tomorrow".to_string()
                    } else if count == 1 {
                        format!("1 {} ago", singular)
                    } else {
                        format!("{} {} ago", count, plural)
                    }
                }
            }
        } else {
            "-".to_string()
        }
    }
}

/* -------------------- HumanTime -------------------- */

/// Humanises a [`Duration`] into a hours/minutes/seconds string.
///
/// Breaks a duration down into its hour, minute, and second components and
/// formats them in either concise (e.g. `"1h 5m 30s"`) or full
/// (e.g. `"1 hour 5 minutes 30 seconds"`) format.
///
/// The [`Display`](fmt::Display) implementation outputs the [`full`](HumanTime::full) format.
///
/// # Examples
///
/// ```
/// use humanly::HumanTime;
/// use std::time::Duration;
///
/// assert_eq!(HumanTime::from(Duration::from_secs(3661)).concise(), "1h 1m 1s");
/// assert_eq!(HumanTime::from(Duration::from_secs(3661)).to_string(), "1 hour 1 minute 1 second");
/// ```
pub struct HumanTime {
    duration: Duration,
}

impl HumanTime {
    /// Creates a new `HumanTime` from a [`Duration`].
    ///
    /// # Parameters
    ///
    /// * `duration` - The time duration to humanise.
    ///
    /// # Returns
    ///
    /// A new [`HumanTime`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanTime;
    /// use std::time::Duration;
    ///
    /// let time = HumanTime::from(Duration::from_secs(90));
    /// assert_eq!(time.concise(), "1m 30s");
    /// ```
    pub fn from(duration: Duration) -> Self {
        Self { duration }
    }

    /// Returns the duration in concise format.
    ///
    /// Uses short suffixes: `h` (hours), `m` (minutes), `s` (seconds).
    /// Zero-value components are omitted where possible, though minutes are
    /// always shown when hours are present.
    ///
    /// # Returns
    ///
    /// A [`String`] with the concise duration (e.g. `"45s"`, `"1m 30s"`, `"2h 0m 5s"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanTime;
    /// use std::time::Duration;
    ///
    /// assert_eq!(HumanTime::from(Duration::from_secs(45)).concise(), "45s");
    /// assert_eq!(HumanTime::from(Duration::from_secs(90)).concise(), "1m 30s");
    /// assert_eq!(HumanTime::from(Duration::from_secs(3661)).concise(), "1h 1m 1s");
    /// ```
    pub fn concise(&self) -> String {
        self.format(HumanFormat::Concise)
    }

    fn full(&self) -> String {
        self.format(HumanFormat::Full)
    }

    fn format(&self, format: HumanFormat) -> String {
        let secs = self.duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        match format {
            HumanFormat::Concise => {
                let mut parts = Vec::new();
                if hours > 0 {
                    parts.push(format!("{}h", hours));
                }
                if minutes > 0 || hours > 0 {
                    parts.push(format!("{}m", minutes));
                }
                if seconds > 0 || parts.is_empty() {
                    parts.push(format!("{}s", seconds));
                }
                parts.join(" ")
            }
            HumanFormat::Full => {
                let mut parts = Vec::new();
                if hours > 0 {
                    parts.push(format!(
                        "{} {}",
                        hours,
                        if hours == 1 { "hour" } else { "hours" }
                    ));
                }
                if minutes > 0 {
                    parts.push(format!(
                        "{} {}",
                        minutes,
                        if minutes == 1 { "minute" } else { "minutes" }
                    ));
                }
                if seconds > 0 || parts.is_empty() {
                    parts.push(format!(
                        "{} {}",
                        seconds,
                        if seconds == 1 { "second" } else { "seconds" }
                    ));
                }
                parts.join(" ")
            }
        }
    }
}

/* -------------------- HumanPercent -------------------- */

/// Humanises a floating-point value into a percentage string.
///
/// Rounds to a configurable number of decimal places and formats the result
/// as either a concise percentage (e.g. `"12.3%"`) or full words
/// (e.g. `"12.3 percent"`). Non-finite values (`NaN`, infinity) return `"-"`.
///
/// The [`Display`](fmt::Display) implementation outputs the [`full`](HumanPercent::full) format.
///
/// # Examples
///
/// ```
/// use humanly::HumanPercent;
///
/// assert_eq!(HumanPercent::from(12.3456, 1).concise(), "12.3%");
/// assert_eq!(HumanPercent::from(12.3456, 2).to_string(), "12.35 percent");
/// assert_eq!(HumanPercent::from(f64::NAN, 1).concise(), "-");
/// ```
pub struct HumanPercent {
    value: f64,
    decimals: usize,
}

impl HumanPercent {
    /// Creates a new `HumanPercent` from a value and decimal precision.
    ///
    /// # Parameters
    ///
    /// * `value` - The percentage value (e.g. `12.3456` for ~12.3%).
    /// * `decimals` - The number of decimal places to round to.
    ///
    /// # Returns
    ///
    /// A new [`HumanPercent`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanPercent;
    ///
    /// let pct = HumanPercent::from(99.999, 1);
    /// assert_eq!(pct.concise(), "100%");
    /// ```
    pub fn from(value: f64, decimals: usize) -> Self {
        Self { value, decimals }
    }

    /// Returns the percentage in concise format with a `%` symbol.
    ///
    /// # Returns
    ///
    /// A [`String`] with the percentage (e.g. `"12.3%"`), or `"-"` if the value
    /// is non-finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use humanly::HumanPercent;
    ///
    /// assert_eq!(HumanPercent::from(12.3456, 0).concise(), "12%");
    /// assert_eq!(HumanPercent::from(12.3456, 1).concise(), "12.3%");
    /// assert_eq!(HumanPercent::from(12.3456, 2).concise(), "12.35%");
    /// ```
    pub fn concise(&self) -> String {
        self.format(HumanFormat::Concise)
    }

    fn full(&self) -> String {
        self.format(HumanFormat::Full)
    }

    fn format(&self, format: HumanFormat) -> String {
        let multiplier = 10_f64.powi(self.decimals as i32);
        let rounded = (self.value * multiplier).round() / multiplier;

        if !rounded.is_finite() {
            return "-".to_string();
        }
        match format {
            HumanFormat::Concise => format!("{}%", rounded),
            HumanFormat::Full => format!("{} percent", rounded),
        }
    }
}
