use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;
use printing::PrintableError;

#[test]
fn chain_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "chain", "a"]).validate().end();
}

#[test]
fn chain_three() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d", "e"]);
    test(&mut list, &["todo", "chain", "a", "b", "c"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(5),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn chain_would_cause_cycle() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "--chain"]);
    test(&mut list, &["todo", "chain", "b", "a"])
        .validate()
        .printed_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: 1,
            requested_dependency: 2,
        })
        .end();
}
