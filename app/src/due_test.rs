#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::task,
    super::testing::Fixture,
    printing::{Plicit::*, PrintableError, Status::*},
    testing::ymdhms,
};

#[test]
fn show_tasks_with_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let in_1_day = ymdhms(2021, 04, 13, 23, 59, 59);
    fix.test("todo new a b c --due 1 day");
    fix.test("todo new d e f");
    fix.test("todo due")
        .modified(false)
        .validate()
        .printed_task(&task("a", 1, Incomplete).due_date(Explicit(in_1_day)))
        .printed_task(&task("b", 2, Incomplete).due_date(Explicit(in_1_day)))
        .printed_task(&task("c", 3, Incomplete).due_date(Explicit(in_1_day)))
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
    fix.test("todo due -b")
        .modified(false)
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
    fix.test("todo due")
        .modified(false)
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
    fix.test("todo due --include-done")
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
fn set_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 14, 00, 00);
    let thursday = ymdhms(2021, 04, 15, 23, 59, 59);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due d e --on thursday")
        .modified(true)
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
        .modified(true)
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
        .modified(true)
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
        .modified(true)
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
fn reset_due_date() {
    let mut fix = Fixture::default();
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due c --none")
        .modified(true)
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
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
