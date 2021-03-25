use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;
use printing::PrintableError;
use printing::PrintableWarning;

#[test]
fn restore_incomplete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "restore", "1"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "restore", "0"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "restore", "-1"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "restore", "0", "0"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "b", "--on", "a"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "restore", "0"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "b", "--on", "a"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "restore", "-1"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "check", "a"]);
    test(&mut list, &["todo", "restore", "a"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Uncheck),
        ])
        .end();
}
