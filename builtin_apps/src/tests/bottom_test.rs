use todo_lookup_key::Key;
use todo_printing::PrintableWarning;
use todo_printing::Status::*;

use super::testing::task;
use super::testing::Fixture;
use super::testing::Mutated;

#[test]
fn bottom_empty() {
    let mut fix = Fixture::default();
    fix.test("todo bottom")
        .modified(Mutated::No)
        .validate()
        .end();
}

#[test]
fn bottom_all_tasks_uncategorized() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo bottom")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete))
        .printed_task(&task("b", 2, Incomplete))
        .printed_task(&task("c", 3, Incomplete))
        .end();
}

#[test]
fn bottom_all_tasks_categorized_the_same() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo bottom")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("b", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("c", 3, Incomplete).adeps_stats(0, 1))
        .end();
}

#[test]
fn bottom_multiple_categories() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f");
    fix.test("todo new g -p a b c");
    fix.test("todo new h -p d e f");
    fix.test("todo bottom")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("b", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("c", 3, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("d", 4, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("e", 5, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("f", 6, Incomplete).adeps_stats(0, 1))
        .end();
}

#[test]
fn bottom_deep_category() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo bottom")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("d", 2, Incomplete).adeps_stats(1, 2))
        .end();
}

#[test]
fn bottom_does_not_show_complete_tasks_by_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo bottom")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .printed_task(&task("c", 2, Incomplete))
        .end();
}

#[test]
fn bottom_show_complete_tasks_with_option() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo bottom -d")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 0, Complete))
        .printed_task(&task("b", 1, Incomplete))
        .printed_task(&task("c", 2, Incomplete))
        .end();
}

#[test]
fn bottom_show_only_bottom_level_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo check a b c d e f");
    fix.test("todo bottom -d")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", -5, Complete))
        .printed_task(&task("d", -4, Complete))
        .end();
}

#[test]
fn bottom_above_top_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo bottom c")
        .modified(Mutated::No)
        .validate()
        .end();
}

#[test]
fn bottom_above_blocking_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo bottom a")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn bottom_above_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo bottom b")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("c", 3, Blocked).deps_stats(1, 2))
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
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 3, Blocked).deps_stats(1, 1))
        .printed_task(&task("b", 4, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 5, Blocked).deps_stats(2, 2))
        .printed_task(&task("d", 6, Blocked).deps_stats(2, 2))
        .printed_task(&task("e", 7, Blocked).deps_stats(1, 1))
        .printed_task(&task("f", 8, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn bottom_exclude_adeps_with_indirect_connection() {
    let mut fix = Fixture::default();
    fix.test("todo new x");
    fix.test("todo new a b --chain -p x");
    fix.test("todo bottom x")
        .modified(Mutated::No)
        .validate()
        // b should be excluded because there's also an indirect connection to
        // it, through a. On the other hand, a is included because the only
        // path to it from the "bottom" task is direct.
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn bottom_with_typo() {
    let mut fix = Fixture::default();
    fix.test("todo new blah");
    fix.test("todo bottom bleh")
        .modified(Mutated::No)
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
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("b", 0, Complete))
        .end();
}
