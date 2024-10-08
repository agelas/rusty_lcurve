use chrono::{DateTime, Utc};

pub fn format_date(date: DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}
