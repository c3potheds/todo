#![allow(clippy::zero_prefixed_literal)]

use todo_printing::Plicit::*;
use todo_printing::PrintableError;
use todo_printing::Status::*;
use todo_testing::ymdhms;

use super::testing::task;
use super::testing::Fixture;
use super::testing::Mutated;

#[test]
fn show_tasks_due_today() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let end_of_day = ymdhms(2021, 04, 12, 23, 59, 59);
    fix.test("todo new a b c --due today");
    fix.test("todo new d e f");
    fix.test("todo due")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).due_date(Explicit(end_of_day)))
        .printed_task(&task("b", 2, Incomplete).due_date(Explicit(end_of_day)))
        .printed_task(&task("c", 3, Incomplete).due_date(Explicit(end_of_day)))
        .end();
}

#[test]
fn tasks_due_later_than_eod_not_included_without_flag() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let end_of_day = ymdhms(2021, 04, 12, 23, 59, 59);
    fix.test("todo new a b c --due today");
    fix.test("todo new d e f --due tomorrow");
    fix.test("todo due")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).due_date(Explicit(end_of_day)))
        .printed_task(&task("b", 2, Incomplete).due_date(Explicit(end_of_day)))
        .printed_task(&task("c", 3, Incomplete).due_date(Explicit(end_of_day)))
        .end();
}

#[test]
fn show_tasks_with_due_date_includes_blocked() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_5_hours = ymdhms(2021, 04, 12, 19, 00, 00);
    let in_2_days = ymdhms(2021, 04, 14, 23, 59, 59);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due -b --in 2 days")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Explicit(in_5_hours))
                .adeps_stats(1, 2),
        )
        .printed_task(
            &task("b", 5, Blocked)
                .due_date(Implicit(in_2_days))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 6, Blocked)
                .due_date(Explicit(in_2_days))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn show_tasks_with_due_date_excludes_complete_and_blocked() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_2_days = ymdhms(2021, 04, 14, 23, 59, 59);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo check a");
    fix.test("todo due --in 2 days")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("b", 1, Incomplete)
                .due_date(Implicit(in_2_days))
                .adeps_stats(1, 1),
        )
        .end();
}

#[test]
fn show_tasks_with_due_date_include_done() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_2_days = ymdhms(2021, 04, 14, 23, 59, 59);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo check a");
    fix.test("todo due --include-done --in 2 days")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 0, Complete).punctuality(-chrono::Duration::hours(5)),
        )
        .printed_task(
            &task("b", 1, Incomplete)
                .due_date(Implicit(in_2_days))
                .adeps_stats(1, 1),
        )
        .end();
}

#[test]
fn show_tasks_with_due_date_earlier_than_given_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_5_hours = ymdhms(2021, 04, 12, 19, 00, 00);
    let in_6_hours = ymdhms(2021, 04, 12, 20, 00, 00);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo new g --due 6 hours");
    fix.test("todo due --in 1 day")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Explicit(in_5_hours))
                .adeps_stats(1, 2),
        )
        .printed_task(&task("g", 2, Incomplete).due_date(Explicit(in_6_hours)))
        .end();
}

#[test]
fn show_tasks_with_due_date_earlier_than_given_date_include_done() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_5_hours = ymdhms(2021, 04, 12, 19, 00, 00);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo new g --due 6 hours");
    fix.test("todo check g");
    fix.test("todo due --in 1 day -d")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("g", 0, Complete).punctuality(-chrono::Duration::hours(6)),
        )
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Explicit(in_5_hours))
                .adeps_stats(1, 2),
        )
        .end();
}

#[test]
fn show_source_of_implicit_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_2_days = ymdhms(2021, 04, 14, 23, 59, 59);
    fix.test("todo new a --due 5 days");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f --due today");
    fix.test("todo due a")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 4, Incomplete)
                .due_date(Implicit(in_2_days))
                .adeps_stats(1, 2),
        )
        .printed_task(
            &task("b", 5, Blocked)
                .due_date(Implicit(in_2_days))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 6, Blocked)
                .due_date(Explicit(in_2_days))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn show_source_of_nonexistent_implicit_due_date() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo due a")
        .modified(Mutated::No)
        .validate()
        .end();
}

#[test]
fn show_source_of_implicit_due_date_where_some_adeps_do_not_have_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2023, 10, 14, 13, 00, 00);
    let end_of_day = ymdhms(2023, 10, 14, 23, 59, 59);
    fix.test("todo new a b c");
    fix.test("todo block b --on a");
    fix.test("todo block c --on a");
    fix.test("todo due b --on today");
    fix.test("todo due a")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Implicit(end_of_day))
                .adeps_stats(2, 2),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .due_date(Explicit(end_of_day))
                .deps_stats(1, 1),
        )
        .end();
}

#[test]
fn set_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let thursday = ymdhms(2021, 04, 15, 23, 59, 59);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due d e --on thursday")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("d", 2, Incomplete).due_date(Explicit(thursday)))
        .printed_task(&task("e", 3, Incomplete).due_date(Explicit(thursday)))
        .end();
}

#[test]
fn set_due_date_excludes_complete_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let thursday = ymdhms(2021, 04, 15, 23, 59, 59);
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo due b --on thursday")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).due_date(Explicit(thursday)))
        .end();
}

#[test]
fn set_due_date_include_done() {
    let mut fix = Fixture::default();
    // Monday.
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let thursday = ymdhms(2021, 04, 15, 23, 59, 59);
    let punctuality = fix.clock.now - thursday;
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo due b --on thursday --include-done")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).punctuality(punctuality))
        .printed_task(&task("b", 1, Incomplete).due_date(Explicit(thursday)))
        .end();
}

#[test]
fn set_due_date_prints_affected_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_1_hour = ymdhms(2021, 04, 12, 15, 00, 00);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due c --in 1 hour")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Implicit(in_1_hour))
                .adeps_stats(1, 2),
        )
        .printed_task(
            &task("b", 5, Blocked)
                .due_date(Implicit(in_1_hour))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 6, Blocked)
                .due_date(Explicit(in_1_hour))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn set_due_date_no_change() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2023, 10, 14, 13, 00, 00);
    let end_of_day = ymdhms(2023, 10, 14, 23, 59, 59);
    fix.test("todo new a --due today");
    fix.test("todo due a --on today")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).due_date(Explicit(end_of_day)))
        .end();
}

#[test]
fn reset_due_date() {
    let mut fix = Fixture::default();
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due c --none")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 5, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 6, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn show_tasks_without_due_dates() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --due today");
    fix.test("todo new d e f --due tomorrow -p a b c");
    fix.test("todo new g h i --chain");
    fix.test("todo due --none")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("g", 4, Incomplete).adeps_stats(1, 2))
        .end();
}

#[test]
fn show_tasks_without_due_date_excludes_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --due today");
    fix.test("todo new d e f --chain");
    fix.test("todo check d");
    fix.test("todo due --none")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("e", 4, Incomplete).adeps_stats(1, 1))
        .end();
}

#[test]
fn show_tasks_without_due_date_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --due today");
    fix.test("todo new d e f --chain");
    fix.test("todo check d");
    fix.test("todo due --none -d")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("d", 0, Complete))
        .printed_task(&task("e", 4, Incomplete).adeps_stats(1, 1))
        .end();
}

#[test]
fn show_tasks_without_due_date_include_blocked() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --due today");
    fix.test("todo new d e f --chain");
    fix.test("todo due --none -b")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("d", 4, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("e", 5, Blocked).deps_stats(1, 1))
        .printed_task(&task("f", 6, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn cannot_use_due_and_none_flags_at_the_same_time() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 13, 18, 00, 00);
    fix.test("todo due --in 1 day --none")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::ConflictingArgs((
            "due".to_string(),
            "none".to_string(),
        )))
        .end();
}

#[test]
fn only_shows_unblocked_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2023, 09, 30, 12, 00, 00);
    let end_of_day = ymdhms(2023, 09, 30, 23, 59, 59);
    fix.test("todo new a b --due today --chain");
    fix.test("todo due --on today")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .adeps_stats(1, 1)
                .due_date(Explicit(end_of_day)),
        )
        .end();
}

#[test]
fn shows_blocked_tasks_with_flag() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2023, 09, 30, 12, 00, 00);
    let end_of_day = ymdhms(2023, 09, 30, 23, 59, 59);
    fix.test("todo new a b --due today --chain");
    fix.test("todo due --on today -b")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .adeps_stats(1, 1)
                .due_date(Explicit(end_of_day)),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .deps_stats(1, 1)
                .due_date(Explicit(end_of_day)),
        )
        .end();
}
