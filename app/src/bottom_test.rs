use {
    super::testing::Fixture,
    lookup_key::Key,
    printing::{PrintableTask, PrintableWarning, Status::*},
};

#[test]
fn bottom_empty() {
    let mut fix = Fixture::default();
    fix.test("todo bottom").modified(false).validate().end();
}

#[test]
fn bottom_all_tasks_uncategorized() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo bottom")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .end();
}

#[test]
fn bottom_all_tasks_categorized_the_same() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo bottom")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .end();
}

#[test]
fn bottom_multiple_categories() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f");
    fix.test("todo new g -p a b c");
    fix.test("todo new h -p d e f");
    fix.test("todo bottom")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .printed_task(&PrintableTask::new("d", 4, Incomplete))
        .printed_task(&PrintableTask::new("e", 5, Incomplete))
        .printed_task(&PrintableTask::new("f", 6, Incomplete))
        .end();
}

#[test]
fn bottom_deep_category() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo bottom")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Incomplete))
        .end();
}

#[test]
fn bottom_does_not_show_complete_tasks_by_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo bottom")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .end();
}

#[test]
fn bottom_show_complete_tasks_with_option() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo bottom -d")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .end();
}

#[test]
fn bottom_show_only_bottom_level_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo check a b c d e f");
    fix.test("todo bottom -d")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", -5, Complete))
        .printed_task(&PrintableTask::new("d", -4, Complete))
        .end();
}

#[test]
fn bottom_above_top_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo bottom c").modified(false).validate().end();
}

#[test]
fn bottom_above_blocking_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo bottom a")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .end();
}

#[test]
fn bottom_above_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo bottom b")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("c", 3, Blocked))
        .end();
}

#[test]
fn bottom_above_two_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new x y");
    fix.test("todo new a b -p x");
    fix.test("todo new c d -p x y");
    fix.test("todo new e f -p y");
    fix.test("todo bottom x y")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Blocked))
        .printed_task(&PrintableTask::new("b", 4, Blocked))
        .printed_task(&PrintableTask::new("c", 5, Blocked))
        .printed_task(&PrintableTask::new("d", 6, Blocked))
        .printed_task(&PrintableTask::new("e", 7, Blocked))
        .printed_task(&PrintableTask::new("f", 8, Blocked))
        .end();
}

#[test]
fn bottom_exclude_adeps_with_indirect_connection() {
    let mut fix = Fixture::default();
    fix.test("todo new x");
    fix.test("todo new a b --chain -p x");
    fix.test("todo bottom x")
        .modified(false)
        .validate()
        // b should be excluded because there's also an indirect connection to
        // it, through a. On the other hand, a is included because the only
        // path to it from the "bottom" task is direct.
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn bottom_with_typo() {
    let mut fix = Fixture::default();
    fix.test("todo new blah");
    fix.test("todo bottom bleh")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByName("bleh".to_string()),
        })
        .end();
}

#[test]
fn bottom_implicit_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a b");
    fix.test("todo bottom a")
        .modified(false)
        .validate()
        .printed_task(&PrintableTask::new("b", 0, Complete))
        .end();
}
