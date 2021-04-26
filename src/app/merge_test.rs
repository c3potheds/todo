use app::testing::Fixture;
use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use printing::Action::*;
use printing::PrintableTask;
use printing::Status::*;

fn ymdhms(
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

#[test]
fn merge_two_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo merge a b --into ab")
        .validate()
        .printed_task(&PrintableTask::new("ab", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_three_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into abc")
        .validate()
        .printed_task(&PrintableTask::new("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_preserves_deps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo merge b c --into bc")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("bc", 2, Blocked).action(Select))
        .end();
}

#[test]
fn merge_preserves_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo merge a b --into ab")
        .validate()
        .printed_task(&PrintableTask::new("ab", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("c", 2, Blocked))
        .end();
}

#[test]
fn merge_preserves_deps_and_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e --chain");
    fix.test("todo merge b c d --into bcd")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("bcd", 2, Blocked).action(Select))
        .printed_task(&PrintableTask::new("e", 3, Blocked))
        .end();
}

#[test]
fn merged_task_has_min_due_date_of_sources() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 04, 25, 23, 20, 00);
    let in_10_min = ymdhms(2021, 04, 25, 23, 30, 00);
    fix.test("todo new a --due 15 min");
    fix.test("todo new b --due 10 min");
    fix.test("todo new c --due 20 min");
    fix.test("todo merge a b c --into abc")
        .validate()
        .printed_task(
            &PrintableTask::new("abc", 1, Incomplete)
                .action(Select)
                .due_date(in_10_min),
        )
        .end();
}

#[test]
fn merged_task_has_max_priority_of_sources() {
    let mut fix = Fixture::new();
    fix.test("todo new a --priority 1");
    fix.test("todo new b --priority -1");
    fix.test("todo new c --priority 2");
    fix.test("todo new d");
    fix.test("todo merge a b c d --into abcd")
        .validate()
        .printed_task(
            &PrintableTask::new("abcd", 1, Incomplete)
                .action(Select)
                .priority(2),
        )
        .end();
}
