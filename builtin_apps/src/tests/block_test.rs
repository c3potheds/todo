use {
    super::testing::task,
    super::testing::Fixture,
    todo_app::Mutated,
    todo_printing::{
        Action::*, BriefPrintableTask, Plicit::*, PrintableError, Status::*,
    },
};

#[test]
fn block_one_on_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 1 --on 2")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn block_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block a --on b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("a", 2, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn block_one_on_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo block 1 --on 2 3 4")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", 1, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("c", 2, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("d", 3, Incomplete).adeps_stats(0, 1))
        .printed_task(&task("a", 4, Blocked).action(Lock).deps_stats(3, 3))
        .end();
}

#[test]
fn block_three_on_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo block 1 2 3 --on 4")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("d", 1, Incomplete).adeps_stats(3, 3))
        .printed_task(&task("a", 2, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("b", 3, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("c", 4, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn block_on_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check 1 2");
    fix.test("todo block 1 --on -1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -1, Complete))
        .printed_task(&task("c", 1, Incomplete).action(Lock))
        .end();
}

#[test]
fn block_multiple_on_following_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d");
    fix.test("todo block 1 2 --on 3")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("c", 1, Incomplete).adeps_stats(2, 2))
        .printed_task(&task("a", 3, Blocked).action(Lock).deps_stats(1, 1))
        .printed_task(&task("b", 4, Blocked).action(Lock).deps_stats(1, 1))
        .end();
}

#[test]
fn cannot_block_on_self() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo block 1 --on 1")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(1, Incomplete),
        })
        .end();
}

#[test]
fn cannot_block_on_adep() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo block a --on b")
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(2, Blocked),
        })
        .end();
}

#[test]
fn block_updates_implicit_priority_of_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo block c --on b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 2),
        )
        .printed_task(
            &task("b", 2, Blocked).priority(Implicit(1)).deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .action(Lock)
                .priority(Explicit(1))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn block_does_not_print_priority_updates_for_unaffected_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain --priority 1");
    fix.test("todo new c --priority 1");
    fix.test("todo block c --on b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("b", 2, Blocked).priority(Explicit(1)).deps_stats(1, 1),
        )
        .printed_task(
            &task("c", 3, Blocked)
                .action(Lock)
                .priority(Explicit(1))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn block_excludes_complete_affected_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo check a");
    fix.test("todo block c --on b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(
            &task("b", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &task("c", 2, Blocked)
                .action(Lock)
                .priority(Explicit(1))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn block_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo check a");
    fix.test("todo block c --on b -d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 0, Complete).priority(Implicit(1)))
        .printed_task(
            &task("b", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &task("c", 2, Blocked)
                .action(Lock)
                .priority(Explicit(1))
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn block_complete_task_on_preceding_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --done");
    fix.test("todo block b --on a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -1, Complete))
        .printed_task(&task("b", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_complete_task_on_distantly_preceding_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --done");
    fix.test("todo block e --on a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", -4, Complete))
        .printed_task(&task("e", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_complete_task_on_later_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --done");
    fix.test("todo block a --on b")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("b", -1, Complete))
        .printed_task(&task("a", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_complete_task_on_distant_later_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --done");
    fix.test("todo block a --on e")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("e", -1, Complete))
        .printed_task(&task("a", 0, Complete).action(Lock))
        .end();
}

#[test]
fn block_multiple_complete_tasks_on_later_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d --done");
    fix.test("todo block a b --on d")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("d", -2, Complete))
        .printed_task(&task("a", -1, Complete).action(Lock))
        .printed_task(&task("b", 0, Complete).action(Lock))
        .end();
}

#[test]
#[ignore = "This is an optimization that can be done later"]
fn redundant_block_does_not_modify() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo block b --on a");
    fix.test("todo block b --on a")
        .modified(Mutated::No)
        .validate()
        .printed_task(&task("a", 0, Incomplete))
        .printed_task(&task("b", 1, Blocked).action(Lock))
        .end();
}
