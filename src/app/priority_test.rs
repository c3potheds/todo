use app::testing::Fixture;
use model::TaskStatus::*;
use printing::PrintableTask;

#[test]
fn priority_set_for_one_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo priority a --is 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).priority(1))
        .end();
}

#[test]
fn priority_set_for_three_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo priority a b c --is 2")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).priority(2))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).priority(2))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).priority(2))
        .end();
}

#[test]
fn priority_reorders_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo priority b --is 1")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).priority(1))
        .end();
    fix.test("todo")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).priority(1))
        .printed_task(&PrintableTask::new("a", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .end();
}

#[test]
fn priority_shows_affected_deps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo new d -p a c");
    fix.test("todo priority d --is 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).priority(1))
        .printed_task(&PrintableTask::new("c", 2, Incomplete).priority(1))
        .printed_task(&PrintableTask::new("d", 4, Blocked).priority(1))
        .end();
}

#[test]
fn priority_shows_affected_transitive_deps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d --chain");
    fix.test("todo priority c --is 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).priority(1))
        .printed_task(&PrintableTask::new("b", 2, Blocked).priority(1))
        .printed_task(&PrintableTask::new("c", 3, Blocked).priority(1))
        .end();
}

#[test]
fn priority_set_negative() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo priority a --is -1")
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Incomplete).priority(-1))
        .end();
}

#[test]
fn priority_does_not_show_deps_with_higher_priorities() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --priority 3");
    fix.test("todo new d e f --priority 1");
    fix.test("todo new g -p a b c d e f");
    println!("Setting priority of g");
    fix.test("todo priority g --is 2")
        .validate()
        .printed_task(&PrintableTask::new("d", 4, Incomplete).priority(2))
        .printed_task(&PrintableTask::new("e", 5, Incomplete).priority(2))
        .printed_task(&PrintableTask::new("f", 6, Incomplete).priority(2))
        .printed_task(&PrintableTask::new("g", 7, Blocked).priority(2))
        .end();
}