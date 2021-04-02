use app::testing::Fixture;
use model::TaskStatus::*;
use printing::Action::*;
use printing::PrintableError;
use printing::PrintableTask;

#[test]
fn put_one_after_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo put a --after b")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn put_three_after_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d");
    fix.test("todo put a b c --after d")
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("b", 3, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("c", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn put_one_after_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d");
    fix.test("todo put a --after b c d")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .printed_task(&PrintableTask::new("d", 3, Incomplete))
        .printed_task(&PrintableTask::new("a", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn put_after_task_with_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c");
    fix.test("todo put c --after a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("b", 3, Blocked).action(Lock))
        .end();
}

#[test]
fn put_one_before_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo put b --before a")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn put_three_before_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d");
    fix.test("todo put b c d --before a")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .printed_task(&PrintableTask::new("d", 3, Incomplete))
        .printed_task(&PrintableTask::new("a", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn put_one_before_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d");
    fix.test("todo put d --before a b c")
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("b", 3, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("c", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn put_before_task_with_deps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c");
    fix.test("todo put c --before b")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("b", 3, Blocked).action(Lock))
        .end();
}

#[test]
fn put_before_and_after() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo new g");
    fix.test("todo put g -b b -a e")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("e", 3, Blocked))
        .printed_task(&PrintableTask::new("g", 4, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("b", 5, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("f", 6, Blocked).action(Lock))
        .end();
}

#[test]
fn put_causing_cycle() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo put a --after b")
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: 1,
            requested_dependency: 2,
        })
        .end();
    fix.test("todo -a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .end();
}
