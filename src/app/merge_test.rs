use app::testing::ymdhms;
use app::testing::Fixture;
use printing::Action::*;
use printing::BriefPrintableTask;
use printing::PrintableError;
use printing::PrintableTask;
use printing::Status::*;

#[test]
fn merge_two_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo merge a b --into ab")
        .validate()
        .printed_task(&PrintableTask::new("ab", 2, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_three_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo merge a b c --into abc")
        .validate()
        .printed_task(&PrintableTask::new("abc", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn merge_preserves_deps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo merge b c --into bc")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("bc", 2, Blocked).action(Select))
        .end();
}

#[test]
fn merge_preserves_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo merge a b --into ab")
        .validate()
        .printed_task(&PrintableTask::new("ab", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("c", 2, Blocked))
        .end();
}

#[test]
fn merge_preserves_deps_and_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e --chain");
    fix.test("todo merge b c d --into bcd")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("bcd", 2, Blocked).action(Select))
        .printed_task(&PrintableTask::new("e", 3, Blocked))
        .end();
}

#[test]
fn merged_task_has_min_due_date_of_sources() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 04, 25, 23, 20, 00);
    let in_10_min = ymdhms(2021, 04, 25, 23, 30, 00);
    fix.test("todo new a --due 15 min");
    fix.test("todo new b --due 10 min");
    fix.test("todo new c --due 20 min");
    fix.test("todo merge a b c --into abc")
        .validate()
        .printed_task(
            &PrintableTask::new("abc", 1, Incomplete)
                .action(Select)
                .due_date(in_10_min),
        )
        .end();
}

#[test]
fn merged_task_has_max_priority_of_sources() {
    let mut fix = Fixture::new();
    fix.test("todo new a --priority 1");
    fix.test("todo new b --priority -1");
    fix.test("todo new c --priority 2");
    fix.test("todo new d");
    fix.test("todo merge a b c d --into abcd")
        .validate()
        .printed_task(
            &PrintableTask::new("abcd", 1, Incomplete)
                .action(Select)
                .priority(2),
        )
        .end();
}

#[test]
fn merge_causes_cycle() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo merge a c --into ac")
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
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e --chain");
    fix.test("todo merge a e --into ae")
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
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e f --chain");
    fix.test("todo merge c d --into cd")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("cd", 3, Blocked).action(Select))
        .printed_task(&PrintableTask::new("e", 4, Blocked))
        .end();
}
