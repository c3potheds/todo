use {
    super::testing::Fixture,
    printing::{Plicit::*, PrintableTask, Status::*},
};

#[test]
fn priority_set_for_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo priority a --is 1")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn priority_set_for_three_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo priority a b c --is 2")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Explicit(2)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Incomplete).priority(Explicit(2)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Incomplete).priority(Explicit(2)),
        )
        .end();
}

#[test]
fn priority_reorders_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo priority b --is 1")
        .validate()
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete).priority(Explicit(1)),
        )
        .end();
    fix.test("todo")
        .validate()
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete).priority(Explicit(1)),
        )
        .printed_task(&PrintableTask::new("a", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .end();
}

#[test]
fn priority_shows_affected_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a c");
    fix.test("todo priority d --is 1")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 2, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("d", 4, Blocked).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn priority_does_not_show_complete_affected_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a c");
    fix.test("todo check a");
    fix.test("todo priority d --is 1")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("d", 3, Blocked).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn priority_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a c");
    fix.test("todo check a");
    fix.test("todo priority d --is 1 --include-done")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 0, Complete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("d", 3, Blocked).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn priority_shows_affected_transitive_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d --chain");
    fix.test("todo priority c --is 1")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn priority_set_negative() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo priority a --is -1")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 3, Incomplete).priority(Explicit(-1)),
        )
        .end();
}

#[test]
fn priority_does_not_show_deps_with_higher_priorities() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --priority 3");
    fix.test("todo new d e f --priority 1");
    fix.test("todo new g -p a b c d e f");
    println!("Setting priority of g");
    fix.test("todo priority g --is 2")
        .validate()
        .printed_task(
            &PrintableTask::new("d", 4, Incomplete).priority(Implicit(2)),
        )
        .printed_task(
            &PrintableTask::new("e", 5, Incomplete).priority(Implicit(2)),
        )
        .printed_task(
            &PrintableTask::new("f", 6, Incomplete).priority(Implicit(2)),
        )
        .printed_task(
            &PrintableTask::new("g", 7, Blocked).priority(Explicit(2)),
        )
        .end();
}

#[test]
fn get_all_tasks_with_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d e f --priority 1");
    fix.test("todo new g h i --priority 2");
    fix.test("todo priority --is 2")
        .validate()
        .printed_task(
            &PrintableTask::new("g", 1, Incomplete).priority(Explicit(2)),
        )
        .printed_task(
            &PrintableTask::new("h", 2, Incomplete).priority(Explicit(2)),
        )
        .printed_task(
            &PrintableTask::new("i", 3, Incomplete).priority(Explicit(2)),
        )
        .end();
}

#[test]
fn get_all_tasks_with_unspecified_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d e f --priority 1");
    fix.test("todo new g h i --priority 2");
    fix.test("todo priority --is 1")
        .validate()
        .printed_task(
            &PrintableTask::new("g", 1, Incomplete).priority(Explicit(2)),
        )
        .printed_task(
            &PrintableTask::new("h", 2, Incomplete).priority(Explicit(2)),
        )
        .printed_task(
            &PrintableTask::new("i", 3, Incomplete).priority(Explicit(2)),
        )
        .printed_task(
            &PrintableTask::new("d", 4, Incomplete).priority(Explicit(1)),
        )
        .printed_task(
            &PrintableTask::new("e", 5, Incomplete).priority(Explicit(1)),
        )
        .printed_task(
            &PrintableTask::new("f", 6, Incomplete).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn explain_source_of_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo priority c --is 1");
    fix.test("todo priority a")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn explain_source_of_priority_deep() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f g h i --chain");
    fix.test("todo priority g --is 1");
    fix.test("todo priority a")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("d", 4, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("e", 5, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("f", 6, Blocked).priority(Implicit(1)),
        )
        .printed_task(
            &PrintableTask::new("g", 7, Blocked).priority(Explicit(1)),
        )
        .end();
}
