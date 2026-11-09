//! Turning numbers into the short strings a task manager's columns hold.

/// A byte count in binary units, such as `512 MiB` or `1.4 GiB`.
///
/// A column of a few characters is what the table has room for, so the
/// count is scaled to the largest unit that keeps it under four digits.
pub fn bytes(value: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut scaled = value as f64;
    let mut unit = 0;
    while scaled >= 1024.0 && unit + 1 < UNITS.len() {
        scaled /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{value} B")
    } else if scaled < 10.0 {
        format!("{scaled:.2} {}", UNITS[unit])
    } else if scaled < 100.0 {
        format!("{scaled:.1} {}", UNITS[unit])
    } else {
        format!("{scaled:.0} {}", UNITS[unit])
    }
}

/// A byte rate, as `1.2 MiB/s`.
pub fn rate(bytes_per_second: f64) -> String {
    if bytes_per_second < 1.0 {
        "0 B/s".to_string()
    } else {
        format!("{}/s", bytes(bytes_per_second as u64))
    }
}

/// A percentage, as `12.3%`; below a tenth of a percent it reads `0.0%` so a
/// column of them stays aligned.
pub fn percent(value: f32) -> String {
    if !value.is_finite() {
        return "--".to_string();
    }
    let value = value.clamp(0.0, 999.9);
    format!("{value:.1}%")
}

/// A percentage used by a bar, clamped to what a bar can show.
pub fn fraction(part: u64, whole: u64) -> f32 {
    if whole == 0 {
        0.0
    } else {
        (part as f64 / whole as f64).clamp(0.0, 1.0) as f32
    }
}

/// A span of seconds as `1d 4h`, `3h 7m` or `12m 30s`.
pub fn duration(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let rest = seconds % 60;
    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else if minutes > 0 {
        format!("{minutes}m {rest}s")
    } else {
        format!("{rest}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_scale_to_the_largest_unit_that_fits() {
        assert_eq!(bytes(0), "0 B");
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(1024), "1.00 KiB");
        assert_eq!(bytes(1536), "1.50 KiB");
        assert_eq!(bytes(20 * 1024), "20.0 KiB");
        assert_eq!(bytes(200 * 1024), "200 KiB");
        assert_eq!(bytes(3 * 1024 * 1024 * 1024), "3.00 GiB");
    }

    #[test]
    fn rates_never_read_a_bare_zero() {
        assert_eq!(rate(0.0), "0 B/s");
        assert_eq!(rate(2048.0), "2.00 KiB/s");
    }

    #[test]
    fn percentages_keep_one_decimal() {
        assert_eq!(percent(0.0), "0.0%");
        assert_eq!(percent(12.34), "12.3%");
        assert_eq!(percent(f32::NAN), "--");
        assert_eq!(percent(-1.0), "0.0%");
    }

    #[test]
    fn a_fraction_of_nothing_is_nothing() {
        assert_eq!(fraction(1, 0), 0.0);
        assert_eq!(fraction(1, 2), 0.5);
        assert_eq!(fraction(5, 2), 1.0);
    }

    #[test]
    fn durations_keep_the_two_largest_units() {
        assert_eq!(duration(0), "0s");
        assert_eq!(duration(59), "59s");
        assert_eq!(duration(90), "1m 30s");
        assert_eq!(duration(3_600), "1h 0m");
        assert_eq!(duration(90_061), "1d 1h");
    }
}
