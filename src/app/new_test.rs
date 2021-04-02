use app::testing::Fixture;
use model::TaskStatus::*;
use printing::Action::*;
use printing::PrintableError;
use printing::PrintableTask;

#[test]
fn new_one_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_multiple_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(New))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).action(New))
        .end();
}

#[test]
fn new_block_on_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -p 0")
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .end();
}

#[test]
fn new_blocking_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo new b -b 0")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_by_name() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo new d -p c -b a")
        .validate()
        .printed_task(&PrintableTask::new("c", 2, Incomplete))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("a", 4, Blocked))
        .end();
}

#[test]
fn new_chain_three() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(New))
        .end();
}

#[test]
fn new_one_blocking_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b --blocking 1")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_blocked_by_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b --blocked-by 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .end();
}

#[test]
fn new_one_blocking_one_short() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b -b 1")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .end();
}

#[test]
fn new_blocked_by_one_short() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b -p 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(New))
        .end();
}

#[test]
fn new_blocking_multiple() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo new d -b 1 2 3")
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete).action(New))
        .printed_task(&PrintableTask::new("a", 2, Blocked))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .end();
}

#[test]
fn new_blocking_and_blocked_by() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c -p 1 -b 2")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_in_between_blocking_pair() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p 1 -b 2")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_one_before_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --before b")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Blocked).action(New))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn new_three_before_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --before b")
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
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b c d -p a");
    fix.test("todo new e --before b c d")
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
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d --after b")
        .validate()
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("d", 3, Blocked).action(New))
        .printed_task(&PrintableTask::new("c", 4, Blocked))
        .end();
}

#[test]
fn new_three_after_one() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --after b")
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
    let mut fix = Fixture::new();
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
#[ignore = "app.new.print-warning-on-cycle"]
fn print_warning_on_cycle() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p b -b a")
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: 3,
            requested_dependency: 1,
        })
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(New))
        .end();
}

#[test]
#[ignore = "app.new.priority"]
fn new_with_priority() {
    let mut fix = Fixture::new();
    fix.test("todo new a --priority 1")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(New)
                .priority(1),
        )
        .end();
}

#[test]
#[ignore = "app.new.priority"]
fn new_task_with_priority_inserted_before_unprioritized_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c --priority 1")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete)
                .action(New)
                .priority(1),
        )
        .end();
}

#[test]
#[ignore = "app.new.priority"]
fn new_task_with_negative_priority_inserted_after_unprioritized_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c --priority -1")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 3, Incomplete)
                .action(New)
                .priority(-1),
        )
        .end();
}

#[test]
#[ignore = "app.new.priority"]
fn new_task_with_priority_inserted_in_sorted_order() {
    let mut fix = Fixture::new();
    fix.test("todo new a --priority 1");
    fix.test("todo new b --priority 3");
    fix.test("todo new c --priority 2")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 2, Incomplete)
                .action(New)
                .priority(2),
        )
        .end();
}
