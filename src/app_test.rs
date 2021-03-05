use app::*;
use cli::Options;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;
use printing::FakePrinter;
use printing::PrintingContext;
use std::ffi::OsString;
use structopt::StructOpt;

fn test<I>(list: &mut TodoList, args: I) -> FakePrinter
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let printing_context = PrintingContext {
        // TODO: Get the number of tasks from the list.
        max_index_digits: 3,
        width: 80,
    };
    let mut printer = FakePrinter::new();
    todo(
        list,
        &printing_context,
        &mut printer,
        &Options::from_iter_safe(args).unwrap(),
    );
    printer
}

#[test]
fn new_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Action(Action::New),
        ])
        .end();
}

#[test]
fn new_multiple_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .end();
}

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
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed(&[
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
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed(&[
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
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn check_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn status_after_check_multiple_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "2", "3"])
        .validate()
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
    test(&mut list, &["todo"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

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
        .printed(&[
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
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn restore_incomplete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "restore", "1"]).validate().end();
}

#[test]
fn restore_complete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "restore", "0"])
        .validate()
        .printed(&[Expect::Desc("a"), Expect::Number(1)])
        .end();
}

#[test]
#[ignore = "Figure out how to parse negative numbers."]
fn restore_task_with_negative_number() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "a", "b"]);
    test(&mut list, &["todo", "restore", "-1"])
        .validate()
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
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
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
        ])
        .end();
}

#[test]
fn cannot_block_on_self() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "block", "1", "--on", "1"])
        .validate()
        .end();
}

#[test]
fn block_one_on_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "1", "--on", "2"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
        ])
        .end();
}

#[test]
fn block_one_on_three() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "block", "1", "--on", "2", "3", "4"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
        ])
        .end();
}

#[test]
fn block_three_on_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "block", "1", "2", "3", "--on", "4"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
        ])
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
        ])
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
        ])
        .end();
}

#[test]
fn cannot_check_blocked_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "1", "--on", "2"]);
    test(&mut list, &["todo", "check", "2"]).validate().end();
}

#[test]
fn check_newly_unblocked_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "1", "--on", "2"]);
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed(&[
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
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
        ])
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed(&[
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
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn check_same_task_twice_in_one_command() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "check", "1", "1"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::Check),
        ])
        .end();
}

#[test]
fn new_one_blocking_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "--blocking", "1"])
        .validate()
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn new_blocked_by_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "--blocked-by", "1"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::New),
        ])
        .end();
}

#[test]
fn new_one_blocking_one_short() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "-b", "1"])
        .validate()
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn new_blocked_by_one_short() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "-p", "1"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::New),
        ])
        .end();
}

#[test]
fn new_blocking_multiple() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "new", "d", "-b", "1", "2", "3"])
        .validate()
        .printed(&[
            Expect::Desc("d"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn new_blocking_and_blocked_by() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "new", "c", "-p", "1", "-b", "2"])
        .validate()
        .printed(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed(&[
            Expect::Desc("c"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::New),
        ])
        .printed(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}
