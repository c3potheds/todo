#![allow(clippy::zero_prefixed_literal)]

use chrono::Duration;

use {
    super::testing::Fixture,
    printing::{Action::*, PrintableTask, Status::*},
    testing::ymdhms,
};

#[test]
fn split_one_into_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into a1 a2 a3")
        .validate()
        .printed_task(&PrintableTask::new("a1", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a2", 2, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a3", 3, Incomplete).action(New))
        .end();
}

#[test]
fn split_chained() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into a1 a2 a3 --chain")
        .validate()
        .printed_task(&PrintableTask::new("a1", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a2", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("a3", 3, Blocked).action(New))
        .end();
}

#[test]
fn split_preserves_dependency_structure() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo split b --into b1 b2 b3")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b1", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b2", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("b3", 4, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 5, Blocked))
        .end();
}

#[test]
fn split_with_prefix() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into x y -P a")
        .validate()
        .printed_task(&PrintableTask::new("a x", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a y", 2, Incomplete).action(New))
        .end();
}

#[test]
fn split_with_multiple_prefixes() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into x y -P a -P b")
        .validate()
        .printed_task(&PrintableTask::new("a b x", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a b y", 2, Incomplete).action(New))
        .end();
}

#[test]
fn split_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 30, 09, 00, 00);
    fix.test("todo new a --snooze 1 day");
    fix.test("todo split a --into x y")
        .validate()
        .printed_task(
            &PrintableTask::new("x", 1, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 31, 00, 00, 00)),
        )
        .printed_task(
            &PrintableTask::new("y", 2, Blocked)
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
        .validate()
        .printed_task(
            &PrintableTask::new("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(1)),
        )
        .printed_task(
            &PrintableTask::new("y", 2, Blocked)
                .action(New)
                .budget(Duration::hours(1)),
        )
        .printed_task(
            &PrintableTask::new("z", 3, Blocked)
                .action(New)
                .budget(Duration::hours(1)),
        )
        .end();
}

#[test]
fn split_task_with_budget_keeps_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 3 hours");
    fix.test("todo split a --into x y z")
        .validate()
        .printed_task(
            &PrintableTask::new("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .printed_task(
            &PrintableTask::new("y", 2, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .printed_task(
            &PrintableTask::new("z", 3, Incomplete)
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
        .validate()
        .printed_task(&PrintableTask::new("x", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("y", 2, Incomplete).action(New))
        .printed_task(&PrintableTask::new("z", 3, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 4, Blocked).action(Select))
        .end();
}

#[test]
fn split_task_keep_chained() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo split a --into x y z --keep --chain")
        .validate()
        .printed_task(&PrintableTask::new("x", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("y", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("z", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("a", 4, Blocked).action(Select))
        .end();
}

#[test]
fn split_task_keep_with_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 3 hours");
    fix.test("todo split a --into x y z --keep")
        .validate()
        .printed_task(
            &PrintableTask::new("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .printed_task(
            &PrintableTask::new("y", 2, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .printed_task(
            &PrintableTask::new("z", 3, Incomplete)
                .action(New)
                .budget(Duration::hours(3)),
        )
        .printed_task(&PrintableTask::new("a", 4, Blocked).action(Select))
        .end();
}

#[test]
fn split_task_chain_keep_with_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 3 hours");
    fix.test("todo split a --into x y z --keep --chain")
        .validate()
        .printed_task(
            &PrintableTask::new("x", 1, Incomplete)
                .action(New)
                .budget(Duration::hours(1)),
        )
        .printed_task(
            &PrintableTask::new("y", 2, Blocked)
                .action(New)
                .budget(Duration::hours(1)),
        )
        .printed_task(
            &PrintableTask::new("z", 3, Blocked)
                .action(New)
                .budget(Duration::hours(1)),
        )
        .printed_task(&PrintableTask::new("a", 4, Blocked).action(Select))
        .end();
}

#[test]
fn split_tag_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo split a --into x y z")
        .validate()
        .printed_task(
            &PrintableTask::new("x", 1, Incomplete).action(New).as_tag(),
        )
        .printed_task(
            &PrintableTask::new("y", 2, Incomplete).action(New).as_tag(),
        )
        .printed_task(
            &PrintableTask::new("z", 3, Incomplete).action(New).as_tag(),
        )
        .end();
}

#[test]
fn split_tag_into_non_tags() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo split a --into x y z --tag false")
        .validate()
        .printed_task(&PrintableTask::new("x", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("y", 2, Incomplete).action(New))
        .printed_task(&PrintableTask::new("z", 3, Incomplete).action(New))
        .end();
}

#[test]
fn split_tag_keep() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo split a --into x y z --keep")
        .validate()
        .printed_task(
            &PrintableTask::new("x", 1, Incomplete).action(New).tag("a"),
        )
        .printed_task(
            &PrintableTask::new("y", 2, Incomplete).action(New).tag("a"),
        )
        .printed_task(
            &PrintableTask::new("z", 3, Incomplete).action(New).tag("a"),
        )
        .printed_task(
            &PrintableTask::new("a", 4, Blocked).action(Select).as_tag(),
        )
        .end();
}
