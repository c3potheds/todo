use {
    super::testing::Fixture,
    printing::{Action::*, PrintableTask, Status::*},
};

#[test]
fn rm_nonexistent_task() {
    let mut fix = Fixture::default();
    fix.test("todo rm a").modified(false).validate().end();
}

#[test]
fn rm_only_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo rm a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Removed).action(Delete))
        .end();
}

#[test]
fn rm_task_with_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo rm a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Removed).action(Delete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .end();
}

#[test]
fn rm_task_with_deps_and_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo rm b")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Removed).action(Delete))
        .printed_task(&PrintableTask::new("c", 2, Blocked))
        .end();
}

#[test]
fn rm_three_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo rm a c e")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Removed).action(Delete))
        .printed_task(&PrintableTask::new("c", 3, Removed).action(Delete))
        .printed_task(&PrintableTask::new("e", 5, Removed).action(Delete))
        .end();
    fix.test("todo")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Incomplete))
        .end();
}

#[test]
fn rm_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo rm a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Removed).action(Delete))
        .end();
}
