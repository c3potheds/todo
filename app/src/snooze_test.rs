#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::task,
    super::testing::Fixture,
    todo_printing::{
        Action::*, BriefPrintableTask, Plicit::Explicit, PrintableError,
        PrintableWarning, Status::*,
    },
    todo_testing::ymdhms,
};

#[test]
fn snooze_no_date() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo snooze a")
        .modified(false)
        .validate()
        .printed_error(&PrintableError::EmptyDate {
            flag: Some("--until".to_string()),
        })
        .end();
}

#[test]
fn snooze_one_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b");
    fix.test("todo snooze a --until 1 day")
        .modified(true)
        .validate()
        .printed_task(
            &task("a", 2, Blocked)
                .start_date(ymdhms(2021, 05, 28, 00, 00, 00))
                .action(Snooze),
        )
        .end();
}

#[test]
fn snooze_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b c d e");
    fix.test("todo snooze a c e --until saturday")
        .modified(true)
        .validate()
        .printed_task(
            &task("a", 3, Blocked)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00))
                .action(Snooze),
        )
        .printed_task(
            &task("c", 4, Blocked)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00))
                .action(Snooze),
        )
        .printed_task(
            &task("e", 5, Blocked)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00))
                .action(Snooze),
        )
        .end();
}

#[test]
fn snooze_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a --snooze 2 hours");
    fix.test("todo snooze a --until 3 hours")
        .modified(true)
        .validate()
        .printed_task(
            &task("a", 1, Blocked)
                .start_date(ymdhms(2021, 05, 27, 14, 00, 00))
                .action(Snooze),
        )
        .end();
}

#[test]
fn cannot_snooze_completed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo snooze a --until saturday")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::CannotSnoozeBecauseComplete {
            cannot_snooze: BriefPrintableTask::new(0, Complete),
        })
        .end();
}

#[test]
fn snooze_blocked_task_above_layer_1() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b c --chain");
    fix.test("todo snooze c --until 1 day")
        .modified(true)
        .validate()
        .printed_task(
            &task("c", 3, Blocked)
                .start_date(ymdhms(2021, 05, 28, 00, 00, 00))
                .action(Snooze)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn snooze_after_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 10, 02, 23, 00, 00);
    fix.test("todo new a --due 1 day");
    fix.test("todo snooze a --until 2 days")
        .modified(true)
        .validate()
        .printed_warning(&PrintableWarning::SnoozedAfterDueDate {
            snoozed_task: BriefPrintableTask::new(1, Blocked),
            due_date: ymdhms(2022, 10, 03, 23, 59, 59),
            snooze_date: ymdhms(2022, 10, 04, 00, 00, 00),
        })
        .printed_task(
            &task("a", 1, Blocked)
                .start_date(ymdhms(2022, 10, 04, 00, 00, 00))
                .due_date(Explicit(ymdhms(2022, 10, 03, 23, 59, 59)))
                .action(Snooze),
        )
        .end();
}

#[test]
fn snooze_until_time_that_has_already_passed_should_leave_tasks_unmodified() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2023, 09, 30, 14, 00, 00);
    fix.test("todo new a");
    fix.test("todo snooze a --until last friday")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::SnoozedUntilPast {
            snoozed_task: BriefPrintableTask::new(1, Incomplete),
            snooze_date: ymdhms(2023, 09, 29, 00, 00, 00),
        })
        .printed_task(&task("a", 1, Incomplete))
        .end();
}

#[test]
fn snoozed_tasks_should_appear_before_tasks_they_block() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2023, 09, 30, 14, 00, 00);
    let tomorrow = ymdhms(2023, 10, 01, 00, 00, 00);
    fix.test("todo new a b c --chain");
    fix.test("todo snooze a --until tomorrow")
        .modified(true)
        .validate()
        .printed_task(
            &task("a", 1, Blocked).start_date(tomorrow).action(Snooze),
        )
        .end();
    fix.test("todo -a")
        .validate()
        .printed_task(&task("a", 1, Blocked).start_date(tomorrow))
        .printed_task(&task("b", 2, Blocked).deps_stats(0, 1))
        .printed_task(&task("c", 3, Blocked).deps_stats(0, 2))
        .end();
}
