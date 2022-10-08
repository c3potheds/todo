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
fn new_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(New))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).action(New))
        .end();
}

#[test]
fn new_block_on_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -p 0")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_blocking_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -b 0")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p c -b a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("a", 4, Blocked))
        .end();
}

#[test]
fn new_chain_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(New))
        .end();
}

#[test]
fn new_one_blocking_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b --blocking 1")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_blocked_by_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b --blocked-by 1")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .end();
}

#[test]
fn new_one_blocking_one_short() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b -b 1")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_blocked_by_one_short() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b -p 1")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .end();
}

#[test]
fn new_blocking_multiple() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -b 1 2 3")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .end();
}

#[test]
fn new_blocking_and_blocked_by() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p 1 -b 2")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_in_between_blocking_pair() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p 1 -b 2")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_one_before_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --before b")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_three_before_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --before b")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("e", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("f", 4, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 5, Blocked))
        .end();
}

#[test]
fn new_one_before_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b c d -p a");
    fix.test("todo new e --before b c d")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("e", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .printed_task(&PrintableTask::new("d", 5, Blocked))
        .end();
}

#[test]
fn new_one_after_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --after b")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .end();
}

#[test]
fn new_three_after_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --after b")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("e", 4, Blocked).action(New))
        .printed_task(&PrintableTask::new("f", 5, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 6, Blocked))
        .end();
}

#[test]
fn new_one_after_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo new e --after a b c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
        .printed_task(&PrintableTask::new("e", 4, Blocked).action(New))
        .printed_task(&PrintableTask::new("d", 5, Blocked))
        .end();
}

#[test]
fn print_warning_on_cycle() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p b -b a")
        .modified(false)
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
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(New)
                .priority(Explicit(1)),
        )
        .end();
}

#[test]
fn new_task_with_priority_inserted_before_unprioritized_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c --priority 1")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete)
                .action(New)
                .priority(Explicit(1)),
        )
        .end();
}

#[test]
fn new_task_with_negative_priority_inserted_after_unprioritized_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c --priority -1")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("c", 3, Incomplete)
                .action(New)
                .priority(Explicit(-1)),
        )
        .end();
}

#[test]
fn new_task_with_priority_inserted_in_sorted_order() {
    let mut fix = Fixture::default();
    fix.test("todo new a --priority 1");
    fix.test("todo new b --priority 3");
    fix.test("todo new c --priority 2")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("c", 2, Incomplete)
                .action(New)
                .priority(Explicit(2)),
        )
        .end();
}

#[test]
fn new_with_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    let in_5_hours = ymdhms(2021, 04, 12, 20, 00, 00);
    fix.test("todo new a --due 5 hours")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .due_date(Explicit(in_5_hours))
                .action(New),
        )
        .end();
}

#[test]
fn new_with_invalid_due_date() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    fix.test("todo new a --due blah blah")
        .modified(false)
        .validate()
        .printed_error(&PrintableError::CannotParseDueDate {
            cannot_parse: "blah blah".to_string(),
        })
        .end();
}

#[test]
fn new_with_due_date_shows_affected_deps() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 04, 12, 15, 00, 00);
    let in_2_days = ymdhms(2021, 04, 14, 23, 59, 59);
    fix.test("todo new a b c --chain");
    fix.test("todo new d -p c --due 2 days")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .due_date(Implicit(in_2_days)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked).due_date(Implicit(in_2_days)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked).due_date(Implicit(in_2_days)),
        )
        .printed_task(
            &PrintableTask::new("d", 4, Blocked)
                .due_date(Explicit(in_2_days))
                .action(New),
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
    fix.test("todo new b -p a --due today --budget 5 hours")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .due_date(Implicit(before_7)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .due_date(Explicit(end_of_day))
                .action(New),
        )
        .end();
}

#[test]
fn new_with_too_long_time_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget 137 years")
        .modified(false)
        .validate()
        .printed_error(&PrintableError::DurationIsTooLong {
            duration: 4323391200,
            string_repr: "137 years".to_string(),
        })
        .end();
}

#[test]
fn new_with_unintelligible_time_budget() {
    let mut fix = Fixture::default();
    fix.test("todo new a --budget blah")
        .modified(false)
        .validate()
        .printed_error(&PrintableError::CannotParseDuration {
            cannot_parse: "blah".to_string(),
        })
        .end();
}

#[test]
fn new_invalid_snooze_date() {
    let mut fix = Fixture::default();
    fix.test("todo new a --snooze blah")
        .modified(false)
        .validate()
        .printed_error(&PrintableError::CannotParseDueDate {
            cannot_parse: "blah".to_string(),
        })
        .end();
}

#[test]
fn new_snooze_one_task() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 16, 00, 00);
    fix.test("todo new a --snooze 1 day")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00)),
        )
        .end();
}

#[test]
fn new_snooze_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.clock.now = ymdhms(2021, 05, 28, 16, 00, 00);
    fix.test("todo new a b c --snooze 2 days")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 30, 00, 00, 00)),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 30, 00, 00, 00)),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .action(New)
                .start_date(ymdhms(2021, 05, 30, 00, 00, 00)),
        )
        .end();
}

#[test]
fn new_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a --done")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete).action(New))
        .end();
}

#[test]
fn multiple_new_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --done")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", -2, Complete).action(New))
        .printed_task(&PrintableTask::new("b", -1, Complete).action(New))
        .printed_task(&PrintableTask::new("c", 0, Complete).action(New))
        .end();
}

#[test]
fn new_complete_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain --done")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", -2, Complete).action(New))
        .printed_task(&PrintableTask::new("b", -1, Complete).action(New))
        .printed_task(&PrintableTask::new("c", 0, Complete).action(New))
        .end();
}

#[test]
fn new_blocked_by_incomplete_task_but_tried_to_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b -p a --done")
        .modified(false)
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
        .modified(false)
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: BriefPrintableTask::new(2, Blocked),
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
        .modified(false)
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
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_transitively_block_completed_task() {
    let mut fix = Fixture::default();
    fix.test("todo new b c --chain --done");
    fix.test("todo new a -b b")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("c", 3, Blocked))
        .end();
}

#[test]
fn new_as_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).action(New).as_tag(),
        )
        .end();
}

#[test]
fn new_multiple_as_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --tag")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).action(New).as_tag(),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Incomplete).action(New).as_tag(),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Incomplete).action(New).as_tag(),
        )
        .end();
}

#[test]
fn new_blocking_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo new b -b a")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete).action(New).tag("a"),
        )
        .printed_task(&PrintableTask::new("a", 2, Blocked).as_tag())
        .end();
}

#[test]
fn new_tag_blocking_tag() {
    let mut fix = Fixture::default();
    fix.test("todo new a --tag");
    fix.test("todo new b -b a --tag")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete)
                .action(New)
                .tag("a")
                .as_tag(),
        )
        .printed_task(&PrintableTask::new("a", 2, Blocked).as_tag())
        .end();
}

#[test]
fn new_tag_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain --tag")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(New)
                .tag("c")
                .tag("b")
                .as_tag(),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(New)
                .tag("c")
                .as_tag(),
        )
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(New).as_tag())
        .end();
}

#[test]
fn trim_leading_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .end();
    fix.test("todo new ' a'")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 2, Incomplete).action(New))
        .end();
    fix.test("todo new '  a'")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Incomplete).action(New))
        .end();
}

#[test]
fn trim_trailing_whitespace_from_desc() {
    let mut fix = Fixture::default();
    fix.test("todo new a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .end();
    fix.test("todo new 'a '")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 2, Incomplete).action(New))
        .end();
    fix.test("todo new 'a  '")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Incomplete).action(New))
        .end();
}