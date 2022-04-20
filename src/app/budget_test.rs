#![allow(clippy::zero_prefixed_literal)]

use crate::{
    app::testing::Fixture,
    printing::{Action::*, PrintableError, PrintableTask, Status::*},
    testing::ymdhms,
};
use chrono::Duration;

#[test]
fn budget_one_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 15, 00, 00);
    let in_2_days = ymdhms(2021, 05, 01, 23, 59, 59);
    fix.test("todo new a --due 2 days");
    fix.test("todo budget a --is 1 day")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .due_date(in_2_days)
                .action(Select),
        )
        .end();
}

#[test]
fn budget_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo budget a c e --is 2 days")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("e", 5, Incomplete).action(Select))
        .end();
}

#[test]
fn budget_chain_alters_due_dates() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 12, 00, 00);
    fix.test("todo new a b c d e --chain --due today");
    fix.test("todo budget a b c d e --is 1 hour")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .due_date(ymdhms(2021, 04, 29, 19, 59, 59))
                .action(Select),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .due_date(ymdhms(2021, 04, 29, 20, 59, 59))
                .action(Select),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .due_date(ymdhms(2021, 04, 29, 21, 59, 59))
                .action(Select),
        )
        .printed_task(
            &PrintableTask::new("d", 4, Blocked)
                .due_date(ymdhms(2021, 04, 29, 22, 59, 59))
                .action(Select),
        )
        .printed_task(
            &PrintableTask::new("e", 5, Blocked)
                .due_date(ymdhms(2021, 04, 29, 23, 59, 59))
                .action(Select),
        )
        .end();
}

#[test]
fn budget_shows_affected_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 15, 00, 00);
    fix.test("todo new a b c --chain --due 5 hours");
    fix.test("todo budget c --is 2 hours")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .due_date(ymdhms(2021, 04, 29, 18, 00, 00)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .due_date(ymdhms(2021, 04, 29, 18, 00, 00)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .due_date(ymdhms(2021, 04, 29, 20, 00, 00))
                .action(Select),
        )
        .end();
}

#[test]
fn budget_does_not_show_unaffected_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 15, 00, 00);
    fix.test("todo new a b c --chain --due 5 hours");
    fix.test("todo due a --in 1 hour");
    fix.test("todo budget c --is 2 hours")
        .validate()
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .due_date(ymdhms(2021, 04, 29, 18, 00, 00)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .due_date(ymdhms(2021, 04, 29, 20, 00, 00))
                .action(Select),
        )
        .end();
}

#[test]
fn invalid_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo budget a --is blah")
        .validate()
        .printed_error(&PrintableError::CannotParseDuration {
            cannot_parse: "blah".to_string(),
        })
        .end();
}

#[test]
fn too_long_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo budget a --is 200 years")
        .validate()
        .printed_error(&PrintableError::DurationIsTooLong {
            duration: 6311520000,
            string_repr: "200 years".to_string(),
        })
        .end();
}

#[test]
fn budget_does_not_include_complete_affected_deps() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 30, 11, 00, 00);
    fix.test("todo new a b c --chain --due today");
    fix.test("todo check a");
    fix.test("todo budget c --is 1 hour")
        .validate()
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete)
                .due_date(ymdhms(2021, 04, 30, 22, 59, 59)),
        )
        .printed_task(
            &PrintableTask::new("c", 2, Blocked)
                .due_date(ymdhms(2021, 04, 30, 23, 59, 59))
                .action(Select),
        )
        .end();
}

#[test]
fn budget_include_complete_affected_deps() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 30, 11, 00, 00);
    fix.test("todo new a b c --chain --due today");
    fix.test("todo check a");
    fix.test("todo budget c --is 1 hour -d")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 0, Complete)
                .punctuality(-chrono::Duration::hours(10)),
        )
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete)
                .due_date(ymdhms(2021, 04, 30, 22, 59, 59)),
        )
        .printed_task(
            &PrintableTask::new("c", 2, Blocked)
                .due_date(ymdhms(2021, 04, 30, 23, 59, 59))
                .action(Select),
        )
        .end();
}

#[test]
fn budget_of_zero() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 1 hour");
    fix.test("todo budget a --is 0")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .budget(Duration::hours(1))
                .action(Select),
        )
        .end();
}
