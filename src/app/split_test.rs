use app::testing::Fixture;
use printing::Action::*;
use printing::PrintableTask;
use printing::Status::*;

#[test]
fn split_one_into_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo split a --into a1 a2 a3")
        .validate()
        .printed_task(&PrintableTask::new("a1", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("a2", 2, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("a3", 3, Incomplete).action(Select))
        .end();
}

#[test]
fn split_chained() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo split a --into a1 a2 a3 --chain")
        .validate()
        .printed_task(&PrintableTask::new("a1", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("a2", 2, Blocked).action(Select))
        .printed_task(&PrintableTask::new("a3", 3, Blocked).action(Select))
        .end();
}

#[test]
fn split_preserves_dependency_structure() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo split b --into b1 b2 b3")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b1", 2, Blocked).action(Select))
        .printed_task(&PrintableTask::new("b2", 3, Blocked).action(Select))
        .printed_task(&PrintableTask::new("b3", 4, Blocked).action(Select))
        .printed_task(&PrintableTask::new("c", 5, Blocked))
        .end();
}

#[test]
fn split_with_prefix() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo split a --into x y -P a")
        .validate()
        .printed_task(&PrintableTask::new("a x", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("a y", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn split_with_multiple_prefixes() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo split a --into x y -P a -P b")
        .validate()
        .printed_task(
            &PrintableTask::new("a b x", 1, Incomplete).action(Select),
        )
        .printed_task(
            &PrintableTask::new("a b y", 2, Incomplete).action(Select),
        )
        .end();
}
