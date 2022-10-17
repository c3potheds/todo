use {
    super::testing::Fixture,
    printing::{
        Action::*, BriefPrintableTask, Plicit::*, PrintableError,
        PrintableTask, Status::*,
    },
};

#[test]
fn chain_one() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo chain a").modified(false).validate().end();
}

#[test]
fn chain_three() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo chain a b c")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(
            &PrintableTask::new("b", 4, Blocked)
                .action(Lock)
                .deps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("c", 5, Blocked)
                .action(Lock)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn chain_would_cause_cycle() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo chain b a")
        .modified(false)
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: BriefPrintableTask::new(1, Incomplete),
            requested_dependency: BriefPrintableTask::new(2, Blocked),
        })
        .end();
}

#[test]
fn chain_shows_affected_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo chain b c")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 2),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .priority(Implicit(1))
                .deps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .priority(Explicit(1))
                .action(Lock)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn chain_excludes_complete_affected_deps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c --priority 1");
    fix.test("todo check a");
    fix.test("todo chain b c")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("b", 1, Incomplete)
                .priority(Implicit(1))
                .adeps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("c", 2, Blocked)
                .priority(Explicit(1))
                .action(Lock)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn chain_by_range() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo chain 1..3")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(Lock)
                .deps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .action(Lock)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn chain_ambiguous_key() {
    let mut fix = Fixture::default();
    fix.test("todo new a a a");
    fix.test("todo chain a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).adeps_stats(1, 2))
        .printed_task(
            &PrintableTask::new("a", 2, Blocked)
                .action(Lock)
                .deps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("a", 3, Blocked)
                .action(Lock)
                .deps_stats(1, 2),
        )
        .end();
}
