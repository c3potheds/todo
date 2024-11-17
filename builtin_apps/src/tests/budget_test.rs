#![allow(clippy::zero_prefixed_literal)]

use chrono::Duration;
use todo_app::Mutated;
use todo_printing::Action::*;
use todo_printing::Plicit::*;
use todo_printing::PrintableError;
use todo_printing::Status::*;
use todo_testing::ymdhms;

use super::testing::task;
use super::testing::Fixture;

#[test]
fn budget_one_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 15, 00, 00);
    let in_2_days = ymdhms(2021, 05, 01, 23, 59, 59);
    fix.test("todo new a --due 2 days");
    fix.test("todo budget a --is 1 day")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Explicit(in_2_days))
                .budget(Duration::days(1))
                .action(Select),
        )
        .end();
}

#[test]
fn budget_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo budget a c e --is 2 days")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .budget(Duration::days(2))
                .action(Select),
        )
        .printed_task(
            &task("c", 3, Incomplete)
                .budget(Duration::days(2))
                .action(Select),
        )
        .printed_task(
            &task("e", 5, Incomplete)
                .budget(Duration::days(2))
                .action(Select),
        )
        .end();
}

#[test]
fn budget_chain_alters_due_dates() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 12, 00, 00);
    fix.test("todo new a b c d e --chain --due today");
    fix.test("todo budget a b c d e --is 1 hour")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Implicit(ymdhms(2021, 04, 29, 19, 59, 59)))
                .budget(Duration::hours(1))
                .action(Select)
                .adeps_stats(1, 4),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .due_date(Implicit(ymdhms(2021, 04, 29, 20, 59, 59)))
                .budget(Duration::hours(1))
                .action(Select)
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .due_date(Implicit(ymdhms(2021, 04, 29, 21, 59, 59)))
                .budget(Duration::hours(1))
                .action(Select)
                .deps_stats(1, 2),
        )
        .printed_task(
            &task("d", 4, Blocked)
                .due_date(Implicit(ymdhms(2021, 04, 29, 22, 59, 59)))
                .budget(Duration::hours(1))
                .action(Select)
                .deps_stats(1, 3),
        )
        .printed_task(
            &task("e", 5, Blocked)
                .due_date(Explicit(ymdhms(2021, 04, 29, 23, 59, 59)))
                .budget(Duration::hours(1))
                .action(Select)
                .deps_stats(1, 4),
        )
        .end();
}

#[test]
fn budget_shows_affected_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 15, 00, 00);
    fix.test("todo new a b c --chain --due 5 hours");
    fix.test("todo budget c --is 2 hours")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Implicit(ymdhms(2021, 04, 29, 18, 00, 00)))
                .adeps_stats(1, 2),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .due_date(Implicit(ymdhms(2021, 04, 29, 18, 00, 00)))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .due_date(Explicit(ymdhms(2021, 04, 29, 20, 00, 00)))
                .budget(Duration::hours(2))
                .action(Select)
                .deps_stats(1, 2),
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
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("b", 2, Blocked)
                .due_date(Implicit(ymdhms(2021, 04, 29, 18, 00, 00)))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .due_date(Explicit(ymdhms(2021, 04, 29, 20, 00, 00)))
                .budget(Duration::hours(2))
                .action(Select)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn invalid_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo budget a --is blah")
        .modified(Mutated::No)
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
        .modified(Mutated::No)
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
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("b", 1, Incomplete)
                .due_date(Implicit(ymdhms(2021, 04, 30, 22, 59, 59)))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &task("c", 2, Blocked)
                .due_date(Explicit(ymdhms(2021, 04, 30, 23, 59, 59)))
                .budget(Duration::hours(1))
                .action(Select)
                .deps_stats(1, 2),
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
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).punctuality(
            -chrono::Duration::hours(12) + chrono::Duration::seconds(1),
        ))
        .printed_task(
            &task("b", 1, Incomplete)
                .due_date(Implicit(ymdhms(2021, 04, 30, 22, 59, 59)))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &task("c", 2, Blocked)
                .due_date(Explicit(ymdhms(2021, 04, 30, 23, 59, 59)))
                .budget(Duration::hours(1))
                .action(Select)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn budget_of_zero() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 1 hour");
    fix.test("todo budget a --is 0")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select))
        .end();
}
