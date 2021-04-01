use app::testing::*;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use model::TaskStatus;
use printing::Action;
use printing::Expect;
use printing::LogDate;

#[test]
fn log_with_no_tasks_completed() {
    let mut fix = Fixture::new();
    fix.test("todo log").validate().end();
}

#[test]
fn log_after_single_task_completed() {
    let mut fix = Fixture::new();
    fix.clock.now = Utc.ymd(2021, 03, 26).and_hms(17, 37, 00);
    fix.test("todo new a b c");
    fix.test("todo check 2");
    fix.test("todo log")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
            Expect::LogDate(LogDate::YearMonthDay(2021, 03, 26)),
        ])
        .end();
}

#[test]
fn log_after_multiple_tasks_completed() {
    let mut fix = Fixture::new();
    fix.clock.now = Utc.ymd(2021, 03, 26).and_hms(17, 42, 00);
    fix.test("todo new a b c");
    fix.test("todo check 1 3");
    fix.test("todo log")
        .validate()
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
            Expect::LogDate(LogDate::YearMonthDay(2021, 03, 26)),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
            // Don't repeat the completion date if it's the same.
            Expect::LogDate(LogDate::Invisible),
        ])
        .end();
}

#[test]
fn log_shows_date_when_it_changes() {
    let mut fix = Fixture::new();
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
        .printed_task(&[
            Expect::Desc("d"),
            Expect::LogDate(LogDate::YearMonthDay(2021, 01, 02)),
        ])
        .printed_task(&[Expect::Desc("c"), Expect::LogDate(LogDate::Invisible)])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::LogDate(LogDate::YearMonthDay(2021, 01, 01)),
        ])
        .printed_task(&[Expect::Desc("a"), Expect::LogDate(LogDate::Invisible)])
        .end();
}
