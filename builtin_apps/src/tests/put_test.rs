use todo_app::Mutated;
use todo_printing::Action::*;
use todo_printing::BriefPrintableTask;
use todo_printing::Plicit::*;
use todo_printing::PrintableError;
use todo_printing::Status::*;

use super::testing::task;
use super::testing::Fixture;

#[test]
fn put_one_after_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo put a --after b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn put_three_after_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo put a b c --after d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("d", 1, Incomplete).adeps_stats(3, 3))
        .printed_task(&task("a", 2, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn put_one_after_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo put a --after b c d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("c", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("d", 3, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("a", 4, Blocked).action(Lock).deps_stats(3, 3))
        .end();
}

#[test]
fn put_after_task_with_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c");
    fix.test("todo put c --after a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("c", 2, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).action(Lock).deps_stats(1, 2))
        .end();
}

#[test]
fn put_one_before_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo put b --before a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn put_three_before_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo put b c d --before a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("c", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("d", 3, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("a", 4, Blocked).action(Lock).deps_stats(3, 3))
        .end();
}

#[test]
fn put_one_before_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo put d --before a b c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("d", 1, Incomplete).adeps_stats(3, 3))
        .printed_task(&task("a", 2, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn put_before_task_with_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c");
    fix.test("todo put c --before b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("c", 2, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).action(Lock).deps_stats(1, 2))
        .end();
}

#[test]
fn put_before_and_after() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo new g");
    fix.test("todo put g -B b -A e")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 4))
        .printed_task(&task("e", 3, Blocked).deps_stats(1, 1))
        .printed_task(&task("g", 4, Blocked).action(Lock).deps_stats(2, 3))
        .printed_task(&task("b", 5, Blocked).action(Lock).deps_stats(2, 4))
        .printed_task(&task("f", 6, Blocked).action(Lock).deps_stats(2, 4))
        .end();
}

#[test]
fn put_causing_cycle() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo put a --after b")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(2, Blocked),
        })
        .end();
    fix.test("todo -a")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn put_before_prints_updated_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b d --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo put c --before d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 3),
        )
        .printed_task(
            &task("b", 2, Blocked).priority(Implicit(1)).deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .priority(Explicit(1))
                .action(Lock)
                .deps_stats(1, 2),
        )
        .printed_task(&task("d", 4, Blocked).action(Lock).deps_stats(1, 3))
        .end();
}

#[test]
fn put_after_prints_updated_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b d --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo put c --after b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 3),
        )
        .printed_task(
            &task("b", 2, Blocked).priority(Implicit(1)).deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .priority(Explicit(1))
                .action(Lock)
                .deps_stats(1, 2),
        )
        .printed_task(&task("d", 4, Blocked).action(Lock).deps_stats(1, 3))
        .end();
}

#[test]
fn put_excludes_complete_affected_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo new c --priority 1");
    fix.test("todo put c --after b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("b", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &task("c", 2, Blocked)
                .priority(Explicit(1))
                .action(Lock)
                .deps_stats(1, 2),
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
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).priority(Implicit(1)))
        .printed_task(
            &task("b", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &task("c", 2, Blocked)
                .priority(Explicit(1))
                .action(Lock)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn put_task_by_initial_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new t");
    fix.test("todo put t --by a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("t", 2, Incomplete).adeps_stats(0, 2))
        .printed_task(&task("b", 3, Blocked).action(Lock).deps_stats(2, 2))
        .end();
}

#[test]
fn put_task_by_interior_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new t");
    fix.test("todo put t --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(2, 3))
        .printed_task(&task("t", 3, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).action(Lock).deps_stats(1, 3))
        .end();
}

#[test]
fn put_task_by_terminal_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new t");
    fix.test("todo put t --by c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("t", 4, Blocked).action(Lock).deps_stats(1, 2))
        .end();
}

#[test]
fn put_task_by_isolated_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new t");
    fix.test("todo put t --by a")
        .modified(Mutated::No)
        .validate()
        .end();
}

#[test]
fn put_task_by_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b");
    fix.test("todo new t");
    fix.test("todo put t --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("t", 1, Incomplete).action(Lock).adeps_stats(1, 1))
        .printed_task(&task("c", 2, Blocked).action(Lock).deps_stats(1, 3))
        .end();
}

#[test]
fn put_task_by_complete_task_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b");
    fix.test("todo new t");
    fix.test("todo put t --by b -d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -1, Complete))
        .printed_task(&task("t", 1, Incomplete).action(Lock).adeps_stats(1, 1))
        .printed_task(&task("c", 2, Blocked).action(Lock).deps_stats(1, 3))
        .end();
}

#[test]
fn put_task_by_task_whose_adeps_are_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b c");
    fix.test("todo new t");
    fix.test("todo put t --by b -d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -1, Complete))
        .printed_task(&task("t", 1, Incomplete).action(Lock).adeps_stats(1, 1))
        .printed_task(&task("c", 2, Blocked).action(Lock).deps_stats(1, 3))
        .end();
}

#[test]
fn put_complete_task_by_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new t -d");
    fix.test("todo put t --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(2, 3))
        .printed_task(&task("t", 3, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).action(Lock).deps_stats(1, 3))
        .end();
}
