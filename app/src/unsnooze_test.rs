#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::Fixture,
    printing::{
        Action::*, BriefPrintableTask, PrintableTask, PrintableWarning,
        Status::*,
    },
    testing::ymdhms,
};

#[test]
fn unsnooze_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a --snooze 1 hour");
    fix.test("todo unsnooze 1")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Unsnooze))
        .end();
}

#[test]
fn unsnoozed_task_appears_at_end_of_incomplete_list() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a b c");
    fix.test("todo snooze a --until 1 hour");
    fix.test("todo unsnooze a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Incomplete).action(Unsnooze))
        .end();
}

#[test]
fn unsnooze_task_that_is_not_snoozed_is_no_op() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a b c");
    fix.test("todo unsnooze b")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::CannotUnsnoozeBecauseNotSnoozed(
            BriefPrintableTask::new(2, Incomplete),
        ))
        .end();
}

#[test]
fn show_warning_when_unsnoozing_complete_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a --snooze 1 hour");
    fix.test("todo check a");
    fix.test("todo unsnooze a")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::CannotUnsnoozeBecauseComplete(
            BriefPrintableTask::new(0, Complete),
        ))
        .end();
}

#[test]
fn show_warning_when_unsnoozing_blocked_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a b --chain");
    fix.test("todo snooze b --until 1 hour");
    fix.test("todo unsnooze b")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::CannotUnsnoozeBecauseBlocked {
            cannot_unsnooze: BriefPrintableTask::new(2, Blocked),
            blocked_by: vec![BriefPrintableTask::new(1, Incomplete)],
        })
        .end();
}
