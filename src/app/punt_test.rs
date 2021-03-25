use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;

#[test]
fn punt_first_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "punt", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Punt),
        ])
        .end();
}

#[test]
fn punt_blocked_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "c", "-p", "1"]);
    test(&mut list, &["todo", "punt", "2"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Punt),
        ])
        .end();
}

#[test]
fn punt_by_name() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "punt", "a"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Punt),
        ])
        .end();
}
