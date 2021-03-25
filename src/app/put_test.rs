use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;
use printing::PrintableError;

#[test]
fn put_one_after_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "put", "a", "--after", "b"])
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
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_three_after_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "put", "a", "b", "c", "--after", "d"])
        .validate()
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_one_after_three() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "put", "a", "--after", "b", "c", "d"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
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
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_after_task_with_adeps() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "--chain"]);
    test(&mut list, &["todo", "new", "c"]);
    test(&mut list, &["todo", "put", "c", "--after", "a"])
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
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_one_before_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "put", "b", "--before", "a"])
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
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_three_before_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "put", "b", "c", "d", "--before", "a"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
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
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_one_before_three() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "put", "d", "--before", "a", "b", "c"])
        .validate()
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_before_task_with_deps() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "--chain"]);
    test(&mut list, &["todo", "new", "c"]);
    test(&mut list, &["todo", "put", "c", "--before", "b"])
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
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn put_before_and_after() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "--chain"]);
    test(&mut list, &["todo", "new", "d", "e", "f", "--chain"]);
    test(&mut list, &["todo", "new", "g"]);
    test(&mut list, &["todo", "put", "g", "-b", "b", "-a", "e"])
        .validate()
        .printed_task(&[Expect::Desc("a")])
        .printed_task(&[Expect::Desc("e")])
        .printed_task(&[Expect::Desc("g")])
        .printed_task(&[Expect::Desc("b")])
        .printed_task(&[Expect::Desc("f")])
        .end();
}

#[test]
fn put_causing_cycle() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "--chain"]);
    test(&mut list, &["todo", "put", "a", "--after", "b"])
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: 1,
            requested_dependency: 2,
        })
        .end();
    test(&mut list, &["todo", "-a"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Status(TaskStatus::Incomplete),
        ])
        .printed_task(&[Expect::Desc("b"), Expect::Status(TaskStatus::Blocked)])
        .end();
}
