use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Temporal;

impl Temporal {
    /// Formatuje znacznik czasu UNIX podany w sekundach.
    pub fn format(timestamp_secs: u64, pattern: &str) -> String {
        Self::format_nanos(timestamp_secs as u128 * 1_000_000_000, pattern)
    }

    /// Formatuje znacznik czasu UNIX podany w milisekundach.
    pub fn format_millis(timestamp_millis: u64, pattern: &str) -> String {
        Self::format_nanos(timestamp_millis as u128 * 1_000_000, pattern)
    }

    /// Formatuje znacznik czasu UNIX podany w nanosekundach.
    pub fn format_nanos(timestamp_nanos: u128, pattern: &str) -> String {
        let dt = DateTimeUtc::from_total_nanos(timestamp_nanos);
        Self::replace_tokens(pattern, &dt)
    }

    /// Formatuje czas bezpośrednio ze struktury `std::time::SystemTime`.
    pub fn format_system_time(st: SystemTime, pattern: &str) -> String {
        let duration = st.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
        Self::format_nanos(duration.as_nanos(), pattern)
    }

    fn replace_tokens(pattern: &str, dt: &DateTimeUtc) -> String {
        let mut out = String::with_capacity(pattern.len() + 32);
        let mut remaining = pattern;

        while remaining.is_empty() == false {
            if let Some(rest) = remaining.strip_prefix("WYYY") { out.push_str(&format!("{:04}", dt.iso_year)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("YYYY") { out.push_str(&format!("{:04}", dt.year)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("hhhh") { out.push_str(&dt.hour_12_format()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("AAA") { out.push_str(dt.astro_season_abbr()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("SSS") { out.push_str(dt.meteo_season_abbr()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("MMM") { out.push_str(dt.month_abbr()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("YDD") { out.push_str(&format!("{:03}", dt.day_of_year)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("DDD") { out.push_str(dt.weekday_abbr()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("zzz") { out.push_str(&format!("{:03}", dt.millisecond)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("WY") { out.push_str(&format!("{:02}", dt.iso_year % 100)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("YY") { out.push_str(&format!("{:02}", dt.year % 100)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("MM") { out.push_str(&format!("{:02}", dt.month)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("MD") { out.push_str(&format!("{:02}", dt.day)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("WW") { out.push_str(&format!("{:02}", dt.iso_week)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("hh") { out.push_str(&format!("{:02}", dt.hour)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("mm") { out.push_str(&format!("{:02}", dt.minute)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("ss") { out.push_str(&format!("{:02}", dt.second)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("tt") { out.push_str(&format!("{:02}", dt.tierce)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("qq") { out.push_str(&format!("{:02}", dt.quadra)); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("A") { out.push_str(&dt.astro_season_num().to_string()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("S") { out.push_str(&dt.meteo_season_num().to_string()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("Q") { out.push_str(&dt.quarter().to_string()); remaining = rest; }
            else if let Some(rest) = remaining.strip_prefix("D") { out.push_str(&dt.weekday_num().to_string()); remaining = rest; }
            else {
                let mut chars = remaining.chars();
                if let Some(c) = chars.next() {
                    out.push(c);
                    remaining = chars.as_str();
                }
            }
        }

        out
    }
}

struct DateTimeUtc {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
    tierce: u32,
    quadra: u32,
    day_of_year: u32,
    weekday: u32,
    iso_week: u32,
    iso_year: i32,
}

impl DateTimeUtc {
    fn from_total_nanos(nanos_total: u128) -> Self {
        let secs = (nanos_total / 1_000_000_000) as u64;
        let subsec_nanos = (nanos_total % 1_000_000_000) as u64;

        let millisecond = (subsec_nanos / 1_000_000) as u32;
        let tierce = ((subsec_nanos * 60) / 1_000_000_000) as u32;
        let quadra = (((subsec_nanos * 3600) / 1_000_000_000) % 60) as u32;

        let days = (secs / 86400) as i64;
        let rem_secs = (secs % 86400) as u32;

        let hour = rem_secs / 3600;
        let minute = (rem_secs % 3600) / 60;
        let second = rem_secs % 60;

        let weekday = (((days + 3) % 7) + 1) as u32;

        let z = days + 719468;
        let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
        let doe = (z - era * 146097) as u32;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i32 + era as i32 * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = if m <= 2 { y + 1 } else { y };

        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let doy_calendar = if m > 2 {
            let month_days: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
            month_days.get((m - 1) as usize).copied().unwrap_or(0)
                + d
                + if is_leap { 1 } else { 0 }
        } else {
            let month_days: [u32; 2] = [0, 31];
            month_days.get((m - 1) as usize).copied().unwrap_or(0) + d
        };

        let wday = weekday;
        let doy_i = doy_calendar as i32;
        let iso_w = (doy_i - wday as i32 + 10) / 7;
        let (iso_week, iso_year) = if iso_w < 1 {
            (52, year - 1)
        } else if iso_w > 52 {
            (1, year + 1)
        } else {
            (iso_w as u32, year)
        };

        Self {
            year,
            month: m,
            day: d,
            hour,
            minute,
            second,
            millisecond,
            tierce,
            quadra,
            day_of_year: doy_calendar,
            weekday,
            iso_week,
            iso_year,
        }
    }

    fn weekday_num(&self) -> u32 {
        self.weekday
    }

    fn weekday_abbr(&self) -> &'static str {
        match self.weekday {
            1 => "MON",
            2 => "TUE",
            3 => "WED",
            4 => "THU",
            5 => "FRI",
            6 => "SAT",
            7 => "SUN",
            _ => "MON",
        }
    }

    fn month_abbr(&self) -> &'static str {
        match self.month {
            1 => "JAN",
            2 => "FEB",
            3 => "MAR",
            4 => "APR",
            5 => "MAY",
            6 => "JUN",
            7 => "JUL",
            8 => "AUG",
            9 => "SEP",
            10 => "OCT",
            11 => "NOV",
            12 => "DEC",
            _ => "JAN",
        }
    }

    fn hour_12_format(&self) -> String {
        let is_pm = self.hour >= 12;
        let h12 = match self.hour % 12 {
            0 => 12,
            h => h,
        };
        format!("{}{:02}", if is_pm { "pm" } else { "am" }, h12)
    }

    fn quarter(&self) -> u32 {
        ((self.month - 1) / 3) + 1
    }

    fn meteo_season_num(&self) -> u32 {
        match self.month {
            9..=11 => 1,
            12 | 1 | 2 => 2,
            3..=5 => 3,
            6..=8 => 4,
            _ => 1,
        }
    }

    fn meteo_season_abbr(&self) -> &'static str {
        match self.meteo_season_num() {
            1 => "AUT",
            2 => "WIN",
            3 => "SPR",
            4 => "SUM",
            _ => "AUT",
        }
    }

    fn astro_season_num(&self) -> u32 {
        let m = self.month;
        let d = self.day;

        match (m, d) {
            (3, 21..=31) | (4, _) | (5, _) | (6, 1..=21) => 3,
            (6, 22..=31) | (7, _) | (8, _) | (9, 1..=22) => 4,
            (9, 23..=31) | (10, _) | (11, _) | (12, 1..=21) => 1,
            _ => 2,
        }
    }

    fn astro_season_abbr(&self) -> &'static str {
        match self.astro_season_num() {
            1 => "AUT",
            2 => "WIN",
            3 => "SPR",
            4 => "SUM",
            _ => "AUT",
        }
    }
}