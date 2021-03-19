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

// The humantime::format_duration() function will format durations like "5m 32s"
// to however much precision is representable. For "laconic" representation of
// duration, presented to the user, we don't need second-level precision for
// durations in the order of minutes, or minute-level precision for durations
// in the order of hourse, etc, so we strip off all but the first "word" in the
// formatted time.
fn format_duration_laconic(duration: chrono::Duration) -> String {
    let formatted =
        humantime::format_duration(duration.to_std().unwrap().into());
    match format!("{}", formatted).split(" ").next() {
        Some(chunk) => chunk.to_string(),
        None => panic!("Formatted duration is empty string: {}", formatted),
    }
}

pub fn display_relative_time<Tz: TimeZone>(
    now: DateTime<Tz>,
    then: DateTime<Tz>,
) -> String {
    let duration = then - now;
    if duration < chrono::Duration::zero() {
        format!("{} ago", format_duration_laconic(-duration))
    } else {
        format!("in {}", format_duration_laconic(duration))
    }
}
