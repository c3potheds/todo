use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;
use printing::PrintableWarning;

#[test]
fn unblock_task_from_direct_dependency() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "unblock", "2", "--from", "1"])
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
            Expect::Action(Action::Unlock),
        ])
        .end();
}

#[test]
fn unblock_task_from_indirect_dependency() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "block", "3", "--on", "2"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "unblock", "3", "--from", "1"])
        .validate()
        .printed_warning(
            &PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                cannot_unblock: 3,
                requested_unblock_from: 1,
            },
        )
        .end();
}

#[test]
fn unblock_complete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "--chain"]);
    test(&mut list, &["todo", "check", "1", "2"]);
    test(&mut list, &["todo", "unblock", "0", "--from", "-1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Unlock),
        ])
        .end();
}

#[test]
fn unblock_by_name() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "--chain"]);
    test(&mut list, &["todo", "unblock", "b", "--from", "a"])
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
            Expect::Action(Action::Unlock),
        ])
        .end();
}

#[test]
fn unblock_from_all() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo unblock c")
        .validate()
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Unlock),
        ])
        .end();
}

#[test]
fn unblock_from_all2() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p a b");
    fix.test("todo unblock c")
        .validate()
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Unlock),
        ])
        .end();
}
