use chrono::{DateTime, Local, TimeZone, Utc};

pub fn ymdhms(
    yr: i32,
    mon: u32,
    day: u32,
    hr: u32,
    min: u32,
    sec: u32,
) -> DateTime<Utc> {
    Local
        .ymd(yr, mon, day)
        .and_hms(hr, min, sec)
        .with_timezone(&Utc)
}
