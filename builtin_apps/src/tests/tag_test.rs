use todo_printing::Action::*;
use todo_printing::Status::*;

use super::testing::task;
use super::testing::Fixture;
use super::testing::Mutated;

#[test]
fn tag_show_no_tags() {
    let mut fix = Fixture::default();
    fix.test("todo tag").modified(Mutated::No).validate().end();
}

#[test]
fn tag_show_all_tags() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo new d e f");
    fix.test("todo tag")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 1, Incomplete).as_tag())
        .printed_task(&task("b", 2, Incomplete).as_tag())
        .printed_task(&task("c", 3, Incomplete).as_tag())
        .end();
}

#[test]
fn tag_show_blocked_tags() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag --chain");
    fix.test("todo tag")
        .modified(Mutated::No)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .tag("c")
                .tag("b")
                .as_tag()
                .adeps_stats(1, 2),
        )
        .printed_task(&task("b", 2, Blocked).tag("c").as_tag().deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).as_tag().deps_stats(1, 2))
        .end();
}

#[test]
fn tag_does_not_show_complete_tags_by_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo check a");
    fix.test("todo tag")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("b", 1, Incomplete).as_tag())
        .printed_task(&task("c", 2, Incomplete).as_tag())
        .end();
}

#[test]
fn tag_show_complete_tags() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo check a");
    fix.test("todo tag --include-done")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 0, Complete).as_tag())
        .printed_task(&task("b", 1, Incomplete).as_tag())
        .printed_task(&task("c", 2, Incomplete).as_tag())
        .end();
}

#[test]
fn tag_mark_single() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo tag a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select).as_tag())
        .end();
}

#[test]
fn tag_mark_multiple() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo tag a b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select).as_tag())
        .printed_task(&task("b", 2, Incomplete).action(Select).as_tag())
        .end();
}

#[test]
fn tag_mark_already_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo tag a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select).as_tag())
        .end();
    fix.test("todo tag a").validate().end();
}

#[test]
fn tag_prints_affected_deps_when_marking() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo tag c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).tag("c").adeps_stats(1, 2))
        .printed_task(&task("b", 2, Blocked).tag("c").deps_stats(1, 1))
        .printed_task(
            &task("c", 3, Blocked)
                .action(Select)
                .as_tag()
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn tag_unmark_single() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo tag -u a")
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn tag_unmark_multiple() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo tag -u a b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select))
        .printed_task(&task("b", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn tag_unmark_already_unmarked() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo tag -u a")
        .modified(Mutated::No)
        .validate()
        .end();
}

#[test]
fn tag_mark_and_unmark() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b --tag");
    fix.test("todo tag a -u b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select).as_tag())
        .printed_task(&task("b", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn tag_does_not_show_complete_deps_by_default_when_marking() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a");
    fix.test("todo tag c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).tag("c").adeps_stats(1, 1))
        .printed_task(
            &task("c", 2, Blocked)
                .action(Select)
                .as_tag()
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn tag_show_complete_deps_when_marking() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a");
    fix.test("todo tag c --include-done")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).tag("c"))
        .printed_task(&task("b", 1, Incomplete).tag("c").adeps_stats(1, 1))
        .printed_task(
            &task("c", 2, Blocked)
                .action(Select)
                .as_tag()
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn tag_does_not_show_complete_deps_by_default_when_unmarking() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo tag c");
    fix.test("todo check a");
    fix.test("todo tag -u c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("c", 2, Blocked).action(Select).deps_stats(1, 2))
        .end();
}

#[test]
fn tag_show_complete_deps_when_unmarking() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo tag c");
    fix.test("todo check a");
    fix.test("todo tag -u c -d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete))
        .printed_task(&task("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("c", 2, Blocked).action(Select).deps_stats(1, 2))
        .end();
}
