use chrono::Local;
use chrono::TimeZone;
use time_format::*;

#[test]
fn in_five_minutes_abbreviated() {
    let now = Local.ymd(2021, 03, 18).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(12, 05, 00);
    let actual = parse_time(now, "5 min").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_five_minutes_verbose() {
    let now = Local.ymd(2021, 03, 18).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(12, 05, 00);
    let actual = parse_time(now, "5 minutes").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_one_hour() {
    let now = Local.ymd(2021, 03, 18).and_hms(17, 12, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(18, 12, 00);
    let actual = parse_time(now, "1 hour").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_two_days() {
    let now = Local.ymd(2021, 03, 18).and_hms(17, 12, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(18, 12, 00);
    let actual = parse_time(now, "1 hour").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_six_days() {
    let now = Local.ymd(2021, 03, 18).and_hms(20, 05, 00);
    let expected = Local.ymd(2021, 03, 24).and_hms(20, 05, 00);
    let actual = parse_time(now, "6 days").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_two_weeks() {
    let now = Local.ymd(2021, 03, 19).and_hms(23, 23, 00);
    let expected = Local.ymd(2021, 04, 02).and_hms(23, 23, 00);
    let actual = parse_time(now, "2 weeks").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn across_leap_year() {
    let now = Local.ymd(2020, 02, 27).and_hms(12, 00, 00);
    let expected = Local.ymd(2020, 03, 01).and_hms(12, 00, 00);
    let actual = parse_time(now, "3 days").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn two_months() {
    let now = Local.ymd(2021, 03, 19).and_hms(12, 00, 00);
    let expected = Local
        .ymd(2021, 05, 19)
        // TODO: humantime represents a month as 30.44 days. Either snap to the
        // same time of day as "now" or to the end of the day.
        .and_hms(09, 07, 12);
    let actual = parse_time(now, "2 months").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.day-of-week"]
fn next_monday() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 22).and_hms(23, 59, 59);
    let actual = parse_time(now, "monday").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.day-of-week"]
fn wednesday_abbreviated() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 24).and_hms(23, 59, 59);
    let actual = parse_time(now, "wed").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.day-of-week.previous"]
fn last_monday() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 15).and_hms(23, 59, 59);
    let actual = parse_time(now, "last monday").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.today"]
fn today() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(23, 59, 59);
    let actual = parse_time(now, "today").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.tomorrow"]
fn tomorrow() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 20).and_hms(23, 59, 59);
    let actual = parse_time(now, "tomorrow").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.time-of-day"]
fn five_o_clock_pm_verbose() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(17, 00, 00);
    let actual = parse_time(now, "5:00 pm").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.time-of-day"]
fn five_o_clock_pm_abbreviated() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(17, 00, 00);
    let actual = parse_time(now, "5pm").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.time-of-day"]
fn am_next_day() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 20).and_hms(05, 00, 00);
    let actual = parse_time(now, "5am").unwrap();
    assert_eq!(actual, expected);
}
