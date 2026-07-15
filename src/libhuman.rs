use std::fmt;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, Debug)]
enum HumanFormat {
    Short,
    Long,
}

/// A lazily-formatted, ready-to-display view of a humanised value.
///
/// Returned by the `short` and `long` methods on the `Human*` types. It
/// implements [`Display`](fmt::Display) and writes straight into the formatter,
/// so `write!`/`println!` allocate nothing. Call
/// [`to_string`](ToString::to_string) when you actually need an
/// owned [`String`].
///
/// # Examples
///
/// ```
/// use human::HumanNumber;
///
/// // Format with no intermediate allocation:
/// assert_eq!(format!("{}", HumanNumber::new(1_500).short()), "1.5k");
///
/// // Or materialize a String when you need one:
/// assert_eq!(HumanNumber::new(1_500).short().to_string(), "1.5k");
/// ```
#[derive(Clone, Copy, Debug)]
pub struct HumanDisplay<T> {
    value: T,
    format: HumanFormat,
}

/// Internal formatting hook implemented by every `Human*` type.
///
/// Kept private so it cannot be implemented downstream; [`HumanDisplay`] is the
/// only public surface.
trait Humanize: Copy {
    fn fmt_human(&self, f: &mut fmt::Formatter<'_>, format: HumanFormat) -> fmt::Result;

    /// Wraps `self` in a [`HumanDisplay`] for the given format. Backs the
    /// `short` / `long` methods so their construction lives in one place.
    fn view(&self, format: HumanFormat) -> HumanDisplay<Self> {
        HumanDisplay {
            value: *self,
            format,
        }
    }
}

impl<T: Humanize> fmt::Display for HumanDisplay<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt_human(f, self.format)
    }
}

macro_rules! human_display {
    ($t:ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.fmt_human(f, HumanFormat::Long)
            }
        }
    };
}

human_display!(HumanNumber);
human_display!(HumanSize);
human_display!(HumanDuration);
human_display!(HumanRelative);
human_display!(HumanPercent);

/// Writes `value` rounded to at most one decimal place, dropping a trailing
/// `.0` so whole numbers render without a fractional part.
fn write_number(f: &mut fmt::Formatter<'_>, value: f64) -> fmt::Result {
    let rounded = (value * 10.0).round() / 10.0;
    if rounded.fract() == 0.0 {
        write!(f, "{}", rounded as i64)
    } else {
        write!(f, "{:.1}", rounded)
    }
}

/* -------------------- HumanNumber -------------------- */

/// Humanizes large numbers into readable abbreviated or word formats.
///
/// Converts numeric values into short notation (e.g. `"1.8k"`, `"2.5B"`) or
/// long word format (e.g. `"1.8 thousand"`, `"2.5 billion"`).
///
/// The [`Display`](fmt::Display) implementation outputs the [`long`](HumanNumber::long) format.
///
/// # Examples
///
/// ```
/// use human::HumanNumber;
///
/// assert_eq!(HumanNumber::new(1_500).short().to_string(), "1.5k");
/// assert_eq!(HumanNumber::new(1_500).long().to_string(), "1.5 thousand");
/// assert_eq!(HumanNumber::new(2_500_000_000.0).to_string(), "2.5 billion");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HumanNumber {
    number: f64,
}

impl HumanNumber {
    /// Creates a new `HumanNumber` from any value convertible to `f64`.
    ///
    /// Concrete numeric types also convert via [`From`]/[`Into`]
    /// (e.g. `let n: HumanNumber = 1_000_i32.into();`); prefer `new` for
    /// unsuffixed integer literals, whose type would otherwise be ambiguous.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanNumber;
    ///
    /// let from_int = HumanNumber::new(1_000);
    /// let from_float = HumanNumber::new(2_500_000_000.0);
    /// let via_into: HumanNumber = 1_000_i32.into();
    /// ```
    pub fn new(number: impl Into<f64>) -> Self {
        Self {
            number: number.into(),
        }
    }

    /// Returns a [`Display`](fmt::Display) view in short (abbreviated) form.
    ///
    /// Uses short suffixes: `k` (thousand), `M` (million), `B` (billion), `T` (trillion).
    /// Numbers below 1,000 are rendered without a suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanNumber;
    ///
    /// assert_eq!(HumanNumber::new(500).short().to_string(), "500");
    /// assert_eq!(HumanNumber::new(1_200).short().to_string(), "1.2k");
    /// assert_eq!(HumanNumber::new(1_800_000).short().to_string(), "1.8M");
    /// assert_eq!(HumanNumber::new(1_000_000_000).short().to_string(), "1B");
    /// assert_eq!(HumanNumber::new(3_700_000_000_000.0).short().to_string(), "3.7T");
    /// ```
    pub fn short(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Short)
    }

    /// Returns a [`Display`](fmt::Display) view in long (spelled-out) form.
    ///
    /// Uses word suffixes: `thousand`, `million`, `billion`, `trillion`.
    /// Numbers below 1,000 are rendered without a suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanNumber;
    ///
    /// assert_eq!(HumanNumber::new(500).long().to_string(), "500");
    /// assert_eq!(HumanNumber::new(1_200).long().to_string(), "1.2 thousand");
    /// assert_eq!(HumanNumber::new(1_800_000).long().to_string(), "1.8 million");
    /// assert_eq!(HumanNumber::new(1_000_000_000).long().to_string(), "1 billion");
    /// assert_eq!(HumanNumber::new(3_700_000_000_000.0).long().to_string(), "3.7 trillion");
    /// ```
    pub fn long(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Long)
    }
}

impl Humanize for HumanNumber {
    fn fmt_human(&self, f: &mut fmt::Formatter<'_>, format: HumanFormat) -> fmt::Result {
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

        write_number(f, number / divisor)?;
        match format {
            HumanFormat::Short => f.write_str(short_suffix),
            HumanFormat::Long => f.write_str(long_suffix),
        }
    }
}

macro_rules! human_number_from {
    ($($t:ty),* $(,)?) => {
        $(impl From<$t> for HumanNumber {
            fn from(number: $t) -> Self {
                Self { number: number as f64 }
            }
        })*
    };
}
human_number_from!(i8, i16, i32, u8, u16, u32, f32, f64);

/* -------------------- HumanSize -------------------- */

#[derive(Clone, Copy, Debug, PartialEq)]
enum UnitSystem {
    Binary,  // IEC, 1024-based
    Decimal, // SI, 1000-based
}

/// Humanizes byte counts into readable file-size strings.
///
/// Supports both binary (IEC, 1024-based: KiB, MiB, GiB…) and decimal
/// (SI, 1000-based: kB, MB, GB…) unit systems. Defaults to binary.
///
/// Use the [`decimal`](HumanSize::decimal) and [`binary`](HumanSize::binary) methods
/// to switch between unit systems via method chaining.
///
/// The [`Display`](fmt::Display) implementation outputs the [`long`](HumanSize::long) format.
///
/// # Examples
///
/// ```
/// use human::HumanSize;
///
/// // Binary (default)
/// assert_eq!(HumanSize::new(5_242_880).short().to_string(), "5 MiB");
/// assert_eq!(HumanSize::new(5_242_880).long().to_string(), "5 mebibytes");
///
/// // Decimal
/// assert_eq!(HumanSize::new(5_000_000).decimal().short().to_string(), "5 MB");
/// assert_eq!(HumanSize::new(5_000_000).decimal().long().to_string(), "5 megabytes");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HumanSize {
    bytes: u64,
    system: UnitSystem,
}

impl From<u64> for HumanSize {
    fn from(bytes: u64) -> Self {
        Self::new(bytes)
    }
}

impl HumanSize {
    /// Creates a new `HumanSize` from a byte count, defaulting to binary (IEC) units.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanSize;
    ///
    /// let size = HumanSize::new(1_048_576);
    /// assert_eq!(size.short().to_string(), "1 MiB");
    /// ```
    pub fn new(bytes: u64) -> Self {
        Self {
            bytes,
            system: UnitSystem::Binary,
        }
    }

    /// Switches to the decimal (SI, 1000-based) unit system.
    ///
    /// Units used: kB, MB, GB, TB, PB, EB, ZB, YB.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanSize;
    ///
    /// assert_eq!(HumanSize::new(5_000_000).decimal().short().to_string(), "5 MB");
    /// assert_eq!(HumanSize::new(5_000_000).decimal().long().to_string(), "5 megabytes");
    /// ```
    pub fn decimal(mut self) -> Self {
        self.system = UnitSystem::Decimal;
        self
    }

    /// Switches to the binary (IEC, 1024-based) unit system.
    ///
    /// This is the default, but is useful when chaining after a previous call to
    /// [`decimal`](HumanSize::decimal). Units used: KiB, MiB, GiB, TiB, PiB, EiB, ZiB, YiB.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanSize;
    ///
    /// assert_eq!(HumanSize::new(1_000_000).binary().short().to_string(), "976.6 KiB");
    /// ```
    pub fn binary(mut self) -> Self {
        self.system = UnitSystem::Binary;
        self
    }

    /// Returns a [`Display`](fmt::Display) view with abbreviated unit suffixes.
    ///
    /// Values below 1,024 bytes are rendered with the `B` unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanSize;
    ///
    /// assert_eq!(HumanSize::new(500).short().to_string(), "500 B");
    /// assert_eq!(HumanSize::new(1024).short().to_string(), "1 KiB");
    /// assert_eq!(HumanSize::new(5_242_880).short().to_string(), "5 MiB");
    /// assert_eq!(HumanSize::new(5_000_000).decimal().short().to_string(), "5 MB");
    /// ```
    pub fn short(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Short)
    }

    /// Returns a [`Display`](fmt::Display) view with pluralised unit names.
    ///
    /// Values below 1,024 bytes are rendered as `"N byte"` or `"N bytes"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanSize;
    ///
    /// assert_eq!(HumanSize::new(1).long().to_string(), "1 byte");
    /// assert_eq!(HumanSize::new(500).long().to_string(), "500 bytes");
    /// assert_eq!(HumanSize::new(1024).long().to_string(), "1 kibibyte");
    /// assert_eq!(HumanSize::new(5_242_880).long().to_string(), "5 mebibytes");
    /// assert_eq!(HumanSize::new(5_000_000).decimal().long().to_string(), "5 megabytes");
    /// ```
    pub fn long(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Long)
    }
}

impl Humanize for HumanSize {
    fn fmt_human(&self, f: &mut fmt::Formatter<'_>, format: HumanFormat) -> fmt::Result {
        // Below one KiB/kB, render the raw byte count.
        if self.bytes < 1024 {
            return match format {
                HumanFormat::Short => write!(f, "{} B", self.bytes),
                HumanFormat::Long => match self.bytes {
                    1 => f.write_str("1 byte"),
                    n => write!(f, "{} bytes", n),
                },
            };
        }

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

        let mut size = self.bytes as f64 / step;
        let mut idx = 0;
        while size >= step && idx < units_short.len() - 1 {
            size /= step;
            idx += 1;
        }

        let rounded = (size * 10.0).round() / 10.0;
        write_number(f, rounded)?;
        match format {
            HumanFormat::Short => write!(f, " {}", units_short[idx]),
            HumanFormat::Long if rounded == 1.0 => write!(f, " {}", units_full[idx]),
            HumanFormat::Long => write!(f, " {}s", units_full[idx]),
        }
    }
}

/* -------------------- HumanDuration -------------------- */

/// Humanizes a [`Duration`] into an hours/minutes/seconds string.
///
/// Breaks a duration into its hour, minute, and second components and formats
/// them in either short (e.g. `"1h 5m 30s"`) or long
/// (e.g. `"1 hour 5 minutes 30 seconds"`) form.
///
/// For a relative "time ago / from now" string, see [`HumanRelative`].
///
/// The [`Display`](fmt::Display) implementation outputs the [`long`](HumanDuration::long) format.
///
/// # Examples
///
/// ```
/// use human::HumanDuration;
/// use std::time::Duration;
///
/// assert_eq!(HumanDuration::new(Duration::from_secs(3661)).short().to_string(), "1h 1m 1s");
/// assert_eq!(HumanDuration::new(Duration::from_secs(3661)).to_string(), "1 hour 1 minute 1 second");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HumanDuration {
    duration: Duration,
}

impl From<Duration> for HumanDuration {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

impl HumanDuration {
    /// Creates a new `HumanDuration` from a [`Duration`].
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanDuration;
    /// use std::time::Duration;
    ///
    /// let time = HumanDuration::new(Duration::from_secs(90));
    /// assert_eq!(time.short().to_string(), "1m 30s");
    /// ```
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    /// Returns a [`Display`](fmt::Display) view in short form.
    ///
    /// Uses short suffixes: `h` (hours), `m` (minutes), `s` (seconds).
    /// Zero-value components are omitted where possible, though minutes are
    /// always shown when hours are present.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(HumanDuration::new(Duration::from_secs(45)).short().to_string(), "45s");
    /// assert_eq!(HumanDuration::new(Duration::from_secs(90)).short().to_string(), "1m 30s");
    /// assert_eq!(HumanDuration::new(Duration::from_secs(3661)).short().to_string(), "1h 1m 1s");
    /// ```
    pub fn short(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Short)
    }

    /// Returns a [`Display`](fmt::Display) view in long (spelled-out) form.
    ///
    /// Uses long-form, pluralised words for each non-zero component
    /// (e.g. `"1 hour 5 minutes 30 seconds"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanDuration;
    /// use std::time::Duration;
    ///
    /// assert_eq!(HumanDuration::new(Duration::from_secs(3661)).long().to_string(), "1 hour 1 minute 1 second");
    /// ```
    pub fn long(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Long)
    }
}

impl Humanize for HumanDuration {
    fn fmt_human(&self, f: &mut fmt::Formatter<'_>, format: HumanFormat) -> fmt::Result {
        let secs = self.duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        let mut wrote = false;
        macro_rules! sep {
            () => {
                if wrote {
                    f.write_str(" ")?;
                }
            };
        }

        match format {
            HumanFormat::Short => {
                if hours > 0 {
                    write!(f, "{}h", hours)?;
                    wrote = true;
                }
                if minutes > 0 || hours > 0 {
                    sep!();
                    write!(f, "{}m", minutes)?;
                    wrote = true;
                }
                if seconds > 0 || !wrote {
                    sep!();
                    write!(f, "{}s", seconds)?;
                }
            }
            HumanFormat::Long => {
                if hours > 0 {
                    write!(f, "{} {}", hours, if hours == 1 { "hour" } else { "hours" })?;
                    wrote = true;
                }
                if minutes > 0 {
                    sep!();
                    write!(
                        f,
                        "{} {}",
                        minutes,
                        if minutes == 1 { "minute" } else { "minutes" }
                    )?;
                    wrote = true;
                }
                if seconds > 0 || !wrote {
                    sep!();
                    write!(
                        f,
                        "{} {}",
                        seconds,
                        if seconds == 1 { "second" } else { "seconds" }
                    )?;
                }
            }
        }
        Ok(())
    }
}

/* -------------------- HumanRelative -------------------- */

/// Humanizes a [`SystemTime`] into a relative "time ago / from now" string.
///
/// Computes the elapsed time between the given timestamp and the current time,
/// then formats it as a human-readable string (e.g. `"45s ago"`, `"2 hours ago"`,
/// `"yesterday"`, `"3m from now"`). Handles both past and future timestamps.
///
/// To format a fixed [`Duration`] as H:M:S instead, see [`HumanDuration`].
///
/// The [`Display`](fmt::Display) implementation outputs the [`long`](HumanRelative::long) format.
///
/// # Examples
///
/// ```
/// use human::HumanRelative;
/// use std::time::{Duration, SystemTime};
///
/// let past = SystemTime::now() - Duration::from_secs(120);
/// assert_eq!(HumanRelative::new(past).short().to_string(), "2m ago");
///
/// // A missing timestamp is the caller's concern:
/// let maybe: Option<SystemTime> = None;
/// let shown = maybe.map(HumanRelative::new).map(|r| r.short().to_string());
/// assert_eq!(shown, None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HumanRelative {
    system_time: SystemTime,
}

impl From<SystemTime> for HumanRelative {
    fn from(system_time: SystemTime) -> Self {
        Self::new(system_time)
    }
}

impl HumanRelative {
    /// Creates a new `HumanRelative` from a [`SystemTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanRelative;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let relative = HumanRelative::new(SystemTime::now() - Duration::from_secs(60));
    /// ```
    pub fn new(system_time: SystemTime) -> Self {
        Self { system_time }
    }

    /// Returns a [`Display`](fmt::Display) view in short form.
    ///
    /// Uses short suffixes: `s` (seconds), `m` (minutes), `h` (hours), `d` (days),
    /// `w`/`wk` (weeks), `mo` (months), `y`/`yr` (years), followed by `ago` or `from now`.
    /// Renders `"just now"` when less than one second has elapsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanRelative;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let past = SystemTime::now() - Duration::from_secs(90);
    /// assert_eq!(HumanRelative::new(past).short().to_string(), "1m ago");
    /// ```
    pub fn short(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Short)
    }

    /// Returns a [`Display`](fmt::Display) view in long (spelled-out) form.
    ///
    /// Uses long-form words (e.g. `"2 minutes ago"`) with the special cases
    /// `"yesterday"`, `"tomorrow"`, and `"just now"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanRelative;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let past = SystemTime::now() - Duration::from_secs(7200);
    /// assert_eq!(HumanRelative::new(past).long().to_string(), "2 hours ago");
    /// ```
    pub fn long(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Long)
    }
}

impl Humanize for HumanRelative {
    fn fmt_human(&self, f: &mut fmt::Formatter<'_>, format: HumanFormat) -> fmt::Result {
        let now = SystemTime::now();
        let elapsed = match now.duration_since(self.system_time) {
            Ok(dur) => dur.as_secs() as i64,
            Err(err) => -(err.duration().as_secs() as i64),
        };

        if elapsed.abs() < 1 {
            return f.write_str("just now");
        }

        let (count, short_suffix, singular, plural) = if elapsed < 0 {
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
            HumanFormat::Short => write!(f, "{}{}", count, short_suffix),
            HumanFormat::Long => {
                if count == 1 && singular == "day" && elapsed >= 0 {
                    f.write_str("yesterday")
                } else if count == 1 && singular == "day" && elapsed < 0 {
                    f.write_str("tomorrow")
                } else {
                    let suffix = if elapsed >= 0 { "ago" } else { "from now" };
                    if count == 1 {
                        write!(f, "1 {} {}", singular, suffix)
                    } else {
                        write!(f, "{} {} {}", count, plural, suffix)
                    }
                }
            }
        }
    }
}

/* -------------------- HumanPercent -------------------- */

/// Humanizes a floating-point value into a percentage string.
///
/// Rounds to a configurable number of decimal places and formats the result as
/// either a short percentage (e.g. `"12.3%"`) or long words
/// (e.g. `"12.3 percent"`). Non-finite values (`NaN`, infinity) render as `"-"`.
///
/// The [`Display`](fmt::Display) implementation outputs the [`long`](HumanPercent::long) format.
///
/// # Examples
///
/// ```
/// use human::HumanPercent;
///
/// assert_eq!(HumanPercent::new(12.3456, 1).short().to_string(), "12.3%");
/// assert_eq!(HumanPercent::new(12.3456, 2).to_string(), "12.35 percent");
/// assert_eq!(HumanPercent::new(f64::NAN, 1).short().to_string(), "-");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HumanPercent {
    value: f64,
    decimals: usize,
}

impl HumanPercent {
    /// Creates a new `HumanPercent` from a value and decimal precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanPercent;
    ///
    /// let pct = HumanPercent::new(99.999, 1);
    /// assert_eq!(pct.short().to_string(), "100%");
    /// ```
    pub fn new(value: f64, decimals: usize) -> Self {
        Self { value, decimals }
    }

    /// Returns a [`Display`](fmt::Display) view with a `%` symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanPercent;
    ///
    /// assert_eq!(HumanPercent::new(12.3456, 0).short().to_string(), "12%");
    /// assert_eq!(HumanPercent::new(12.3456, 1).short().to_string(), "12.3%");
    /// assert_eq!(HumanPercent::new(12.3456, 2).short().to_string(), "12.35%");
    /// ```
    pub fn short(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Short)
    }

    /// Returns a [`Display`](fmt::Display) view spelled out with `percent`.
    ///
    /// # Examples
    ///
    /// ```
    /// use human::HumanPercent;
    ///
    /// assert_eq!(HumanPercent::new(12.3456, 1).long().to_string(), "12.3 percent");
    /// ```
    pub fn long(&self) -> HumanDisplay<Self> {
        self.view(HumanFormat::Long)
    }
}

impl Humanize for HumanPercent {
    fn fmt_human(&self, f: &mut fmt::Formatter<'_>, format: HumanFormat) -> fmt::Result {
        let multiplier = 10_f64.powi(self.decimals as i32);
        let rounded = (self.value * multiplier).round() / multiplier;

        if !rounded.is_finite() {
            return f.write_str("-");
        }
        match format {
            HumanFormat::Short => write!(f, "{}%", rounded),
            HumanFormat::Long => write!(f, "{} percent", rounded),
        }
    }
}
