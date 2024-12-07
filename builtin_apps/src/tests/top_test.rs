use todo_lookup_key::Key;
use todo_printing::PrintableWarning;
use todo_printing::Status::*;

use super::testing::task;
use super::testing::Fixture;
use super::testing::Mutated;

#[test]
fn top_empty() {
    let mut fix = Fixture::default();
    fix.test("todo top").modified(Mutated::No).validate().end();
}

#[test]
fn top_all_tasks_uncategorized() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo top")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete))
        .printed_task(&task("b", 2, Incomplete))
        .printed_task(&task("c", 3, Incomplete))
        .end();
}

#[test]
fn top_all_tasks_categorized_the_same() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo top")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("d", 4, Blocked).deps_stats(3, 3))
        .end();
}

#[test]
fn top_multiple_categories() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f");
    fix.test("todo new g -p a b c");
    fix.test("todo new h -p d e f");
    fix.test("todo top")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("g", 7, Blocked).deps_stats(3, 3))
        .printed_task(&task("h", 8, Blocked).deps_stats(3, 3))
        .end();
}

#[test]
fn top_deep_category() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo top")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("c", 5, Blocked).deps_stats(1, 2))
        .printed_task(&task("f", 6, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn top_does_not_show_complete_tasks_by_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo top")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .printed_task(&task("c", 2, Incomplete))
        .end();
}

#[test]
fn top_show_complete_tasks_with_option() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo top -d")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 0, Complete))
        .printed_task(&task("b", 1, Incomplete))
        .printed_task(&task("c", 2, Incomplete))
        .end();
}

#[test]
fn top_show_only_top_level_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo check a b c d e f");
    fix.test("todo top -d")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("c", -1, Complete))
        .printed_task(&task("f", 0, Complete))
        .end();
}

#[test]
fn top_underneath_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo top c")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("b", 2, Incomplete).adeps_stats(0, 1))
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
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("b", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("c", 3, Incomplete).adeps_stats(0, 2))
        .printed_task(&task("d", 4, Incomplete).adeps_stats(0, 2))
        .printed_task(&task("e", 5, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("f", 6, Incomplete).adeps_stats(0, 1))
        .end();
}

#[test]
fn top_exclude_deps_with_indirect_connection_to_category() {
    let mut fix = Fixture::default();
    fix.test("todo new x");
    fix.test("todo new a b --chain -b x");
    fix.test("todo top x")
        .modified(Mutated::No)
        .validate()
        // a should be excluded because there's also an indirect connection to
        // the top, through b. On the other hand, b is included because the only
        // path to the top is direct.
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn top_with_typo() {
    let mut fix = Fixture::default();
    fix.test("todo new blah");
    fix.test("todo top bleh")
        .modified(Mutated::No)
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
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", -1, Complete))
        .end();
}
