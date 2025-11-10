use std::time::{Duration, SystemTime};

#[derive(Clone, Copy)]
enum OutputFormat {
    Concise,
    Full,
}

pub struct HumanCount {
    number: u64,
}

impl HumanCount {
    pub fn from(number: u64) -> Self {
        Self { number }
    }

    pub fn concise(&self) -> String {
        self.format(OutputFormat::Concise)
    }

    pub fn full(&self) -> String {
        self.format(OutputFormat::Full)
    }

    fn format(&self, format: OutputFormat) -> String {
        let number = self.number;
        let format_val = |val: f64, concise_suffix: &str, full_suffix: &str| {
            let rounded = (val * 10.0).round() / 10.0;
            let formatted = if rounded.fract() == 0.0 {
                format!("{}", rounded as u64)
            } else {
                format!("{:.1}", rounded)
            };
            match format {
                OutputFormat::Concise => format!("{}{}", formatted, concise_suffix),
                OutputFormat::Full => format!("{} {}", formatted, full_suffix),
            }
        };

        if number >= 1_000_000_000_000_000_000 {
            return format_val(number as f64 / 1e18, "Qi", "quintillion");
        }
        if number >= 1_000_000_000_000_000 {
            return format_val(number as f64 / 1e15, "Q", "quadrillion");
        }
        if number >= 1_000_000_000_000 {
            return format_val(number as f64 / 1e12, "T", "trillion");
        }
        if number >= 1_000_000_000 {
            return format_val(number as f64 / 1e9, "B", "billion");
        }
        if number >= 1_000_000 {
            return format_val(number as f64 / 1e6, "M", "million");
        }
        if number >= 1_000 {
            return format_val(number as f64 / 1e3, "K", "thousand");
        }

        number.to_string()
    }
}

/* -------------------- HumanSize -------------------- */

pub struct HumanSize {
    bytes: u64,
}

impl HumanSize {
    pub fn from(bytes: u64) -> Self {
        Self { bytes }
    }

    pub fn concise(&self) -> String {
        self.format(OutputFormat::Concise)
    }
    
    pub fn full(&self) -> String {
        self.format(OutputFormat::Full)
    }

    fn format(&self, format: OutputFormat) -> String {
        let concise_units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
        let full_units = [
            "bytes",
            "kibibytes",
            "mebibytes",
            "gibibytes",
            "tebibytes",
            "pebibytes",
            "exbibytes",
            "zebibytes",
            "yobibytes",
        ];
        let mut size = self.bytes as f64;
        let mut idx = 0;

        while size >= 1024.0 && idx < concise_units.len() - 1 {
            size /= 1024.0;
            idx += 1;
        }

        let rounded = (size * 10.0).round() / 10.0;
        let formatted = if rounded.fract() == 0.0 {
            format!("{}", rounded as u64)
        } else {
            format!("{:.1}", rounded)
        };

        match format {
            OutputFormat::Concise => format!("{} {}", formatted, concise_units[idx]),
            OutputFormat::Full => {
                let unit = full_units[idx];
                if formatted == "1" && unit.ends_with('s') {
                    format!("{} {}", formatted, &unit[..unit.len() - 1])
                } else {
                    format!("{} {}", formatted, unit)
                }
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
        self.format(OutputFormat::Concise)
    }

    pub fn full(&self) -> String {
        self.format(OutputFormat::Full)
    }

    fn format(&self, format: OutputFormat) -> String {
        let now = SystemTime::now();
        if let Some(st) = self.system_time {
            let elapsed = now.duration_since(st).unwrap_or_else(|_| Duration::ZERO);
            let secs = elapsed.as_secs();

            if secs < 10 {
                return "just now".to_string();
            }

            let (count, concise_suffix, singular, plural) = if secs < 60 {
                (secs, "s ago", "second", "seconds")
            } else if secs < 3600 {
                (secs / 60, "m ago", "minute", "minutes")
            } else if secs < 86_400 {
                (secs / 3600, "h ago", "hour", "hours")
            } else if secs < 604_800 {
                (secs / 86_400, "d ago", "day", "days")
            } else if secs < 2_629_746 {
                (secs / 604_800, "wk ago", "week", "weeks")
            } else if secs < 31_556_952 {
                (secs / 2_629_746, "mo ago", "month", "months")
            } else {
                (secs / 31_556_952, "yr ago", "year", "years")
            };

            match format {
                OutputFormat::Concise => format!("{}{}", count, concise_suffix),
                OutputFormat::Full => {
                    if count == 1 {
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
        self.format(OutputFormat::Concise)
    }
    
    pub fn full(&self) -> String {
        self.format(OutputFormat::Full)
    }

    fn format(&self, format: OutputFormat) -> String {
        let secs = self.duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        match format {
            OutputFormat::Concise => {
                if hours > 0 {
                    format!("{}h {}m {}s", hours, minutes, seconds)
                } else if minutes > 0 {
                    format!("{}m {}s", minutes, seconds)
                } else {
                    format!("{}s", seconds)
                }
            }
            OutputFormat::Full => {
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
        self.format(OutputFormat::Concise)
    }

    pub fn full(&self) -> String {
        self.format(OutputFormat::Full)
    }

    fn format(&self, format: OutputFormat) -> String {
        let multiplier = 10_f64.powi(self.decimals as i32);
        let rounded = (self.value * multiplier).round() / multiplier;

        match format {
            OutputFormat::Concise => format!("{}%", rounded),
            OutputFormat::Full => format!("{} percent", rounded),
        }
    }
}
