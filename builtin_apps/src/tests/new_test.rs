#![allow(clippy::zero_prefixed_literal)]

use chrono::Duration;
use todo_printing::Action::*;
use todo_printing::BriefPrintableTask;
use todo_printing::Plicit::*;
use todo_printing::PrintableError;
use todo_printing::Status::*;
use todo_testing::ymdhms;

use super::testing::task;
use super::testing::Fixture;
use super::testing::Mutated;

#[test]
fn new_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New))
        .printed_task(&task("b", 2, Incomplete).action(New))
        .printed_task(&task("c", 3, Incomplete).action(New))
        .end();
}

#[test]
fn new_block_on_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -p 0")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete))
        .printed_task(&task("b", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_blocking_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -b 0")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn new_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p c -b a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("c", 2, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("d", 3, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("a", 4, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn new_chain_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New).adeps_stats(1, 2))
        .printed_task(&task("b", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).action(New).deps_stats(1, 2))
        .end();
}

#[test]
fn new_one_blocking_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b --blocking 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn new_blocked_by_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b --blocked-by 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("b", 2, Blocked).action(New).deps_stats(1, 1))
        .end();
}

#[test]
fn new_one_blocking_one_short() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b -b 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn new_blocked_by_one_short() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b -p 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("b", 2, Blocked).action(New).deps_stats(1, 1))
        .end();
}

#[test]
fn new_blocking_multiple() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -b 1 2 3")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("d", 1, Incomplete).action(New).adeps_stats(3, 3))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn new_blocking_and_blocked_by() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p 1 -b 2")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("c", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn new_in_between_blocking_pair() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p 1 -b 2")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(&task("c", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn new_one_before_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --before b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 3))
        .printed_task(&task("d", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn new_three_before_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --before b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(3, 5))
        .printed_task(&task("d", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("e", 3, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("f", 4, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("b", 5, Blocked).deps_stats(1, 4))
        .end();
}

#[test]
fn new_one_before_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b c d -p a");
    fix.test("todo new e --before b c d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 4))
        .printed_task(&task("e", 2, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).deps_stats(1, 2))
        .printed_task(&task("c", 4, Blocked).deps_stats(1, 2))
        .printed_task(&task("d", 5, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn new_one_after_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --after b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("d", 3, Blocked).action(New).deps_stats(1, 2))
        .printed_task(&task("c", 4, Blocked).deps_stats(1, 3))
        .end();
}

#[test]
fn new_three_after_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --after b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("d", 3, Blocked).action(New).deps_stats(1, 2))
        .printed_task(&task("e", 4, Blocked).action(New).deps_stats(1, 2))
        .printed_task(&task("f", 5, Blocked).action(New).deps_stats(1, 2))
        .printed_task(&task("c", 6, Blocked).deps_stats(1, 5))
        .end();
}

#[test]
fn new_one_after_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo new e --after a b c")
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 2))
        .printed_task(&task("b", 2, Incomplete).adeps_stats(0, 2))
        .printed_task(&task("c", 3, Incomplete).adeps_stats(0, 2))
        .printed_task(&task("e", 4, Blocked).action(New).deps_stats(3, 3))
        .printed_task(&task("d", 5, Blocked).deps_stats(3, 4))
        .end();
}

#[test]
fn new_one_by_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(2, 3))
        .printed_task(&task("d", 3, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).deps_stats(1, 3))
        .end();
}

#[test]
fn new_three_by_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(4, 5))
        .printed_task(&task("d", 3, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("e", 4, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("f", 5, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("c", 6, Blocked).deps_stats(1, 5))
        .end();
}

#[test]
fn new_one_by_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo new e --by a b c")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("e", 4, Incomplete).action(New).adeps_stats(0, 1))
        .printed_task(&task("d", 5, Blocked).deps_stats(4, 4))
        .end();
}

#[test]
fn new_one_by_one_with_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --by b --chain")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(2, 3))
        .printed_task(&task("d", 3, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).deps_stats(1, 3))
        .end();
}

#[test]
fn new_three_by_one_with_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --by b --chain")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(2, 5))
        .printed_task(&task("d", 3, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("e", 4, Blocked).action(New).deps_stats(1, 2))
        .printed_task(&task("f", 5, Blocked).action(New).deps_stats(1, 3))
        .printed_task(&task("c", 6, Blocked).deps_stats(1, 5))
        .end();
}

#[test]
fn print_warning_on_cycle() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p b -b a")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(3, Blocked),
        })
        .end();
}

#[test]
fn new_with_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a --priority 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).action(New).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn new_task_with_priority_inserted_before_unprioritized_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c --priority 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("c", 1, Incomplete).action(New).priority(Explicit(1)),
        )
        .end();
}

#[test]
fn new_task_with_negative_priority_inserted_after_unprioritized_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c --priority -1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("c", 3, Incomplete).action(New).priority(Explicit(-1)),
        )
        .end();
}

#[test]
fn new_task_with_priority_inserted_in_sorted_order() {
    let mut fix = Fixture::default();
    fix.test("todo new a --priority 1");
    fix.test("todo new b --priority 3");
    fix.test("todo new c --priority 2")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("c", 2, Incomplete).action(New).priority(Explicit(2)),
        )
        .end();
}

#[test]
fn new_with_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    let in_5_hours = ymdhms(2021, 04, 12, 20, 00, 00);
    fix.test("todo new a --due '5 hours'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Explicit(in_5_hours))
                .action(New),
        )
        .end();
}

#[test]
fn new_with_due_date_shows_affected_deps() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    let in_2_days = ymdhms(2021, 04, 14, 23, 59, 59);
    fix.test("todo new a b c --chain");
    fix.test("todo new d -p c --due '2 days'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Implicit(in_2_days))
                .adeps_stats(1, 3),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .due_date(Implicit(in_2_days))
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .due_date(Implicit(in_2_days))
                .deps_stats(1, 2),
        )
        .printed_task(
            &task("d", 4, Blocked)
                .due_date(Explicit(in_2_days))
                .action(New)
                .deps_stats(1, 3),
        )
        .end();
}

#[test]
fn new_with_budget_shows_affected_deps() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 29, 09, 30, 00);
    let before_7 = ymdhms(2021, 04, 29, 18, 59, 59);
    let end_of_day = ymdhms(2021, 04, 29, 23, 59, 59);
    fix.test("todo new a");
    fix.test("todo new b -p a --due today --budget '5 hours'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .due_date(Implicit(before_7))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .due_date(Explicit(end_of_day))
                .budget(Duration::hours(5))
                .action(New)
                .deps_stats(1, 1),
        )
        .end();
}

#[test]
#[ignore = "
    Time limits should not be necessary. Replace budget type in model with a 
    chrono::Duration.
"]
fn new_with_too_long_time_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 137 years")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::DurationIsTooLong {
            duration: 4323391200,
            string_repr: "137 years".to_string(),
        })
        .end();
}

#[test]
fn new_snooze_one_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 16, 00, 00);
    fix.test("todo new a --snooze '1 day'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00)),
        )
        .end();
}

#[test]
fn new_snooze_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 16, 00, 00);
    fix.test("todo new a b c --snooze '2 days'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 30, 00, 00, 00)),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 30, 00, 00, 00)),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 30, 00, 00, 00)),
        )
        .end();
}

#[test]
fn new_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a --done")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).action(New))
        .end();
}

#[test]
fn multiple_new_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --done")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -2, Complete).action(New))
        .printed_task(&task("b", -1, Complete).action(New))
        .printed_task(&task("c", 0, Complete).action(New))
        .end();
}

#[test]
fn new_complete_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain --done")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -2, Complete).action(New))
        .printed_task(&task("b", -1, Complete).action(New))
        .printed_task(&task("c", 0, Complete).action(New))
        .end();
}

#[test]
fn new_blocked_by_incomplete_task_but_tried_to_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b -p a --done")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: BriefPrintableTask::new(2, Blocked),
            blocked_by: vec![BriefPrintableTask::new(1, Incomplete)],
        })
        .end();
}

#[test]
fn new_blocked_by_incomplete_task_and_blocks_other_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a c");
    fix.test("todo new b -p a -b c --done")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: BriefPrintableTask::new(3, Blocked),
            blocked_by: vec![BriefPrintableTask::new(1, Incomplete)],
        })
        .end();
}

#[test]
fn new_blocked_by_incomplete_task_and_blocks_other_task_with_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a1 a2 a3");
    fix.test("todo new b1 b2 b3 -p a1 a2 a3");
    fix.test("todo new c1 c2 c3 -p b1 b2 b3 --done")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: BriefPrintableTask::new(7, Blocked),
            blocked_by: vec![
                BriefPrintableTask::new(4, Blocked),
                BriefPrintableTask::new(5, Blocked),
                BriefPrintableTask::new(6, Blocked),
            ],
        })
        .end();
}

#[test]
fn new_block_completed_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new b -b a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn new_transitively_block_completed_task() {
    let mut fix = Fixture::default();
    fix.test("todo new b c --chain --done");
    fix.test("todo new a -b b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New).adeps_stats(1, 2))
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).deps_stats(1, 2))
        .end();
}

#[test]
fn new_as_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New).as_tag())
        .end();
}

#[test]
fn new_multiple_as_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New).as_tag())
        .printed_task(&task("b", 2, Incomplete).action(New).as_tag())
        .printed_task(&task("c", 3, Incomplete).action(New).as_tag())
        .end();
}

#[test]
fn new_blocking_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo new b -b a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("b", 1, Incomplete)
                .action(New)
                .tag("a")
                .adeps_stats(1, 1),
        )
        .printed_task(&task("a", 2, Blocked).as_tag().deps_stats(1, 1))
        .end();
}

#[test]
fn new_tag_blocking_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo new b -b a --tag")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("b", 1, Incomplete)
                .action(New)
                .tag("a")
                .as_tag()
                .adeps_stats(1, 1),
        )
        .printed_task(&task("a", 2, Blocked).as_tag().deps_stats(1, 1))
        .end();
}

#[test]
fn new_tag_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain --tag")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .action(New)
                .tag("c")
                .tag("b")
                .as_tag()
                .adeps_stats(1, 2),
        )
        .printed_task(
            &task("b", 2, Blocked)
                .action(New)
                .tag("c")
                .as_tag()
                .deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked).action(New).as_tag().deps_stats(1, 2),
        )
        .end();
}

#[test]
fn trim_leading_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New))
        .end();
    fix.test("todo new ' a'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 2, Incomplete).action(New))
        .end();
    fix.test("todo new '  a'")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 3, Incomplete).action(New))
        .end();
}

#[test]
fn trim_trailing_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(New))
        .end();
    fix.test("todo new 'a '")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 2, Incomplete).action(New))
        .end();
    fix.test("todo new 'a  '")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 3, Incomplete).action(New))
        .end();
}

#[test]
fn do_not_change_position_of_adep_if_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new y z");
    fix.test("todo new x -b y -d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("x", 0, Complete).action(New))
        .printed_task(&task("y", 1, Incomplete))
        .end();
}

#[test]
fn block_on_task_by_name_that_matches_only_complete_tasks_blocks_that_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new b -b a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn block_on_task_by_name_that_matches_incomplete_and_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new a");
    fix.test("todo new b --blocking a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn block_on_task_by_name_that_matches_multiple_incomplete_tasks_blocks_on_all()
{
    let mut fix = Fixture::default();
    fix.test("todo new a a");
    fix.test("todo new b --blocking a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(2, 2))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("a", 3, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn blocked_by_task_by_name_that_matches_only_complete_tasks_blocked_by_that_task(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new b -p a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete))
        .printed_task(&task("b", 1, Incomplete).action(New))
        .end();
}

#[test]
fn blocked_by_task_by_name_that_matches_incomplete_and_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new a");
    fix.test("todo new b -p a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("b", 2, Blocked).action(New).deps_stats(1, 1))
        .end();
}

#[test]
fn blocked_by_task_by_name_that_matches_multiple_incomplete_tasks_blocked_by_all(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a a");
    fix.test("todo new b -p a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("a", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("b", 3, Blocked).deps_stats(2, 2).action(New))
        .end();
}

#[test]
fn before_task_by_name_that_matches_only_complete_tasks_inserted_before_that_task(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new b --before a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn before_task_by_name_that_matches_incomplete_and_complete_tasks_inserted_before_all_tasks(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new a");
    fix.test("todo new b --before a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn before_task_by_name_that_matches_multiple_incomplete_tasks_inserted_before_all_tasks(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a a");
    fix.test("todo new b --before a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).action(New).adeps_stats(2, 2))
        .printed_task(&task("a", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("a", 3, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn after_task_by_name_that_matches_only_complete_tasks_inserted_after_that_task(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new b --after a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete))
        .printed_task(&task("b", 1, Incomplete).action(New))
        .end();
}

#[test]
fn after_task_by_name_that_matches_incomplete_and_complete_tasks_inserted_after_incomplete_tasks(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a --done");
    fix.test("todo new a");
    fix.test("todo new b --after a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("b", 2, Blocked).action(New).deps_stats(1, 1))
        .end();
}

#[test]
fn after_task_by_name_that_matches_multiple_incomplete_tasks_inserted_after_all_tasks(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a a");
    fix.test("todo new b --after a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("a", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("b", 3, Blocked).action(New).deps_stats(2, 2))
        .end();
}

#[test]
fn by_task_by_name_that_matches_only_complete_task_inserted_by_that_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b");
    fix.test("todo new d --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -1, Complete))
        .printed_task(&task("d", 1, Incomplete).action(New).adeps_stats(1, 1))
        .printed_task(&task("c", 2, Blocked).deps_stats(1, 3))
        .end();
}

#[test]
fn by_task_by_name_that_matches_incomplete_and_complete_tasks_inserted_by_incomplete_tasks(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a1 b c1 --chain -d");
    fix.test("todo restore c1");
    fix.test("todo new a2 b c2 --chain");
    fix.test("todo new d --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a2", 2, Incomplete).adeps_stats(2, 3))
        .printed_task(&task("d", 4, Blocked).action(New).deps_stats(1, 1))
        .printed_task(&task("c2", 5, Blocked).deps_stats(1, 3))
        .end()
}

#[test]
fn by_task_by_name_that_matches_multiple_incomplete_tasks_inserted_by_all_tasks(
) {
    let mut fix = Fixture::default();
    fix.test("todo new a1 b c1 --chain");
    fix.test("todo new a2 b c2 --chain");
    fix.test("todo new d --by b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a1", 1, Incomplete).adeps_stats(1, 4))
        .printed_task(&task("a2", 2, Incomplete).adeps_stats(1, 4))
        .printed_task(&task("d", 5, Blocked).action(New).deps_stats(2, 2))
        .printed_task(&task("c1", 6, Blocked).deps_stats(2, 4))
        .printed_task(&task("c2", 7, Blocked).deps_stats(2, 4))
        .end();
}
