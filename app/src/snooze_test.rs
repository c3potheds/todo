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
fn snooze_one_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b");
    fix.test("todo snooze a --until 1 day")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 2, Blocked)
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
        .validate()
        .printed_task(
            &PrintableTask::new("a", 3, Blocked)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00))
                .action(Snooze),
        )
        .printed_task(
            &PrintableTask::new("c", 4, Blocked)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00))
                .action(Snooze),
        )
        .printed_task(
            &PrintableTask::new("e", 5, Blocked)
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
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Blocked)
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
        .validate()
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .start_date(ymdhms(2021, 05, 28, 00, 00, 00))
                .action(Snooze),
        )
        .end();
}
