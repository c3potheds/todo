#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::Fixture,
    printing::{Action::*, PrintableTask, Status::*},
    testing::ymdhms,
};

#[test]
fn status_while_empty() {
    let mut fix = Fixture::default();
    fix.test("todo").modified(false).validate().end();
}

#[test]
fn status_after_added_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .end();
}

#[test]
fn status_does_not_include_blocked_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo block 2 --on 1");
    fix.test("todo")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .end();
}

#[test]
fn include_blocked_in_status() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 1 --on 2");
    fix.test("todo -b")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&PrintableTask::new("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn include_complete_in_status() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo check 1");
    fix.test("todo -d")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .end();
}

#[test]
fn include_all_in_status() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check 1");
    fix.test("todo -a")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&PrintableTask::new("c", 2, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn status_after_check_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check 2 3");
    fix.test("todo")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .end();
}

#[test]
fn status_after_unblocking_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo unblock 2 --from 1");
    fix.test("todo")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .end();
}

#[test]
fn status_unsnoozes_if_snooze_time_passed() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 18, 00, 00);
    fix.test("todo new a");
    fix.test("todo snooze a --until 1 day");
    fix.clock.now = ymdhms(2021, 05, 29, 18, 00, 00);
    fix.test("todo")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00))
                .action(Unsnooze),
        )
        .end();
}

#[test]
fn status_does_not_unsnooze_if_snooze_time_does_not_pass() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 18, 00, 00);
    fix.test("todo new a");
    fix.test("todo snooze a --until 1 day");
    fix.test("todo").modified(false).validate().end();
}

#[test]
fn status_unsnooze_preserves_order() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 30, 12, 00, 00);
    fix.test("todo new a b c");
    fix.test("todo snooze a --until 1 hour");
    fix.test("todo snooze b --until 2 hours");
    fix.test("todo snooze c --until 3 hours");
    fix.clock.now = ymdhms(2021, 05, 30, 16, 00, 00);
    fix.test("todo")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .start_date(ymdhms(2021, 05, 30, 13, 00, 00))
                .action(Unsnooze),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Incomplete)
                .start_date(ymdhms(2021, 05, 30, 14, 00, 00))
                .action(Unsnooze),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Incomplete)
                .start_date(ymdhms(2021, 05, 30, 15, 00, 00))
                .action(Unsnooze),
        )
        .end();
}
