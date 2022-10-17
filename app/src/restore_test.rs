use {
    super::testing::Fixture,
    printing::{
        Action::*, BriefPrintableTask, PrintableError, PrintableTask,
        PrintableWarning, Status::*,
    },
};

#[test]
fn restore_incomplete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo restore 1")
        .modified(false)
        .validate()
        .printed_warning(
            &PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                cannot_restore: BriefPrintableTask::new(1, Incomplete),
            },
        )
        .end();
}

#[test]
fn restore_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo restore 0")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Uncheck))
        .end();
}

#[test]
fn restore_task_with_negative_number() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo check 1");
    fix.test("todo check 1");
    fix.test("todo restore -1")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 2, Incomplete).action(Uncheck))
        .end();
}

#[test]
fn restore_same_task_with_multiple_keys() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo check 1");
    fix.test("todo restore 0 0")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 2, Incomplete).action(Uncheck))
        .end();
}

#[test]
fn restore_task_with_incomplete_antidependency() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block b --on a");
    fix.test("todo check 1");
    fix.test("todo restore 0")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(Uncheck)
                .adeps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(Lock)
                .deps_stats(1, 1),
        )
        .end();
}

#[test]
fn restore_task_with_complete_antidependency() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block b --on a");
    fix.test("todo check 1");
    fix.test("todo check 1");
    fix.test("todo restore -1")
        .modified(false)
        .validate()
        .printed_error(
            &PrintableError::CannotRestoreBecauseAntidependencyIsComplete {
                cannot_restore: BriefPrintableTask::new(-1, Complete),
                complete_antidependencies: vec![BriefPrintableTask::new(
                    0, Complete,
                )],
            },
        )
        .end();
}

#[test]
fn restore_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo check a");
    fix.test("todo restore a")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 2, Incomplete).action(Uncheck))
        .end();
}

#[test]
fn force_restore_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo restore a --force")
        .modified(true)
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Uncheck))
        .end();
}

#[test]
fn force_restore_incomplete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo restore a --force")
        .modified(false)
        .validate()
        .printed_warning(
            &PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                cannot_restore: BriefPrintableTask::new(1, Incomplete),
            },
        )
        .end();
}

#[test]
fn force_restore_task_with_complete_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a b");
    fix.test("todo restore a --force")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(Uncheck)
                .adeps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(Uncheck)
                .deps_stats(1, 1),
        )
        .end();
}

#[test]
fn force_restore_task_with_complete_adeps_with_complete_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b c");
    fix.test("todo restore a --force")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(Uncheck)
                .adeps_stats(1, 2),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(Uncheck)
                .deps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .action(Uncheck)
                .deps_stats(1, 2),
        )
        .end();
}

#[test]
fn force_restore_task_with_complete_and_incomplete_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d --chain");
    fix.test("todo check a b c");
    fix.test("todo restore a --force")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(Uncheck)
                .adeps_stats(1, 3),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(Uncheck)
                .deps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .action(Uncheck)
                .deps_stats(1, 2),
        )
        .printed_task(
            &PrintableTask::new("d", 4, Blocked)
                .action(Lock)
                .deps_stats(1, 3),
        )
        .end();
}

#[test]
fn restore_chain() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a b");
    fix.test("todo restore a b")
        .modified(true)
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .action(Uncheck)
                .adeps_stats(1, 1),
        )
        .printed_task(
            &PrintableTask::new("b", 2, Blocked)
                .action(Uncheck)
                .deps_stats(1, 1),
        )
        .end();
}
