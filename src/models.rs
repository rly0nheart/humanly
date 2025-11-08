use std::time::{Duration, SystemTime};

pub struct HumanCount;


impl HumanCount {
    pub fn from(number: u64) -> String {
        let format_val = |val: f64, suffix: &str| {
            let rounded = (val * 10.0).round() / 10.0; // round to 1 decimal
            if rounded.fract() == 0.0 {
                format!("{}{}", rounded as u64, suffix)
            } else {
                format!("{:.1}{}", rounded, suffix)
            }
        };

        if number >= 1_000_000_000_000_000_000 {
            // quintillion
            return format_val(number as f64 / 1_000_000_000_000_000_000.0, "Qi");
        }
        if number >= 1_000_000_000_000_000 {
            // quadrillion
            return format_val(number as f64 / 1_000_000_000_000_000.0, "Q");
        }
        if number >= 1_000_000_000_000 {
            // trillion
            return format_val(number as f64 / 1_000_000_000_000.0, "T");
        }
        if number >= 1_000_000_000 {
            // billion
            return format_val(number as f64 / 1_000_000_000.0, "B");
        }
        if number >= 1_000_000 {
            // million
            return format_val(number as f64 / 1_000_000.0, "M");
        }
        if number >= 1_000 {
            // thousand
            return format_val(number as f64 / 1_000.0, "K");
        }

        number.to_string()
    }
}

pub struct HumanSize;

impl HumanSize {
    pub fn from(bytes: u64) -> String {
        let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
        let mut size = bytes as f64;
        let mut idx = 0;

        while size >= 1024.0 && idx < units.len() - 1 {
            size /= 1024.0;
            idx += 1;
        }

        let rounded = (size * 10.0).round() / 10.0;
        if rounded.fract() == 0.0 {
            format!("{} {}", rounded as u64, units[idx])
        } else {
            format!("{:.1} {}", rounded, units[idx])
        }
    }
}

pub struct HumanDuration;

impl HumanDuration {
    pub fn from(system_time: Option<SystemTime>) -> String {
        let now = SystemTime::now();
        if let Some(system_time) = system_time {
            let elapsed = now.duration_since(system_time).unwrap_or_else(|_| Duration::ZERO);
            let secs = elapsed.as_secs();

            if secs < 10 {
                "just now".to_string()
            } else if secs < 60 {
                format!("{}s ago", secs)
            } else if secs < 3600 {
                format!("{}m ago", secs / 60)
            } else if secs < 86_400 {
                format!("{}h ago", secs / 3600)
            } else if secs < 604_800 {
                format!("{}d ago", secs / 86_400)
            } else if secs < 2_629_746 {
                format!("{}wk ago", secs / 604_800)
            } else if secs < 31_556_952 {
                format!("{}mo ago", secs / 2_629_746)
            } else {
                format!("{}yr ago", secs / 31_556_952)
            }
        } else {
            "-".to_string()
        }
    }
}

pub struct HumanTime;

impl HumanTime {
    pub fn from(duration: Duration) -> String {
        let secs = duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

pub struct HumanPercent;

impl HumanPercent {
    pub fn from(value: f64, decimals: usize) -> String {
        let multiplier = 10_f64.powi(decimals as i32);
        let rounded = (value * multiplier).round() / multiplier;
        format!("{}%", rounded)
    }
}

pub struct HumanPermissions;

impl HumanPermissions {
    pub fn from(mode: u32) -> String {
        #[cfg(unix)]
        {
            unix_mode::to_string(mode)
        }

        #[cfg(windows)]
        {
            // Convert the same bits to Windows-style descriptive text
            let user_read = mode & 0o400 != 0;
            let user_write = mode & 0o200 != 0;
            let user_execute = mode & 0o100 != 0;

            let group_read = mode & 0o040 != 0;
            let group_write = mode & 0o020 != 0;
            let group_execute = mode & 0o010 != 0;

            let other_read = mode & 0o004 != 0;
            let other_write = mode & 0o002 != 0;
            let other_execute = mode & 0o001 != 0;

            let to_text = |r: bool, w: bool, x: bool| {
                let mut perms = Vec::new();
                if r { perms.push("Read"); }
                if w { perms.push("Write"); }
                if x { perms.push("Execute"); }
                if perms.is_empty() { perms.push("None"); }
                perms.join(", ")
            };

            format!(
                "User: {}; Group: {}; Other: {}",
                to_text(user_read, user_write, user_execute),
                to_text(group_read, group_write, group_execute),
                to_text(other_read, other_write, other_execute)
            )
        }
    }
}