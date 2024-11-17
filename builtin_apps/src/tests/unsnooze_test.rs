#![allow(clippy::zero_prefixed_literal)]

use todo_app::Mutated;
use todo_printing::Action::*;
use todo_printing::BriefPrintableTask;
use todo_printing::PrintableWarning;
use todo_printing::Status::*;
use todo_testing::ymdhms;

use super::testing::task;
use super::testing::Fixture;

#[test]
fn unsnooze_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a --snooze 1 hour");
    fix.test("todo unsnooze 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Unsnooze))
        .end();
}

#[test]
fn unsnoozed_task_appears_at_end_of_incomplete_list() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a b c");
    fix.test("todo snooze a --until 1 hour");
    fix.test("todo unsnooze a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 3, Incomplete).action(Unsnooze))
        .end();
}

#[test]
fn unsnooze_task_that_is_not_snoozed_is_no_op() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a b c");
    fix.test("todo unsnooze b")
        .modified(Mutated::No)
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
        .modified(Mutated::No)
        .validate()
        .printed_warning(&PrintableWarning::CannotUnsnoozeBecauseComplete(
            BriefPrintableTask::new(0, Complete),
        ))
        .end();
}

#[test]
fn unsnooze_blocked_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2022, 02, 22, 10, 00, 00);
    fix.test("todo new a b --chain");
    fix.test("todo snooze b --until 1 hour");
    fix.test("todo unsnooze b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1).action(Unsnooze))
        .end();
}
