#![allow(clippy::zero_prefixed_literal)]

use {
    crate::*,
    chrono::{Local, TimeZone},
};

#[test]
fn in_five_minutes_abbreviated() {
    let now = Local.ymd(2021, 03, 18).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(12, 05, 00);
    let actual = parse_time(Local, now, "5 min", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_five_minutes_verbose() {
    let now = Local.ymd(2021, 03, 18).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(12, 05, 00);
    let actual = parse_time(Local, now, "5 minutes", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_one_hour() {
    let now = Local.ymd(2021, 03, 18).and_hms(17, 12, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(18, 12, 00);
    let actual = parse_time(Local, now, "1 hour", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_two_days() {
    let now = Local.ymd(2021, 03, 18).and_hms(17, 12, 00);
    let expected = Local.ymd(2021, 03, 18).and_hms(18, 12, 00);
    let actual = parse_time(Local, now, "1 hour", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_six_days() {
    let now = Local.ymd(2021, 03, 18).and_hms(20, 05, 00);
    let expected = Local.ymd(2021, 03, 24).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "6 days", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn in_two_weeks() {
    let now = Local.ymd(2021, 03, 19).and_hms(23, 23, 00);
    let expected = Local.ymd(2021, 04, 02).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "2 weeks", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn across_leap_year() {
    let now = Local.ymd(2020, 02, 27).and_hms(12, 00, 00);
    let expected = Local.ymd(2020, 03, 01).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "3 days", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn two_months() {
    let now = Local.ymd(2021, 03, 19).and_hms(12, 00, 00);
    let expected = Local.ymd(2021, 05, 19).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "2 months", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn next_monday() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 22).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "monday", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn wednesday_abbreviated() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 24).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "wed", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn last_monday() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 15).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "last monday", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn last_tuesday_when_today_is_tuesday() {
    let now = Local.ymd(2021, 03, 30).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 23).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "last tue", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn today() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "today", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn today_snap_to_start() {
    let now = Local.ymd(2022, 04, 09).and_hms(10, 00, 00);
    let expected = Local.ymd(2022, 04, 09).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "today", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn tomorrow() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 20).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "tomorrow", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn tomorrow_snap_to_start() {
    let now = Local.ymd(2022, 04, 09).and_hms(09, 30, 00);
    let expected = Local.ymd(2022, 04, 10).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "tomorrow", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn five_o_clock_pm_verbose() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(17, 00, 00);
    let actual = parse_time(Local, now, "5:00 pm", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn five_o_clock_pm_abbreviated() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 19).and_hms(17, 00, 00);
    let actual = parse_time(Local, now, "5pm", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn am_next_day() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 03, 20).and_hms(05, 00, 00);
    let actual = parse_time(Local, now, "5am", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn today_12pm() {
    let now = Local.ymd(2022, 04, 08).and_hms(10, 00, 00);
    let expected = Local.ymd(2022, 04, 08).and_hms(12, 00, 00);
    let actual = parse_time(Local, now, "12pm", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn today_1pm() {
    let now = Local.ymd(2022, 04, 08).and_hms(10, 00, 00);
    let expected = Local.ymd(2022, 04, 08).and_hms(13, 00, 00);
    let actual = parse_time(Local, now, "1pm", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn today_10am() {
    let now = Local.ymd(2022, 04, 08).and_hms(5, 00, 00);
    let expected = Local.ymd(2022, 04, 08).and_hms(10, 00, 00);
    let actual = parse_time(Local, now, "10am", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn month_without_day_next_month() {
    let now = Local.ymd(2021, 03, 30).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 04, 30).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "april", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn month_without_day_in_four_months() {
    let now = Local.ymd(2021, 03, 30).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 07, 31).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "july", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn month_without_day_february() {
    let now = Local.ymd(2020, 12, 25).and_hms(00, 00, 00);
    let expected = Local.ymd(2021, 02, 28).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "feb", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn month_without_day_february_leap_year() {
    let now = Local.ymd(2019, 12, 25).and_hms(00, 00, 00);
    let expected = Local.ymd(2020, 02, 29).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "feb", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn month_with_day() {
    let now = Local.ymd(2021, 04, 01).and_hms(11, 00, 00);
    let expected = Local.ymd(2021, 07, 04).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "july 4", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn month_without_day_this_month() {
    let now = Local.ymd(2021, 04, 01).and_hms(09, 00, 00);
    let expected = Local.ymd(2021, 04, 30).and_hms(23, 59, 59);
    let actual = parse_time(Local, now, "april", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_in_1_day() {
    let now = Local.ymd(2021, 05, 21).and_hms(11, 00, 00);
    let expected = Local.ymd(2021, 05, 22).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "1 day", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_in_2_days() {
    let now = Local.ymd(2021, 05, 21).and_hms(11, 00, 00);
    let expected = Local.ymd(2021, 05, 23).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "2 days", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_in_1_week() {
    let now = Local.ymd(2021, 05, 21).and_hms(11, 00, 00);
    let expected = Local.ymd(2021, 05, 28).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "1 week", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_in_1_month() {
    let now = Local.ymd(2021, 05, 21).and_hms(11, 00, 00);
    let expected = Local.ymd(2021, 06, 20).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "1 month", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_in_named_month() {
    let now = Local.ymd(2021, 05, 21).and_hms(11, 00, 00);
    let expected = Local.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "june", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_named_month_next_year() {
    let now = Local.ymd(2021, 05, 21).and_hms(11, 00, 00);
    let expected = Local.ymd(2022, 02, 01).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "february", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_on_month_day() {
    let now = Local.ymd(2021, 05, 21).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 06, 10).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "june 10", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_named_day() {
    let now = Local.ymd(2021, 05, 21).and_hms(11, 00, 00);
    let expected = Local.ymd(2021, 05, 24).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "monday", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn start_previous_day_of_week() {
    let now = Local.ymd(2021, 05, 24).and_hms(10, 00, 00);
    let expected = Local.ymd(2021, 05, 21).and_hms(00, 00, 00);
    let actual = parse_time(Local, now, "last friday", Snap::ToStart).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn time_of_day_earlier_than_now_at_end_of_month() {
    let now = Local.ymd(2022, 04, 30).and_hms(10, 00, 00);
    let expected = Local.ymd(2022, 05, 01).and_hms(09, 00, 00);
    let actual = parse_time(Local, now, "9 am", Snap::ToEnd).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn complete_gobbledeygook() {
    let now = Local.ymd(2022, 04, 09).and_hms(11, 00, 00);
    assert_eq!(
        parse_time(Local, now, "blah", Snap::ToEnd),
        Err(ParseTimeError)
    );
}

#[test]
fn invalid_hour() {
    let now = Local.ymd(2022, 04, 09).and_hms(11, 00, 00);
    assert_eq!(
        parse_time(Local, now, "13am", Snap::ToStart),
        Err(ParseTimeError)
    );
}

#[test]
fn too_large_hour_am() {
    let now = Local.ymd(2022, 04, 09).and_hms(11, 00, 00);
    assert_eq!(
        parse_time(Local, now, "256am", Snap::ToStart),
        Err(ParseTimeError)
    );
}

#[test]
fn too_large_hour_pm() {
    let now = Local.ymd(2022, 04, 09).and_hms(11, 00, 00);
    assert_eq!(
        parse_time(Local, now, "256pm", Snap::ToStart),
        Err(ParseTimeError)
    );
}

#[test]
fn too_large_hour_followed_by_minutes() {
    let now = Local.ymd(2022, 04, 09).and_hms(11, 00, 00);
    assert_eq!(
        parse_time(Local, now, "256:30am", Snap::ToStart),
        Err(ParseTimeError)
    );
}

#[test]
fn too_large_minute_am() {
    let now = Local.ymd(2022, 04, 09).and_hms(11, 00, 00);
    assert_eq!(
        parse_time(Local, now, "1:256am", Snap::ToStart),
        Err(ParseTimeError)
    );
}

#[test]
fn too_large_minute_pm() {
    let now = Local.ymd(2022, 04, 09).and_hms(11, 00, 00);
    assert_eq!(
        parse_time(Local, now, "1:256pm", Snap::ToStart),
        Err(ParseTimeError)
    );
}

#[test]
fn relative_time_in_one_second() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 00, 01);
    let expected = "in 1 second";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_one_minute() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 01, 00);
    let expected = "in 1 minute";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_one_hour() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(11, 00, 00);
    let expected = "in 1 hour";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_five_minutes() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 05, 00);
    let expected = "in 5 minutes";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_minute_level_precision() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 05, 30);
    let expected = "in 5 minutes";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_minute_level_precision_limit() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(10, 06, 59);
    let expected = "in 6 minutes";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_five_minutes_ago() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 19).and_hms(09, 55, 00);
    let expected = "5 minutes ago";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_five_days() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 24).and_hms(23, 59, 59);
    let expected = "in 5 days";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_eight_days() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 03, 27).and_hms(23, 59, 59);
    let expected = "in 8 days";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_a_month() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2021, 04, 27).and_hms(23, 59, 59);
    let expected = "in 1 month";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}

#[test]
fn relative_time_in_11_months() {
    let now = Local.ymd(2021, 03, 19).and_hms(10, 00, 00);
    let then = Local.ymd(2022, 03, 18).and_hms(23, 59, 59);
    let expected = "in 11 months";
    let actual = display_relative_time(now, then);
    assert_eq!(actual, expected);
}
