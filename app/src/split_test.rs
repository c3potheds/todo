#![allow(clippy::zero_prefixed_literal)]

use chrono::Duration;

use {
    super::testing::task,
    super::testing::Fixture,
    todo_printing::{Action::*, Status::*},
    todo_testing::ymdhms,
};

#[test]
fn split_one_into_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into a1 a2 a3")
        .modified(true)
        .validate()
        .printed_task(&task("a1", 1, Incomplete).action(New))
        .printed_task(&task("a2", 2, Incomplete).action(New))
        .printed_task(&task("a3", 3, Incomplete).action(New))
        .end();
}

#[test]
fn split_chained() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into a1 a2 a3 --chain")
        .modified(true)
        .validate()
        .printed_task(&task("a1", 1, Incomplete).action(New).adeps_stats(1, 2))
        .printed_task(&task("a2", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("a3", 3, Blocked).action(New).deps_stats(1, 2))
        .end();
}

#[test]
fn split_preserves_dependency_structure() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo split b --into b1 b2 b3")
        .modified(true)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(3, 4))
        .printed_task(&task("b1", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("b2", 3, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("b3", 4, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("c", 5, Blocked).deps_stats(1, 4))
        .end();
}

#[test]
fn split_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 30, 09, 00, 00);
    fix.test("todo new a --snooze 1 day");
    fix.test("todo split a --into x y")
        .modified(true)
        .validate()
        .printed_task(
            &task("x", 1, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 31, 00, 00, 00)),
        )
        .printed_task(
            &task("y", 2, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 31, 00, 00, 00)),
        )
        .end();
}

#[test]
fn chained_split_task_with_budget_distributes_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 3 hours");
    fix.test("todo split a --into x y z --chain")
        .modified(true)
        .validate()
        .printed_task(
            &task("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(1))
                .adeps_stats(1, 2),
        )
        .printed_task(
            &task("y", 2, Blocked)
                .action(New)
                .budget(Duration::hours(1))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("z", 3, Blocked)
                .action(New)
                .budget(Duration::hours(1))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn split_task_with_budget_keeps_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 3 hours");
    fix.test("todo split a --into x y z")
        .modified(true)
        .validate()
        .printed_task(
            &task("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .printed_task(
            &task("y", 2, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .printed_task(
            &task("z", 3, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .end();
}

#[test]
fn split_task_keep() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into x y z --keep")
        .modified(true)
        .validate()
        .printed_task(&task("x", 1, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("y", 2, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("z", 3, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("a", 4, Blocked).action(Select).deps_stats(3, 3))
        .end();
}

#[test]
fn split_task_keep_chained() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into x y z --keep --chain")
        .modified(true)
        .validate()
        .printed_task(&task("x", 1, Incomplete).action(New).adeps_stats(1, 3))
        .printed_task(&task("y", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("z", 3, Blocked).action(New).deps_stats(1, 2))
        .printed_task(&task("a", 4, Blocked).action(Select).deps_stats(1, 3))
        .end();
}

#[test]
fn split_task_keep_with_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 3 hours");
    fix.test("todo split a --into x y z --keep")
        .modified(true)
        .validate()
        .printed_task(
            &task("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(3))
                .adeps_stats(0, 1),
        )
        .printed_task(
            &task("y", 2, Incomplete)
                .action(New)
                .budget(Duration::hours(3))
                .adeps_stats(0, 1),
        )
        .printed_task(
            &task("z", 3, Incomplete)
                .action(New)
                .budget(Duration::hours(3))
                .adeps_stats(0, 1),
        )
        .printed_task(&task("a", 4, Blocked).action(Select).deps_stats(3, 3))
        .end();
}

#[test]
fn split_task_chain_keep_with_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 3 hours");
    fix.test("todo split a --into x y z --keep --chain")
        .modified(true)
        .validate()
        .printed_task(
            &task("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(1))
                .adeps_stats(1, 3),
        )
        .printed_task(
            &task("y", 2, Blocked)
                .action(New)
                .budget(Duration::hours(1))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("z", 3, Blocked)
                .action(New)
                .budget(Duration::hours(1))
                .deps_stats(1, 2),
        )
        .printed_task(&task("a", 4, Blocked).action(Select).deps_stats(1, 3))
        .end();
}

#[test]
fn split_tag_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo split a --into x y z")
        .modified(true)
        .validate()
        .printed_task(&task("x", 1, Incomplete).action(New).as_tag())
        .printed_task(&task("y", 2, Incomplete).action(New).as_tag())
        .printed_task(&task("z", 3, Incomplete).action(New).as_tag())
        .end();
}

#[test]
fn split_tag_into_non_tags() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo split a --into x y z --tag false")
        .modified(true)
        .validate()
        .printed_task(&task("x", 1, Incomplete).action(New))
        .printed_task(&task("y", 2, Incomplete).action(New))
        .printed_task(&task("z", 3, Incomplete).action(New))
        .end();
}

#[test]
fn split_tag_keep() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo split a --into x y z --keep")
        .modified(true)
        .validate()
        .printed_task(
            &task("x", 1, Incomplete)
                .action(New)
                .tag("a")
                .adeps_stats(0, 1),
        )
        .printed_task(
            &task("y", 2, Incomplete)
                .action(New)
                .tag("a")
                .adeps_stats(0, 1),
        )
        .printed_task(
            &task("z", 3, Incomplete)
                .action(New)
                .tag("a")
                .adeps_stats(0, 1),
        )
        .printed_task(
            &task("a", 4, Blocked)
                .action(Select)
                .as_tag()
                .deps_stats(3, 3),
        )
        .end();
}

#[test]
fn trim_leading_whitespace() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into ' a.x' '  a.y' '   a.z' --keep")
        .modified(true)
        .validate()
        .printed_task(&task("a.x", 1, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("a.y", 2, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("a.z", 3, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("a", 4, Blocked).action(Select).deps_stats(3, 3))
        .end();
}

#[test]
fn trim_trailing_whitespace() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into 'a.x ' 'a.y  ' 'a.z   ' --keep")
        .modified(true)
        .validate()
        .printed_task(&task("a.x", 1, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("a.y", 2, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("a.z", 3, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("a", 4, Blocked).action(Select).deps_stats(3, 3))
        .end();
}
