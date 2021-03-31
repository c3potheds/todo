use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;
use printing::PrintableError;
use printing::PrintableWarning;

#[test]
fn check_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn check_by_name() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "b"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn check_task_with_incomplete_dependencies() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "check", "2"])
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: 2,
            blocked_by: vec![1],
        })
        .end();
}

#[test]
fn cannot_check_blocked_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "1", "--on", "2"]);
    test(&mut list, &["todo", "check", "2"])
        .validate()
        .printed_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: 2,
            blocked_by: vec![1],
        })
        .end();
}

#[test]
fn check_newly_unblocked_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "1", "--on", "2"]);
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Unlock),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
        ])
        .end();
}

#[test]
fn check_newly_unblocked_task_with_multiple_dependencies() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "block", "1", "--on", "2", "3"]);
    test(&mut list, &["todo", "check", "1", "2"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Unlock),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
        ])
        .end();
}

#[test]
fn check_newly_unblocked_task_with_chained_dependencies() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "block", "3", "--on", "2"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Unlock),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Unlock),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn check_does_not_show_adeps_that_are_not_unlocked() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "c", "-p", "1", "--chain"]);
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Unlock),
        ])
        // Do not print c, even though it's a direct adep, because it has not
        // been unlocked.
        .end();
}

#[test]
fn check_same_task_twice_in_one_command() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "check", "1", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn check_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo check a")
        .validate()
        .printed_warning(&PrintableWarning::CannotCheckBecauseAlreadyComplete {
            cannot_check: 0,
        })
        .end();
}

#[test]
fn force_check_incomplete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check a --force")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn force_check_blocked_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo check b --force")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn force_check_transitively_blocked_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo check c --force")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(-2),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn force_check_task_with_complete_deps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo check a");
    fix.test("todo check c --force")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn force_check_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo check a --force")
        .validate()
        .printed_warning(&PrintableWarning::CannotCheckBecauseAlreadyComplete {
            cannot_check: 0,
        })
        .end();
}
