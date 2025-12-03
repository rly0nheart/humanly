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

// human_display!(HumanCount);
human_display!(HumanSize);
human_display!(HumanDuration);
human_display!(HumanTime);
human_display!(HumanPercent);


pub struct HumanCount;

impl HumanCount {
    pub fn format(number: impl Into<f64>) -> String {
        let number = number.into();
        let s = format!("{}", number);
        let mut parts = s.split('.');
        let int_part = parts.next().unwrap_or_default();
        let frac_part = parts.next();

        // Format integer part with commas
        let mut result = String::with_capacity(int_part.len() + int_part.len() / 3);
        let mut count = 0;
        for character in int_part.chars().rev() {
            if count != 0 && count % 3 == 0 {
                result.push(',');
            }
            result.push(character);
            count += 1;
        }
        let formatted_int: String = result.chars().rev().collect();

        // Append fractional part if exists
        if let Some(frac) = frac_part {
            format!("{}.{}", formatted_int, frac)
        } else {
            formatted_int
        }
    }
}

/* -------------------- HumanSize -------------------- */

#[derive(Clone, Copy, Debug)]
enum UnitSystem {
    Binary,  // IEC, 1024-based
    Decimal, // SI, 1000-based
}

#[derive(Clone, Copy, Debug)]
pub struct HumanSize {
    bytes: u64,
    system: UnitSystem,
}

impl HumanSize {
    pub fn from(bytes: u64) -> Self {
        Self { bytes, system: UnitSystem::Binary }
    }

    pub fn decimal(mut self) -> Self {
        self.system = UnitSystem::Decimal;
        self
    }

    pub fn binary(mut self) -> Self {
        self.system = UnitSystem::Binary;
        self
    }

    pub fn concise(&self) -> String {
        self.format(HumanFormat::Concise)
    }

    pub fn full(&self) -> String {
        self.format(HumanFormat::Full)
    }

    fn format(&self, format: HumanFormat) -> String {
        // Unit arrays
        let (units_short, units_full, step) = match self.system {
            UnitSystem::Binary => (
                ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"],
                ["byte", "kibibyte", "mebibyte", "gibibyte", "tebibyte", "pebibyte", "exbibyte", "zebibyte", "yobibyte"],
                1024.0,
            ),
            UnitSystem::Decimal => (
                ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"],
                ["byte", "kilobyte", "megabyte", "gigabyte", "terabyte", "petabyte", "exabyte", "zettabyte", "yottabyte"],
                1000.0,
            ),
        };

        let mut size = self.bytes as f64;
        let mut idx = 0;

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
                let pluralized = if rounded == 1.0 { unit.to_string() } else { format!("{}s", unit) };
                format!("{} {}", formatted, pluralized)
            }
        }
    }
}


/* -------------------- HumanDuration -------------------- */

pub struct HumanDuration {
    system_time: Option<SystemTime>,
}

impl HumanDuration {
    pub fn from(system_time: Option<SystemTime>) -> Self {
        Self { system_time }
    }

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

pub struct HumanTime {
    duration: Duration,
}

impl HumanTime {
    pub fn from(duration: Duration) -> Self {
        Self { duration }
    }

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

pub struct HumanPercent {
    value: f64,
    decimals: usize,
}

impl HumanPercent {
    pub fn from(value: f64, decimals: usize) -> Self {
        Self { value, decimals }
    }

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