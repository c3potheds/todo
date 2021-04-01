use app::testing::Fixture;
use model::TaskStatus;
use printing::Action;
use printing::Expect;
use printing::PrintableError;
use printing::PrintableWarning;

#[test]
fn restore_incomplete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo restore 1")
        .validate()
        .printed_warning(
            &PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                cannot_restore: 1,
            },
        )
        .end();
}

#[test]
fn restore_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check 1");
    fix.test("todo restore 0")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}

#[test]
fn restore_task_with_negative_number() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo check 1");
    fix.test("todo check 1");
    fix.test("todo restore -1")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}

#[test]
fn restore_same_task_with_multiple_keys() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo check 1");
    fix.test("todo restore 0 0")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}

#[test]
fn restore_task_with_incomplete_antidependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo block b --on a");
    fix.test("todo check 1");
    fix.test("todo restore 0")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn restore_task_with_complete_antidependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo block b --on a");
    fix.test("todo check 1");
    fix.test("todo check 1");
    fix.test("todo restore -1")
        .validate()
        .printed_error(
            &PrintableError::CannotRestoreBecauseAntidependencyIsComplete {
                cannot_restore: -1,
                complete_antidependencies: vec![0],
            },
        )
        .end();
}

#[test]
fn restore_by_name() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo check a");
    fix.test("todo restore a")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}

#[test]
fn force_restore_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo restore a --force")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}

#[test]
fn force_restore_incomplete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo restore a --force")
        .validate()
        .printed_warning(
            &PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                cannot_restore: 1,
            },
        )
        .end();
}

#[test]
fn force_restore_task_with_complete_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo check a b");
    fix.test("todo restore a --force")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}

#[test]
fn force_restore_task_with_complete_adeps_with_complete_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo check a b c");
    fix.test("todo restore a --force")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Uncheck),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}

#[test]
fn force_restore_task_with_complete_and_incomplete_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d --chain");
    fix.test("todo check a b c");
    fix.test("todo restore a --force")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Uncheck),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Uncheck),
        ])
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}
