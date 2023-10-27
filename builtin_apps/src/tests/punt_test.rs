use {
    super::testing::task,
    super::testing::Fixture,
    todo_app::Mutated,
    todo_printing::{
        Action::*, BriefPrintableTask,
        PrintableWarning::CannotPuntBecauseComplete, Status::*,
    },
};

#[test]
fn punt_first_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo punt 1")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 3, Incomplete).action(Punt))
        .end();
}

#[test]
fn punt_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b c -p 1");
    fix.test("todo punt 2")
        // TODO: Since the position of the task doesn't change, we don't
        // need to mark the session as modified.
        // .modified(Mutated::No)
        .validate()
        .printed_task(&task("b", 3, Blocked).action(Punt).deps_stats(1, 1))
        .end();
}

#[test]
fn punt_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo punt a")
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("a", 3, Incomplete).action(Punt))
        .end();
}

#[test]
fn punt_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a -d");
    fix.test("todo punt a")
        .modified(Mutated::No)
        .validate()
        .printed_warning(&CannotPuntBecauseComplete {
            cannot_punt: BriefPrintableTask::new(0, Complete),
        })
        .end();
}
