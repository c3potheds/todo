extern crate humantime;

use chrono::DateTime;
use chrono::TimeZone;

pub fn parse_time<Tz: TimeZone>(
    now: DateTime<Tz>,
    s: &str,
) -> Result<DateTime<Tz>, humantime::DurationError> {
    humantime::parse_duration(s).map(|duration: std::time::Duration| {
        now + chrono::Duration::milliseconds(duration.as_millis() as i64)
    })
}
