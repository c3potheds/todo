use {
    super::testing::Fixture,
    lookup_key::Key,
    printing::{PrintableTask, PrintableWarning, Status::*},
};

#[test]
fn top_empty() {
    let mut fix = Fixture::default();
    fix.test("todo top").validate().end();
}

#[test]
fn top_all_tasks_uncategorized() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo top")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .end();
}

#[test]
fn top_all_tasks_categorized_the_same() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo top")
        .validate()
        .printed_task(&PrintableTask::new("d", 4, Blocked))
        .end();
}

#[test]
fn top_multiple_categories() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f");
    fix.test("todo new g -p a b c");
    fix.test("todo new h -p d e f");
    fix.test("todo top")
        .validate()
        .printed_task(&PrintableTask::new("g", 7, Blocked))
        .printed_task(&PrintableTask::new("h", 8, Blocked))
        .end();
}

#[test]
fn top_deep_category() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo top")
        .validate()
        .printed_task(&PrintableTask::new("c", 5, Blocked))
        .printed_task(&PrintableTask::new("f", 6, Blocked))
        .end();
}

#[test]
fn top_does_not_show_complete_tasks_by_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo top")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .end();
}

#[test]
fn top_show_complete_tasks_with_option() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo top -d")
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .end();
}

#[test]
fn top_show_only_top_level_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo check a b c d e f");
    fix.test("todo top -d")
        .validate()
        .printed_task(&PrintableTask::new("c", -1, Complete))
        .printed_task(&PrintableTask::new("f", 0, Complete))
        .end();
}

#[test]
fn top_underneath_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo top c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .end();
}

#[test]
fn top_union_of_categories() {
    let mut fix = Fixture::default();
    fix.test("todo new x y");
    fix.test("todo new a b -b x");
    fix.test("todo new c d -b x y");
    fix.test("todo new e f -b y");
    fix.test("todo top x y")
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
fn top_exclude_deps_with_indirect_connection_to_category() {
    let mut fix = Fixture::default();
    fix.test("todo new x");
    fix.test("todo new a b --chain -b x");
    fix.test("todo top x")
        .validate()
        // a should be excluded because there's also an indirect connection to
        // the top, through b. On the other hand, b is included because the only
        // path to the top is direct.
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .end();
}

#[test]
fn top_with_typo() {
    let mut fix = Fixture::default();
    fix.test("todo new blah");
    fix.test("todo top bleh")
        .validate()
        .printed_warning(&PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByName("bleh".to_string()),
        })
        .end();
}

#[test]
fn top_implicit_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a b");
    fix.test("todo top b")
        .validate()
        .printed_task(&PrintableTask::new("a", -1, Complete))
        .end();
}
