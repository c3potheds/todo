use crate::{
    app::testing::Fixture,
    printing::{
        Action::*, BriefPrintableTask, PrintableError, PrintableTask, Status::*,
    },
};

#[test]
fn block_one_on_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 1 --on 2")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn block_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block a --on b")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn block_one_on_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo block 1 --on 2 3 4")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .printed_task(&PrintableTask::new("d", 3, Incomplete))
        .printed_task(&PrintableTask::new("a", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn block_three_on_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo block 1 2 3 --on 4")
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("b", 3, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("c", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn block_on_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check 1 2");
    fix.test("todo block 1 --on -1")
        .validate()
        .printed_task(&PrintableTask::new("a", -1, Complete))
        .printed_task(&PrintableTask::new("c", 1, Incomplete).action(Lock))
        .end();
}

#[test]
fn block_multiple_on_following_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo block 1 2 --on 3")
        .validate()
        .printed_task(&PrintableTask::new("c", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 3, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("b", 4, Blocked).action(Lock))
        .end();
}

#[test]
fn cannot_block_on_self() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo block 1 --on 1")
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(1, Incomplete),
        })
        .end();
}

#[test]
fn block_updates_implicit_priority_of_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo block c --on b")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).priority(1))
        .printed_task(&PrintableTask::new("b", 2, Blocked).priority(1))
        .printed_task(
            &PrintableTask::new("c", 3, Blocked).action(Lock).priority(1),
        )
        .end();
}

#[test]
fn block_does_not_print_priority_updates_for_unaffected_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain --priority 1");
    fix.test("todo new c --priority 1");
    fix.test("todo block c --on b")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked).priority(1))
        .printed_task(
            &PrintableTask::new("c", 3, Blocked).action(Lock).priority(1),
        )
        .end();
}

#[test]
fn block_excludes_complete_affected_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo check a");
    fix.test("todo block c --on b")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).priority(1))
        .printed_task(
            &PrintableTask::new("c", 2, Blocked).action(Lock).priority(1),
        )
        .end();
}

#[test]
fn block_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo check a");
    fix.test("todo block c --on b -d")
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete).priority(1))
        .printed_task(&PrintableTask::new("b", 1, Incomplete).priority(1))
        .printed_task(
            &PrintableTask::new("c", 2, Blocked).action(Lock).priority(1),
        )
        .end();
}

#[test]
fn block_complete_task_on_preceding_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --done");
    fix.test("todo block b --on a")
        .validate()
        .printed_task(&PrintableTask::new("a", -1, Complete))
        .printed_task(&PrintableTask::new("b", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_complete_task_on_distantly_preceding_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --done");
    fix.test("todo block e --on a")
        .validate()
        .printed_task(&PrintableTask::new("a", -4, Complete))
        .printed_task(&PrintableTask::new("e", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_complete_task_on_later_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --done");
    fix.test("todo block a --on b")
        .validate()
        .printed_task(&PrintableTask::new("b", -1, Complete))
        .printed_task(&PrintableTask::new("a", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_complete_task_on_distant_later_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --done");
    fix.test("todo block a --on e")
        .validate()
        .printed_task(&PrintableTask::new("e", -1, Complete))
        .printed_task(&PrintableTask::new("a", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_multiple_complete_tasks_on_later_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d --done");
    fix.test("todo block a b --on d")
        .validate()
        .printed_task(&PrintableTask::new("d", -2, Complete))
        .printed_task(&PrintableTask::new("a", -1, Complete).action(Lock))
        .printed_task(&PrintableTask::new("b", 0, Complete).action(Lock))
        .end();
}
