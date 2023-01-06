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
        .with_ymd_and_hms(yr, mon, day, hr, min, sec)
        .unwrap()
        .with_timezone(&Utc)
}
