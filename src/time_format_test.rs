use chrono::Local;
use chrono::TimeZone;
use time_format::*;

#[test]
fn in_five_minutes_abbreviated() {
    let now = Local.ymd(2021, 03, 18).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(12, 05, 00);
    let actual = parse_time(Local, now, "5 min").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_five_minutes_verbose() {
    let now = Local.ymd(2021, 03, 18).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(12, 05, 00);
    let actual = parse_time(Local, now, "5 minutes").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_one_hour() {
    let now = Local.ymd(2021, 03, 18).and_hms(17, 12, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(18, 12, 00);
    let actual = parse_time(Local, now, "1 hour").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_two_days() {
    let now = Local.ymd(2021, 03, 18).and_hms(17, 12, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(18, 12, 00);
    let actual = parse_time(Local, now, "1 hour").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_six_days() {
    let now = Local.ymd(2021, 03, 18).and_hms(20, 05, 00);
    let expected = Local.ymd(2021, 03, 24).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "6 days").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_two_weeks() {
    let now = Local.ymd(2021, 03, 19).and_hms(23, 23, 00);
    let expected = Local.ymd(2021, 04, 02).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "2 weeks").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn across_leap_year() {
    let now = Local.ymd(2020, 02, 27).and_hms(12, 00, 00);
    let expected = Local.ymd(2020, 03, 01).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "3 days").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn two_months() {
    let now = Local.ymd(2021, 03, 19).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 05, 19).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "2 months").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn next_monday() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 22).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "monday").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn wednesday_abbreviated() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 24).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "wed").unwrap();
    assert_eq!(actual, expected);
}

#[test]
#[ignore = "todo-dev.time-format.day-of-week.previous"]
fn last_monday() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 15).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "last monday").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn today() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "today").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn tomorrow() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 20).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "tomorrow").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn five_o_clock_pm_verbose() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(17, 00, 00);
    let actual = parse_time(Local, now, "5:00 pm").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn five_o_clock_pm_abbreviated() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(17, 00, 00);
    let actual = parse_time(Local, now, "5pm").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn am_next_day() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 20).and_hms(05, 00, 00);
    let actual = parse_time(Local, now, "5am").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_five_minutes() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 05, 00);
    let expected = "in 5m";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_minute_level_precision() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 05, 30);
    let expected = "in 5m";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_minute_level_precision_limit() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 06, 59);
    let expected = "in 6m";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_five_minutes_ago() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(09, 55, 00);
    let expected = "5m ago";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_five_days() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 24).and_hms(23, 59, 59);
    let expected = "in 5days";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_eight_days() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 27).and_hms(23, 59, 59);
    let expected = "in 8days";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_a_month() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 04, 27).and_hms(23, 59, 59);
    let expected = "in 1month";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_11_months() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2022, 03, 18).and_hms(23, 59, 59);
    let expected = "in 11months";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}
