use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;

#[test]
fn log_with_no_tasks_completed() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "log"]).validate().end();
}

#[test]
fn log_after_single_task_completed() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "2"]);
    test(&mut list, &["todo", "log"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn log_after_multiple_tasks_completed() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1", "3"]);
    test(&mut list, &["todo", "log"])
        .validate()
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .end();
}
