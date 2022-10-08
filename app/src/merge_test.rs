#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::Fixture,
    printing::{
        Action::*, BriefPrintableTask, Plicit::*, PrintableError,
        PrintableTask, Status::*,
    },
    testing::ymdhms,
};

#[test]
fn merge_two_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b --into ab")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("ab", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_three_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into abc")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_preserves_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo merge b c --into bc")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("bc", 2, Blocked).action(Select))
        .end();
}

#[test]
fn merge_preserves_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo merge a b --into ab")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("ab", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("c", 2, Blocked))
        .end();
}

#[test]
fn merge_preserves_deps_and_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo merge b c d --into bcd")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("bcd", 2, Blocked).action(Select))
        .printed_task(&PrintableTask::new("e", 3, Blocked))
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
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("abc", 1, Incomplete)
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
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("abcd", 1, Incomplete)
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
        .modified(false)
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
        .modified(false)
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
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("cd", 3, Blocked).action(Select))
        .printed_task(&PrintableTask::new("e", 4, Blocked))
        .end();
}

#[test]
fn merge_task_with_snoozed_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 18, 00, 00);
    fix.test("todo new a b");
    fix.test("todo snooze b --until 1 day");
    fix.test("todo merge a b --into ab")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("ab", 1, Blocked)
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
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("abc", 1, Blocked)
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
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("abc", 1, Incomplete)
                .action(Select)
                .as_tag(),
        )
        .end();
}

#[test]
fn merge_tags_into_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo merge a b c --into abc --tag true")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("abc", 1, Incomplete)
                .action(Select)
                .as_tag(),
        )
        .end();
}

#[test]
fn merge_tasks_into_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into abc --tag true")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("abc", 1, Incomplete)
                .action(Select)
                .as_tag(),
        )
        .end();
}

#[test]
fn merge_tags_into_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo merge a b c --into abc --tag false")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn show_tags_for_merged_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag");
    fix.test("todo block c --on a b");
    fix.test("todo merge a b --into ab")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("ab", 1, Incomplete)
                .action(Select)
                .as_tag()
                .tag("c"),
        )
        .printed_task(&PrintableTask::new("c", 2, Blocked).as_tag())
        .end();
}

#[test]
fn trim_leading_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into '  abc'")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn trim_trailing_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into 'abc  '")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn trim_leading_and_trailing_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into '  abc  '")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("abc", 1, Incomplete).action(Select))
        .end();
}