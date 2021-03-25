use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;

#[test]
fn status_while_empty() {
    let mut list = TodoList::new();
    test(&mut list, &["todo"]).validate().end();
}

#[test]
fn status_after_added_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn status_does_not_include_blocked_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn include_blocked_in_status() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "1", "--on", "2"]);
    test(&mut list, &["todo", "-b"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn include_complete_in_status() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "-d"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn include_all_in_status() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "--chain"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "-a"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn status_after_check_multiple_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "2", "3"])
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
    test(&mut list, &["todo"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn status_after_unblocking_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "unblock", "2", "--from", "1"]);
    test(&mut list, &["todo"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}
