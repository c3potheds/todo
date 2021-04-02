use app::testing::Fixture;
use model::TaskStatus::*;
use printing::Action::*;
use printing::PrintableTask;
use printing::PrintableWarning;

#[test]
fn unblock_task_from_direct_dependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo unblock 2 --from 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(Unlock))
        .end();
}

#[test]
fn unblock_task_from_indirect_dependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo block 3 --on 2");
    fix.test("todo block 2 --on 1");
    fix.test("todo unblock 3 --from 1")
        .validate()
        .printed_warning(
            &PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                cannot_unblock: 3,
                requested_unblock_from: 1,
            },
        )
        .end();
}

#[test]
fn unblock_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo check 1 2");
    fix.test("todo unblock 0 --from -1")
        .validate()
        .printed_task(&PrintableTask::new("a", -1, Complete))
        .printed_task(&PrintableTask::new("b", 0, Complete).action(Unlock))
        .end();
}

#[test]
fn unblock_by_name() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo unblock b --from a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(Unlock))
        .end();
}

#[test]
fn unblock_from_all() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo unblock c")
        .validate()
        .printed_task(&PrintableTask::new("c", 3, Incomplete).action(Unlock))
        .end();
}

#[test]
fn unblock_from_all2() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p a b");
    fix.test("todo unblock c")
        .validate()
        .printed_task(&PrintableTask::new("c", 2, Incomplete).action(Unlock))
        .end();
}
