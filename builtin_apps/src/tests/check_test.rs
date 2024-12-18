use todo_printing::Action::*;
use todo_printing::BriefPrintableTask;
use todo_printing::PrintableError;
use todo_printing::PrintableWarning;
use todo_printing::Status::*;

use super::testing::task;
use super::testing::Fixture;
use super::testing::Mutated;

#[test]
fn check_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Check))
        .end();
}

#[test]
fn check_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 0, Complete).action(Check))
        .end();
}

#[test]
fn check_task_with_incomplete_dependencies() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo check 2")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: BriefPrintableTask::new(2, Blocked),
            blocked_by: vec![BriefPrintableTask::new(1, Incomplete)],
        })
        .end();
}

#[test]
fn cannot_check_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 1 --on 2");
    fix.test("todo check 2")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: BriefPrintableTask::new(2, Blocked),
            blocked_by: vec![BriefPrintableTask::new(1, Incomplete)],
        })
        .end();
}

#[test]
fn check_newly_unblocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 1 --on 2");
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 0, Complete).action(Check))
        .printed_task(&task("a", 1, Incomplete).action(Unlock))
        .end();
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Check))
        .end();
}

#[test]
fn check_newly_unblocked_task_with_multiple_dependencies() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo block 1 --on 2 3");
    fix.test("todo check 1 2")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", -1, Complete).action(Check))
        .printed_task(&task("c", 0, Complete).action(Check))
        .printed_task(&task("a", 1, Incomplete).action(Unlock))
        .end();
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Check))
        .end();
}

#[test]
fn check_newly_unblocked_task_with_chained_dependencies() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo block 3 --on 2");
    fix.test("todo block 2 --on 1");
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Check))
        .printed_task(
            &task("b", 1, Incomplete).action(Unlock).adeps_stats(1, 1),
        )
        .end();
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 0, Complete).action(Check))
        .printed_task(&task("c", 1, Incomplete).action(Unlock))
        .end();
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("c", 0, Complete).action(Check))
        .end();
}

#[test]
fn check_does_not_show_adeps_that_are_not_unlocked() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b c -p 1 --chain");
    fix.test("todo check 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Check))
        .printed_task(
            &task("b", 1, Incomplete).action(Unlock).adeps_stats(1, 1),
        )
        // Do not print c, even though it's a direct adep, because it has not
        // been unlocked.
        .end();
}

#[test]
fn check_same_task_twice_in_one_command() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check 1 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Check))
        .end();
}

#[test]
fn check_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo check a")
        .modified(Mutated::No)
        .validate()
        .printed_warning(&PrintableWarning::CannotCheckBecauseAlreadyComplete {
            cannot_check: BriefPrintableTask::new(0, Complete),
        })
        .end();
}

#[test]
fn force_check_incomplete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check a --force")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Check))
        .end();
}

#[test]
fn force_check_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check b --force")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -1, Complete).action(Check))
        .printed_task(&task("b", 0, Complete).action(Check))
        .end();
}

#[test]
fn force_check_transitively_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check c --force")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -2, Complete).action(Check))
        .printed_task(&task("b", -1, Complete).action(Check))
        .printed_task(&task("c", 0, Complete).action(Check))
        .end();
}

#[test]
fn force_check_task_with_complete_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo check a");
    fix.test("todo check c --force")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", -1, Complete).action(Check))
        .printed_task(&task("c", 0, Complete).action(Check))
        .end();
}

#[test]
fn force_check_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo check a --force")
        .modified(Mutated::No)
        .validate()
        .printed_warning(&PrintableWarning::CannotCheckBecauseAlreadyComplete {
            cannot_check: BriefPrintableTask::new(0, Complete),
        })
        .end();
}

#[test]
fn check_blocking_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -2, Complete).action(Check))
        .printed_task(&task("b", -1, Complete).action(Check))
        .printed_task(&task("c", 0, Complete).action(Check))
        .end();
}
