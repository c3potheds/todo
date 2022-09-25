use {
    super::testing::Fixture,
    printing::{PrintableTask, Status::*},
};

#[test]
fn find_with_exact_match() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo find b")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .end();
}

#[test]
fn find_with_substring_match() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca");
    fix.test("todo find b")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("aba", 2, Incomplete))
        .end();
}

#[test]
fn find_with_multiple_matches() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca");
    fix.test("todo find a")
        .modified(false)
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
    fix.test("todo find b").modified(false).validate().end();
}

#[test]
fn find_includes_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca");
    fix.test("todo check 2");
    fix.test("todo find b -d")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("aba", 0, Complete))
        .end();
}

#[test]
fn find_includes_blocked_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new aaa aba aca --chain");
    fix.test("todo find b")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("aba", 2, Blocked))
        .end();
}

#[test]
fn find_case_insensitive() {
    let mut fix = Fixture::default();
    fix.test("todo new AAA aaa");
    fix.test("todo find aa")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("AAA", 1, Incomplete))
        .printed_task(&PrintableTask::new("aaa", 2, Incomplete))
        .end();
}

#[test]
fn find_includes_matches_with_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f");
    fix.test("todo new g -p a b c --tag");
    // Because a, b, and c are tagged with 'g', they show up in 'find' results.
    fix.test("todo find g -t")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 4, Incomplete).tag("g"))
        .printed_task(&PrintableTask::new("b", 5, Incomplete).tag("g"))
        .printed_task(&PrintableTask::new("c", 6, Incomplete).tag("g"))
        .printed_task(&PrintableTask::new("g", 7, Blocked).as_tag())
        .end();
}

#[test]
fn find_includes_matches_with_tag_excludes_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f -d");
    fix.test("todo new g -p a b c --tag");
    // Although a, b, and c are tagged with 'g', they are complete, so they do
    // not show up in 'find' results by default.
    fix.test("todo find g -t")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("g", 1, Incomplete).as_tag())
        .end();
}

#[test]
fn find_includes_matches_with_tag_include_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f -d");
    fix.test("todo new g -p a b c --tag");
    // Since the '-d' flag is used, a, b, and c show up even though they are
    // complete.
    fix.test("todo find g -d -t")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", -5, Complete).tag("g"))
        .printed_task(&PrintableTask::new("b", -4, Complete).tag("g"))
        .printed_task(&PrintableTask::new("c", -3, Complete).tag("g"))
        .printed_task(&PrintableTask::new("g", 1, Incomplete).as_tag())
        .end();
}
