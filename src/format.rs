/// Format a number for display (incremental game style)
/// - Below 1000: show as-is with appropriate decimal places
/// - 1000 to 999,999: comma separators (e.g., 24,900)
/// - Million+: named suffixes (million, billion, etc.)
/// - Beyond decillion: scientific notation
pub fn format_energy(value: f64) -> String {
    const SUFFIXES: &[&str] = &[
        "",
        "",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
        "sextillion",
        "septillion",
        "octillion",
        "nonillion",
        "decillion",
    ];

    if value.is_nan() || value.is_infinite() {
        return "???".to_string();
    }

    let abs_value = value.abs();

    if abs_value < 1000.0 {
        // Small numbers: show with appropriate decimals
        if abs_value < 10.0 {
            format!("{:.2}", value)
        } else if abs_value < 100.0 {
            format!("{:.1}", value)
        } else {
            format!("{:.0}", value)
        }
    } else if abs_value < 1_000_000.0 {
        // Thousands: show with comma separators (no decimals)
        format_with_commas(value as i64)
    } else {
        // Million+: use named suffixes
        let tier = (abs_value.log10() / 3.0).floor() as usize;

        if tier < SUFFIXES.len() {
            let divisor = 10f64.powi((tier * 3) as i32);
            let scaled = value / divisor;
            let suffix = SUFFIXES[tier];
            let plural = if scaled.abs() == 1.0 || suffix.is_empty() {
                ""
            } else {
                "s"
            };
            format!("{:.3} {}{}", scaled, suffix, plural)
        } else {
            // Fall back to scientific notation for very large numbers
            let exponent = abs_value.log10().floor() as i32;
            let mantissa = value / 10f64.powi(exponent);
            format!("{:.2}e{}", mantissa, exponent)
        }
    }
}

/// Format integer with comma separators (e.g., 1234567 -> "1,234,567")
fn format_with_commas(n: i64) -> String {
    let s = n.abs().to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    if n < 0 {
        result.push('-');
    }
    result.chars().rev().collect()
}

/// Format a cost with "⚛" suffix
pub fn format_cost(value: f64) -> String {
    format!("{} ⚛", format_energy(value))
}

/// Format energy per second rate
pub fn format_rate(value: f64) -> String {
    format!("{}/s", format_energy(value))
}

/// Format duration in human readable form
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        let mins = seconds / 60;
        let secs = seconds % 60;
        if secs > 0 {
            format!("{}m {}s", mins, secs)
        } else {
            format!("{}m", mins)
        }
    } else {
        let hours = seconds / 3600;
        let mins = (seconds % 3600) / 60;
        if mins > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}h", hours)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_small_numbers() {
        assert_eq!(format_energy(0.0), "0.00");
        assert_eq!(format_energy(1.5), "1.50");
        assert_eq!(format_energy(15.5), "15.5");
        assert_eq!(format_energy(150.0), "150");
        assert_eq!(format_energy(999.0), "999");
    }

    #[test]
    fn test_format_thousands_with_commas() {
        assert_eq!(format_energy(1000.0), "1,000");
        assert_eq!(format_energy(1500.0), "1,500");
        assert_eq!(format_energy(2500.0), "2,500");
        assert_eq!(format_energy(24900.0), "24,900");
        assert_eq!(format_energy(999999.0), "999,999");
    }

    #[test]
    fn test_format_named_suffixes() {
        assert_eq!(format_energy(1000000.0), "1.000 million");
        assert_eq!(format_energy(1500000.0), "1.500 millions");
        assert_eq!(format_energy(1000000000.0), "1.000 billion");
        assert_eq!(format_energy(3200000000.0), "3.200 billions");
        assert_eq!(format_energy(1e12), "1.000 trillion");
        assert_eq!(format_energy(2.1e15), "2.100 quadrillions");
        assert_eq!(format_energy(4.5e18), "4.500 quintillions");
        assert_eq!(format_energy(1.2e21), "1.200 sextillions");
        assert_eq!(format_energy(3.0e24), "3.000 septillions");
        assert_eq!(format_energy(2.5e27), "2.500 octillions");
        assert_eq!(format_energy(1.8e30), "1.800 nonillions");
        assert_eq!(format_energy(5.0e33), "5.000 decillions");
    }

    #[test]
    fn test_format_very_large_numbers() {
        // Beyond decillion, fall back to scientific notation
        let result = format_energy(1.5e36);
        assert!(
            result.contains("e36"),
            "Expected scientific notation for 1.5e36, got: {}",
            result
        );
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3600), "1h");
        assert_eq!(format_duration(3720), "1h 2m");
    }
}
