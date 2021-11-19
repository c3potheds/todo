use app::testing::Fixture;
use printing::Action::*;
use printing::PrintableTask;
use printing::Status::*;

#[test]
fn get_incomplete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo get 2")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn get_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check 1 2 3");
    fix.test("todo get -2")
        .validate()
        .printed_task(&PrintableTask::new("a", -2, Complete).action(Select))
        .end();
}

#[test]
fn get_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo get 2 3 4")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("d", 4, Incomplete).action(Select))
        .end();
}

#[test]
fn get_excludes_completed_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo get b")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn get_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a");
    fix.test("todo get b -d")
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn get_shows_blocking_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo get 2")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(Select))
        .end();
}

#[test]
fn get_shows_blocked_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo get 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .end();
}

#[test]
fn get_shows_transitive_deps_and_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo get 3")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(Select))
        .printed_task(&PrintableTask::new("d", 4, Blocked))
        .printed_task(&PrintableTask::new("e", 5, Blocked))
        .end();
}

#[test]
fn get_by_name_multiple_matches() {
    let mut fix = Fixture::default();
    fix.test("todo new bob frank bob");
    fix.test("todo get bob")
        .validate()
        .printed_task(&PrintableTask::new("bob", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("bob", 3, Incomplete).action(Select))
        .end();
}
