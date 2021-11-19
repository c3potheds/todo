use app::testing::Fixture;
use printing::PrintableTask;
use printing::Status::*;

#[test]
fn find_with_exact_match() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo find b")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .end();
}

#[test]
fn find_with_substring_match() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca");
    fix.test("todo find b")
        .validate()
        .printed_task(&PrintableTask::new("aba", 2, Incomplete))
        .end();
}

#[test]
fn find_with_multiple_matches() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca");
    fix.test("todo find a")
        .validate()
        .printed_task(&PrintableTask::new("aaa", 1, Incomplete))
        .printed_task(&PrintableTask::new("aba", 2, Incomplete))
        .printed_task(&PrintableTask::new("aca", 3, Incomplete))
        .end();
}

#[test]
fn find_excludes_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca");
    fix.test("todo check 2");
    fix.test("todo find b").validate().end();
}

#[test]
fn find_includes_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca");
    fix.test("todo check 2");
    fix.test("todo find b -d")
        .validate()
        .printed_task(&PrintableTask::new("aba", 0, Complete))
        .end();
}

#[test]
fn find_includes_blocked_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca --chain");
    fix.test("todo find b")
        .validate()
        .printed_task(&PrintableTask::new("aba", 2, Blocked))
        .end();
}

#[test]
fn find_case_insensitive() {
    let mut fix = Fixture::default();
    fix.test("todo new AAA aaa");
    fix.test("todo find aa")
        .validate()
        .printed_task(&PrintableTask::new("AAA", 1, Incomplete))
        .printed_task(&PrintableTask::new("aaa", 2, Incomplete))
        .end();
}
