use {
    super::testing::Fixture,
    printing::{
        Action::*, BriefPrintableTask, Plicit::*, PrintableError,
        PrintableTask, Status::*,
    },
};

#[test]
fn put_one_after_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo put a --after b")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn put_three_after_one() {
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo put b --before a")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn put_three_before_one() {
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo new g");
    fix.test("todo put g -B b -A e")
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
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo put a --after b")
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(2, Blocked),
        })
        .end();
    fix.test("todo -a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .end();
}

#[test]
fn put_before_prints_updated_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b d --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo put c --before d")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .priority(Explicit(1))
                .action(Lock),
        )
        .printed_task(&PrintableTask::new("d", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn put_after_prints_updated_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b d --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo put c --after b")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .priority(Explicit(1))
                .action(Lock),
        )
        .printed_task(&PrintableTask::new("d", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn put_excludes_complete_affected_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo new c --priority 1");
    fix.test("todo put c --after b")
        .validate()
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 2, Blocked)
                .priority(Explicit(1))
                .action(Lock),
        )
        .end();
}

#[test]
fn put_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo new c --priority 1");
    fix.test("todo put c --after b -d")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 0, Complete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 2, Blocked)
                .priority(Explicit(1))
                .action(Lock),
        )
        .end();
}
