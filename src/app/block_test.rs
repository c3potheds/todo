use app::testing::Fixture;
use model::TaskStatus::*;
use printing::Action::*;
use printing::PrintableError;
use printing::PrintableTask;

#[test]
fn block_one_on_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo block 1 --on 2")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn block_by_name() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo block a --on b")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("a", 2, Blocked).action(Lock))
        .end();
}

#[test]
fn block_one_on_three() {
    let mut fix = Fixture::new();
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
    let mut fix = Fixture::new();
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
    let mut fix = Fixture::new();
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
    let mut fix = Fixture::new();
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
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo block 1 --on 1")
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: 1,
            requested_dependency: 1,
        })
        .end();
}
