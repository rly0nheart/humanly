mod models;
pub use models::HumanCount;
pub use models::HumanDuration;
pub use models::HumanPercent;
pub use models::HumanPermissions;
pub use models::HumanSize;
pub use models::HumanTime;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::models::{HumanCount, HumanDuration, HumanPercent, HumanPermissions, HumanSize, HumanTime};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_human_count() {
        assert_eq!(HumanCount::from(500), "500");
        assert_eq!(HumanCount::from(1_000), "1K");
        assert_eq!(HumanCount::from(1_500), "1.5K");
        assert_eq!(HumanCount::from(1_000_000), "1M");
        assert_eq!(HumanCount::from(1_500_000), "1.5M");
        assert_eq!(HumanCount::from(1_000_000_000), "1B");
        assert_eq!(HumanCount::from(1_500_000_000), "1.5B");
    }

    #[test]
    fn test_human_size() {
        assert_eq!(HumanSize::from(500), "500 B");
        assert_eq!(HumanSize::from(1024), "1 KiB");
        assert_eq!(HumanSize::from(1_048_576), "1 MiB");
        assert_eq!(HumanSize::from(1_500_000), "1.4 MiB");
        assert_eq!(HumanSize::from(1_073_741_824), "1 GiB");
    }

    #[test]
    fn test_human_time() {
        let now = SystemTime::now();
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(5))), "just now");
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(45))), "45s ago");
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(120))), "2m ago");
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(7200))), "2h ago");
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(172_800))), "2d ago");
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(1_209_600))), "2wk ago");
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(5_259_492))), "2mo ago");
        assert_eq!(HumanDuration::from(Some(now - Duration::from_secs(63_113_904))), "2yr ago");
    }

    #[test]
    fn test_human_duration() {
        assert_eq!(HumanTime::from(Duration::from_secs(45)), "45s");
        assert_eq!(HumanTime::from(Duration::from_secs(90)), "1m 30s");
        assert_eq!(HumanTime::from(Duration::from_secs(3672)), "1h 1m 12s");
    }

    #[test]
    fn test_human_percent() {
        assert_eq!(HumanPercent::from(12.3456, 0), "12%");
        assert_eq!(HumanPercent::from(12.3456, 1), "12.3%");
        assert_eq!(HumanPercent::from(12.3456, 2), "12.35%");
        assert_eq!(HumanPercent::from(0.1234 * 100.0, 1), "12.3%");
    }

    #[test]
    #[cfg(unix)]
    fn test_unix_permissions() {
        assert_eq!(HumanPermissions::from(0o40755), "drwxr-xr-x");
        assert_eq!(HumanPermissions::from(0o100644), "-rw-r--r--");
        assert_eq!(HumanPermissions::from(0o100755), "-rwxr-xr-x");
    }

    #[test]
    #[cfg(windows)]
    fn test_windows_permissions() {
        assert_eq!(
            HumanPermissions::from(0o40755),
            "User: Read, Write, Execute; Group: Read, Execute; Other: Read, Execute"
        );

        assert_eq!(
            HumanPermissions::from(0o100644);,
            "User: Read, Write; Group: Read; Other: Read"
        );

        assert_eq!(
            HumanPermissions::from(0o100755),
            "User: Read, Write, Execute; Group: Read, Execute; Other: Read, Execute"
        );
    }
}

