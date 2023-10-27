#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::task,
    super::testing::Fixture,
    todo_app::Mutated,
    todo_printing::{
        Action::*, BriefPrintableTask, Plicit::*, PrintableError, Status::*,
    },
    todo_testing::ymdhms,
};

#[test]
fn merge_two_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b --into ab")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("ab", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_three_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into abc")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_preserves_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo merge b c --into bc")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("bc", 2, Blocked).action(Select).deps_stats(1, 1))
        .end();
}

#[test]
fn merge_preserves_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo merge a b --into ab")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("ab", 1, Incomplete).action(Select).adeps_stats(1, 1),
        )
        .printed_task(&task("c", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn merge_preserves_deps_and_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo merge b c d --into bcd")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("bcd", 2, Blocked).action(Select).deps_stats(1, 1))
        .printed_task(&task("e", 3, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn merged_task_has_min_due_date_of_sources() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 25, 23, 20, 00);
    let in_10_min = ymdhms(2021, 04, 25, 23, 30, 00);
    fix.test("todo new a --due 15 min");
    fix.test("todo new b --due 10 min");
    fix.test("todo new c --due 20 min");
    fix.test("todo merge a b c --into abc")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("abc", 1, Incomplete)
                .action(Select)
                .due_date(Explicit(in_10_min)),
        )
        .end();
}

#[test]
fn merged_task_has_max_priority_of_sources() {
    let mut fix = Fixture::default();
    fix.test("todo new a --priority 1");
    fix.test("todo new b --priority -1");
    fix.test("todo new c --priority 2");
    fix.test("todo new d");
    fix.test("todo merge a b c d --into abcd")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("abcd", 1, Incomplete)
                .action(Select)
                .priority(Explicit(2)),
        )
        .end();
}

#[test]
fn merge_causes_cycle() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo merge a c --into ac")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotMerge {
            cycle_through: vec![BriefPrintableTask::new(2, Blocked)],
            adeps_of: vec![BriefPrintableTask::new(1, Incomplete)],
            deps_of: vec![BriefPrintableTask::new(3, Blocked)],
        })
        .end();
}

#[test]
fn merge_causes_cycle_indirect() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo merge a e --into ae")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotMerge {
            cycle_through: vec![
                BriefPrintableTask::new(2, Blocked),
                BriefPrintableTask::new(3, Blocked),
                BriefPrintableTask::new(4, Blocked),
            ],
            adeps_of: vec![BriefPrintableTask::new(1, Incomplete)],
            deps_of: vec![BriefPrintableTask::new(5, Blocked)],
        })
        .end();
}

#[test]
fn merge_inside_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e f --chain");
    fix.test("todo merge c d --into cd")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("cd", 3, Blocked).action(Select).deps_stats(1, 2))
        .printed_task(&task("e", 4, Blocked).deps_stats(1, 3))
        .end();
}

#[test]
fn merge_task_with_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 18, 00, 00);
    fix.test("todo new a b");
    fix.test("todo snooze b --until 1 day");
    fix.test("todo merge a b --into ab")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("ab", 1, Blocked)
                .action(Select)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00)),
        )
        .end();
}

#[test]
fn merge_snoozed_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 16, 00, 00);
    fix.test("todo new a b c");
    fix.test("todo snooze a --until 1 hour");
    fix.test("todo snooze b --until 2 hours");
    fix.test("todo snooze c --until 3 hours");
    fix.test("todo merge a b c --into abc")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("abc", 1, Blocked)
                .action(Select)
                .start_date(ymdhms(2021, 05, 28, 19, 00, 00)),
        )
        .end();
}

#[test]
fn merge_tags_default() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo merge a b c --into abc")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select).as_tag())
        .end();
}

#[test]
fn merge_tags_into_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo merge a b c --into abc --tag true")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select).as_tag())
        .end();
}

#[test]
fn merge_tasks_into_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into abc --tag true")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select).as_tag())
        .end();
}

#[test]
fn merge_tags_into_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo merge a b c --into abc --tag false")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn show_tags_for_merged_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo block c --on a b");
    fix.test("todo merge a b --into ab")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("ab", 1, Incomplete)
                .action(Select)
                .as_tag()
                .tag("c")
                .adeps_stats(1, 1),
        )
        .printed_task(&task("c", 2, Blocked).as_tag().deps_stats(1, 1))
        .end();
}

#[test]
fn trim_leading_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into '  abc'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn trim_trailing_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into 'abc  '")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn trim_leading_and_trailing_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into '  abc  '")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("abc", 1, Incomplete).action(Select))
        .end();
}
