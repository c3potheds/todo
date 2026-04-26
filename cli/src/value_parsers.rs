use std::error::Error;

use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Timelike;
use chrono::Utc;
use humantime::parse_duration;
use todo_time_format::parse_time;

pub fn parse_due_date(
    s: &str,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    let now = crate::time_utils::now(); // Use configurable system time
    Ok(parse_time(
        Local,
        now.with_timezone(&Local),
        s,
        todo_time_format::Snap::ToEnd,
    )
    .map(|due_date| due_date.with_timezone(&Utc).with_nanosecond(0).unwrap())?)
}

pub fn parse_snooze_date(s: &str) -> Result<DateTime<Utc>, String> {
    let now = crate::time_utils::now(); // Use configurable system time
    parse_time(
        Local,
        now.with_timezone(&Local),
        s,
        todo_time_format::Snap::ToStart,
    )
    .map(|dt| dt.with_timezone(&Utc))
    .map_err(|e| e.to_string())
}

pub fn parse_budget(s: &str) -> Result<Duration, Box<dyn Error + Send + Sync>> {
    if s == "0" || s.is_empty() {
        return Ok(Duration::default());
    }
    Ok(parse_duration(s).map(|d| Duration::seconds(d.as_secs() as i64))?)
}
