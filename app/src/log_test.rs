#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::Fixture,
    chrono::{Local, TimeZone, Utc},
    printing::{LogDate::*, PrintableTask, Status::*},
};

#[test]
fn log_with_no_tasks_completed() {
    let mut fix = Fixture::default();
    fix.test("todo log").validate().end();
}

#[test]
fn log_after_single_task_completed() {
    let mut fix = Fixture::default();
    fix.clock.now = Utc.ymd(2021, 03, 26).and_hms(17, 37, 00);
    fix.test("todo new a b c");
    fix.test("todo check 2");
    fix.test("todo log")
        .validate()
        .printed_task(
            &PrintableTask::new("b", 0, Complete)
                .log_date(YearMonthDay(2021, 03, 26)),
        )
        .end();
}

#[test]
fn log_after_multiple_tasks_completed() {
    let mut fix = Fixture::default();
    fix.clock.now = Utc.ymd(2021, 03, 26).and_hms(17, 42, 00);
    fix.test("todo new a b c");
    fix.test("todo check 1 3");
    fix.test("todo log")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 0, Complete)
                .log_date(YearMonthDay(2021, 03, 26)),
        )
        .printed_task(
            &PrintableTask::new("a", -1, Complete)
                // Don't repeat the log date if it's the same.
                .log_date(Invisible),
        )
        .end();
}

#[test]
fn log_shows_date_when_it_changes() {
    let mut fix = Fixture::default();
    fix.clock.now = Local
        .ymd(2021, 01, 01)
        .and_hms(00, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a b c d");
    fix.test("todo check a b");
    fix.clock.now = Local
        .ymd(2021, 01, 02)
        .and_hms(00, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo check c d");
    fix.test("todo log")
        .validate()
        .printed_task(
            &PrintableTask::new("d", 0, Complete)
                .log_date(YearMonthDay(2021, 01, 02)),
        )
        .printed_task(
            &PrintableTask::new("c", -1, Complete).log_date(Invisible),
        )
        .printed_task(
            &PrintableTask::new("b", -2, Complete)
                .log_date(YearMonthDay(2021, 01, 01)),
        )
        .printed_task(
            &PrintableTask::new("a", -3, Complete).log_date(Invisible),
        )
        .end();
}
