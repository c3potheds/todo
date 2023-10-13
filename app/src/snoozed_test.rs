#![allow(clippy::zero_prefixed_literal)]
use {
    super::testing::task, super::testing::Fixture, todo_printing::Status::*,
    todo_testing::ymdhms,
};

#[test]
fn no_tasks_snoozed() {
    let mut fix = Fixture::default();
    fix.test("todo snoozed").modified(false).validate().end();
}

#[test]
fn one_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b c");
    fix.test("todo snooze b --until 1 hour");
    fix.test("todo snoozed")
        .modified(false)
        .validate()
        .printed_task(
            &task("b", 3, Blocked).start_date(ymdhms(2021, 05, 27, 12, 00, 00)),
        )
        .end();
}

#[test]
fn multiple_snoozed_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 10, 15, 12, 00, 00);
    fix.test("todo new a b c d e");
    fix.test("todo snooze a b c --until 1 hour");
    fix.test("todo snoozed")
        .modified(false)
        .validate()
        .printed_task(
            &task("a", 3, Blocked).start_date(ymdhms(2022, 10, 15, 13, 00, 00)),
        )
        .printed_task(
            &task("b", 4, Blocked).start_date(ymdhms(2022, 10, 15, 13, 00, 00)),
        )
        .printed_task(
            &task("c", 5, Blocked).start_date(ymdhms(2022, 10, 15, 13, 00, 00)),
        )
        .end();
}
