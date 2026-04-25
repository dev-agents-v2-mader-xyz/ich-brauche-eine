use crate::types::DEFAULT_DAILY_LIMIT;

/// Returns CSS color class based on caffeine level vs limit.
pub fn caffeine_color(total: i32, limit: i32) -> &'static str {
    let limit = if limit <= 0 { DEFAULT_DAILY_LIMIT } else { limit };
    if total > limit * 350 / 400 {
        "danger"
    } else if total >= limit / 2 {
        "warning"
    } else {
        "safe"
    }
}

/// Clamps progress to 0.0–1.0 range.
pub fn progress_fraction(total: i32, limit: i32) -> f64 {
    let limit = if limit <= 0 { DEFAULT_DAILY_LIMIT } else { limit };
    let f = total as f64 / limit as f64;
    f.clamp(0.0, 1.0)
}

/// Formats an ISO timestamp string to HH:MM.
pub fn format_time(ts: &str) -> String {
    // ts is like "2026-04-25T14:30:00Z" or "2026-04-25T14:30:00+00:00"
    if let Some(t) = ts.find('T') {
        let time_part = &ts[t + 1..];
        let hm = &time_part[..time_part.len().min(5)];
        return hm.to_string();
    }
    ts.to_string()
}

/// Formats a YYYY-MM-DD string to "25. Apr".
pub fn format_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return date.to_string();
    }
    let month = match parts[1] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mär",
        "04" => "Apr",
        "05" => "Mai",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Okt",
        "11" => "Nov",
        "12" => "Dez",
        _ => parts[1],
    };
    format!("{}. {}", parts[2].trim_start_matches('0'), month)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_fraction_clamps() {
        assert_eq!(progress_fraction(0, 400), 0.0);
        assert_eq!(progress_fraction(400, 400), 1.0);
        assert_eq!(progress_fraction(500, 400), 1.0);
        assert!((progress_fraction(200, 400) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn caffeine_color_thresholds() {
        assert_eq!(caffeine_color(0, 400), "safe");
        assert_eq!(caffeine_color(199, 400), "safe");
        assert_eq!(caffeine_color(200, 400), "warning");
        assert_eq!(caffeine_color(351, 400), "danger");
    }

    #[test]
    fn format_time_parses_iso() {
        assert_eq!(format_time("2026-04-25T14:30:00Z"), "14:30");
        assert_eq!(format_time("2026-04-25T09:05:00+00:00"), "09:05");
    }

    #[test]
    fn format_date_parses() {
        assert_eq!(format_date("2026-04-25"), "25. Apr");
        assert_eq!(format_date("2026-01-01"), "1. Jan");
    }
}
