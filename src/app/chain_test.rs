use app::testing::Fixture;
use model::TaskStatus::*;
use printing::Action::*;
use printing::PrintableError;
use printing::PrintableTask;

#[test]
fn chain_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo chain a").validate().end();
}

#[test]
fn chain_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e");
    fix.test("todo chain a b c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 4, Blocked).action(Lock))
        .printed_task(&PrintableTask::new("c", 5, Blocked).action(Lock))
        .end();
}

#[test]
fn chain_would_cause_cycle() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo chain b a")
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: 1,
            requested_dependency: 2,
        })
        .end();
}
