use crate::{
    app::testing::Fixture,
    printing::{PrintableTask, Status::*},
};

#[test]
fn no_tasks_snoozed() {
    let mut fix = Fixture::default();
    fix.test("todo snoozed").validate().end();
}

#[test]
fn one_snoozed_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo snooze b --until 1 hour");
    fix.test("todo snoozed")
        .validate()
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn multiple_snoozed_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo snooze a b c --until 1 hour");
    fix.test("todo snoozed")
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Blocked))
        .printed_task(&PrintableTask::new("b", 4, Blocked))
        .printed_task(&PrintableTask::new("c", 5, Blocked))
        .end();
}
