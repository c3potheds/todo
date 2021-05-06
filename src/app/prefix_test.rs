use app::testing::Fixture;
use printing::PrintableTask;
use printing::Status::*;

#[test]
fn prefix_one_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo prefix a -P x")
        .validate()
        .printed_task(&PrintableTask::new("x a", 1, Incomplete))
        .end();
}

#[test]
fn prefix_three_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo prefix a b c -P y")
        .validate()
        .printed_task(&PrintableTask::new("y a", 1, Incomplete))
        .printed_task(&PrintableTask::new("y b", 2, Incomplete))
        .printed_task(&PrintableTask::new("y c", 3, Incomplete))
        .end();
}

#[test]
fn add_multiple_prefixes() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo prefix a -P x y")
        .validate()
        .printed_task(&PrintableTask::new("x y a", 1, Incomplete))
        .end();
}

#[test]
fn prefix_multiple_tasks_with_same_description() {
    let mut fix = Fixture::new();
    fix.test("todo new a a a");
    fix.test("todo prefix a -P z")
        .validate()
        .printed_task(&PrintableTask::new("z a", 1, Incomplete))
        .printed_task(&PrintableTask::new("z a", 2, Incomplete))
        .printed_task(&PrintableTask::new("z a", 3, Incomplete))
        .end();
}
