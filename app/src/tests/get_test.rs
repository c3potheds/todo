use {
    super::testing::task,
    super::testing::Fixture,
    todo_printing::{Action::*, Status::*},
};

#[test]
fn get_incomplete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo get 2")
        .modified(false)
        .validate()
        .printed_task(&task("b", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn get_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check 1 2 3");
    fix.test("todo get -2")
        .modified(false)
        .validate()
        .printed_task(&task("a", -2, Complete).action(Select))
        .end();
}

#[test]
fn get_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo get 2 3 4")
        .modified(false)
        .validate()
        .printed_task(&task("b", 2, Incomplete).action(Select))
        .printed_task(&task("c", 3, Incomplete).action(Select))
        .printed_task(&task("d", 4, Incomplete).action(Select))
        .end();
}

#[test]
fn get_excludes_completed_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo get b")
        .modified(false)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn get_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo get b -d")
        .modified(false)
        .validate()
        .printed_task(&task("a", 0, Complete))
        .printed_task(&task("b", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn get_shows_blocking_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo get 2")
        .modified(false)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("b", 2, Blocked).action(Select).deps_stats(1, 1))
        .end();
}

#[test]
fn get_shows_blocked_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo get 1")
        .modified(false)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).action(Select).adeps_stats(1, 1),
        )
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn get_shows_transitive_deps_and_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo get 3")
        .modified(false)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 4))
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).action(Select).deps_stats(1, 2))
        .printed_task(&task("d", 4, Blocked).deps_stats(1, 3))
        .printed_task(&task("e", 5, Blocked).deps_stats(1, 4))
        .end();
}

#[test]
fn get_by_name_multiple_matches() {
    let mut fix = Fixture::default();
    fix.test("todo new bob frank bob");
    fix.test("todo get bob")
        .modified(false)
        .validate()
        .printed_task(&task("bob", 1, Incomplete).action(Select))
        .printed_task(&task("bob", 3, Incomplete).action(Select))
        .end();
}

#[test]
fn get_no_context_single_task_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo get a -n")
        .modified(false)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).action(Select).adeps_stats(1, 2),
        )
        .end();
}

#[test]
fn get_no_context_multiple_tasks_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo get a b -n")
        .modified(false)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).action(Select).adeps_stats(1, 2),
        )
        .printed_task(&task("b", 2, Blocked).action(Select).deps_stats(1, 1))
        .end();
}

#[test]
fn get_no_context_single_completed_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b c");
    fix.test("todo get a -n")
        .modified(false)
        .validate()
        .printed_task(&task("a", -2, Complete).action(Select))
        .end();
}

#[test]
fn get_no_context_multiple_completed_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b c");
    fix.test("todo get a b -n")
        .modified(false)
        .validate()
        .printed_task(&task("a", -2, Complete).action(Select))
        .printed_task(&task("b", -1, Complete).action(Select))
        .end();
}

#[test]
fn get_no_context_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo get c -n")
        .modified(false)
        .validate()
        .printed_task(&task("c", 3, Blocked).action(Select).deps_stats(1, 2))
        .end();
}

#[test]
fn get_no_context_complete_and_incomplete_match() {
    let mut fix = Fixture::default();
    fix.test("todo new a b a --chain");
    fix.test("todo check 1");
    fix.test("todo get a -n")
        .modified(false)
        .validate()
        .printed_task(&task("a", 0, Complete).action(Select))
        .printed_task(&task("a", 2, Blocked).action(Select).deps_stats(1, 2))
        .end();
}

#[test]
fn get_blocked_by_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo get --blocked-by b")
        .modified(false)
        .validate()
        .printed_task(&task("b", 2, Blocked).action(Select).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn get_blocked_by_shows_transitive_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo get --blocked-by b")
        .modified(false)
        .validate()
        .printed_task(&task("b", 2, Blocked).action(Select).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).deps_stats(1, 2))
        .printed_task(&task("d", 4, Blocked).deps_stats(1, 3))
        .printed_task(&task("e", 5, Blocked).deps_stats(1, 4))
        .end();
}

#[test]
fn get_blocking_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo get --blocking b")
        .modified(false)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("b", 2, Blocked).action(Select).deps_stats(1, 1))
        .end();
}

#[test]
fn get_blocking_shows_transitive_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo get --blocking d")
        .modified(false)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 4))
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).deps_stats(1, 2))
        .printed_task(&task("d", 4, Blocked).action(Select).deps_stats(1, 3))
        .end();
}