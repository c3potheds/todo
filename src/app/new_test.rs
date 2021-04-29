use app::testing::Fixture;
use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use printing::Action::*;
use printing::BriefPrintableTask;
use printing::PrintableError;
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
fn new_one_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_multiple_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(New))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).action(New))
        .end();
}

#[test]
fn new_block_on_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -p 0")
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_blocking_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -b 0")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_by_name() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo new d -p c -b a")
        .validate()
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("a", 4, Blocked))
        .end();
}

#[test]
fn new_chain_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(New))
        .end();
}

#[test]
fn new_one_blocking_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b --blocking 1")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_blocked_by_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b --blocked-by 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .end();
}

#[test]
fn new_one_blocking_one_short() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b -b 1")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_blocked_by_one_short() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b -p 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .end();
}

#[test]
fn new_blocking_multiple() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo new d -b 1 2 3")
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .end();
}

#[test]
fn new_blocking_and_blocked_by() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c -p 1 -b 2")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_in_between_blocking_pair() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p 1 -b 2")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_one_before_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --before b")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_three_before_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --before b")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("e", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("f", 4, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 5, Blocked))
        .end();
}

#[test]
fn new_one_before_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b c d -p a");
    fix.test("todo new e --before b c d")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("e", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .printed_task(&PrintableTask::new("d", 5, Blocked))
        .end();
}

#[test]
fn new_one_after_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --after b")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .end();
}

#[test]
fn new_three_after_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --after b")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("e", 4, Blocked).action(New))
        .printed_task(&PrintableTask::new("f", 5, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 6, Blocked))
        .end();
}

#[test]
fn new_one_after_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo new e --after a b c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .printed_task(&PrintableTask::new("e", 4, Blocked).action(New))
        .printed_task(&PrintableTask::new("d", 5, Blocked))
        .end();
}

#[test]
fn print_warning_on_cycle() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p b -b a")
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(3, Blocked),
        })
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(New))
        .end();
}

#[test]
fn new_with_priority() {
    let mut fix = Fixture::new();
    fix.test("todo new a --priority 1")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(New)
                .priority(1),
        )
        .end();
}

#[test]
fn new_task_with_priority_inserted_before_unprioritized_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c --priority 1")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete)
                .action(New)
                .priority(1),
        )
        .end();
}

#[test]
fn new_task_with_negative_priority_inserted_after_unprioritized_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c --priority -1")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 3, Incomplete)
                .action(New)
                .priority(-1),
        )
        .end();
}

#[test]
fn new_task_with_priority_inserted_in_sorted_order() {
    let mut fix = Fixture::new();
    fix.test("todo new a --priority 1");
    fix.test("todo new b --priority 3");
    fix.test("todo new c --priority 2")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 2, Incomplete)
                .action(New)
                .priority(2),
        )
        .end();
}

#[test]
fn new_with_due_date() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    let in_5_hours = ymdhms(2021, 04, 12, 20, 00, 00);
    fix.test("todo new a --due 5 hours")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .due_date(in_5_hours)
                .action(New),
        )
        .end();
}

#[test]
fn new_with_invalid_due_date() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    fix.test("todo new a --due blah blah")
        .validate()
        .printed_error(&PrintableError::CannotParseDueDate {
            cannot_parse: "blah blah".to_string(),
        })
        .end();
}

#[test]
fn new_with_due_date_shows_affected_deps() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    let in_2_days = ymdhms(2021, 04, 14, 23, 59, 59);
    fix.test("todo new a b c --chain");
    fix.test("todo new d -p c --due 2 days")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).due_date(in_2_days),
        )
        .printed_task(&PrintableTask::new("b", 2, Blocked).due_date(in_2_days))
        .printed_task(&PrintableTask::new("c", 3, Blocked).due_date(in_2_days))
        .printed_task(
            &PrintableTask::new("d", 4, Blocked)
                .due_date(in_2_days)
                .action(New),
        )
        .end();
}

#[test]
fn new_with_budget_shows_affected_deps() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 04, 29, 09, 30, 00);
    let before_7 = ymdhms(2021, 04, 29, 18, 59, 59);
    let end_of_day = ymdhms(2021, 04, 29, 23, 59, 59);
    fix.test("todo new a");
    fix.test("todo new b -p a --due today --budget 5 hours")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).due_date(before_7),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .due_date(end_of_day)
                .action(New),
        )
        .end();
}

#[test]
fn new_with_too_long_time_budget() {
    let mut fix = Fixture::new();
    fix.test("todo new a --budget 137 years")
        .validate()
        .printed_error(&PrintableError::DurationIsTooLong {
            duration: 4323391200,
            string_repr: "137 years".to_string(),
        })
        .end();
}

#[test]
fn new_with_unintelligible_time_budget() {
    let mut fix = Fixture::new();
    fix.test("todo new a --budget blah")
        .validate()
        .printed_error(&PrintableError::CannotParseDuration {
            cannot_parse: "blah".to_string(),
        })
        .end();
}
