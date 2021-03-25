use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;

#[test]
fn get_incomplete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "get", "2"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .end();
}

#[test]
fn get_complete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1", "2", "3"]);
    test(&mut list, &["todo", "get", "-2"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(-2),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Select),
        ])
        .end();
}

#[test]
fn get_multiple_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d", "e"]);
    test(&mut list, &["todo", "get", "2", "3", "4"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .end();
}

#[test]
fn get_shows_blocking_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "get", "2"])
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
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Select),
        ])
        .end();
}

#[test]
fn get_shows_blocked_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "get", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn get_shows_transitive_deps_and_adeps() {
    let mut list = TodoList::new();
    test(
        &mut list,
        &["todo", "new", "a", "b", "c", "d", "e", "--chain"],
    );
    test(&mut list, &["todo", "get", "3"])
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
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Select),
        ])
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("e"),
            Expect::Number(5),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn get_by_name_multiple_matches() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "bob", "frank", "bob"]);
    test(&mut list, &["todo", "get", "bob"])
        .validate()
        .printed_task(&[
            Expect::Desc("bob"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .printed_task(&[
            Expect::Desc("bob"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .end();
}
