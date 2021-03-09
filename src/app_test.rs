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
    let options = Options::from_iter_safe(args).expect("Could not parse args");
    todo(list, &printing_context, &mut printer, &options);
    printer
}

#[test]
fn new_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"])
        .validate()
        .printed_task(&[
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
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .end();
}

#[test]
fn new_block_on_complete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "new", "b", "-p", "0"])
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
            Expect::Action(Action::New),
        ])
        .end();
}

#[test]
#[ignore = "Need to be able to record errors."]
fn new_blocking_complete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "new", "b", "-p", "0"])
        .validate()
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
#[ignore = "Need to be able to record errors."]
fn check_task_with_incomplete_dependencies() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    // TODO: Print a warning explaining why it can't be checked.
    test(&mut list, &["todo", "check", "2"]).validate().end();
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
        .printed_task(&[Expect::Desc("a"), Expect::Number(1)])
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
        ])
        .end();
}

#[test]
#[ignore = "TODO: Show implicitly restored tasks."]
fn restore_task_with_complete_antidependency() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    test(&mut list, &["todo", "block", "b", "--on", "a"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "check", "1"]);
    test(&mut list, &["todo", "restore", "-1"])
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

#[test]
#[ignore = "Record warning printing"]
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
        ])
        .end();
}

#[test]
fn block_one_on_three() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "block", "1", "--on", "2", "3", "4"])
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
        ])
        .end();
}

#[test]
fn block_three_on_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "block", "1", "2", "3", "--on", "4"])
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
fn block_on_complete_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1", "2"]);
    test(&mut list, &["todo", "block", "1", "--on", "-1"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(-1),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
fn block_multiple_on_following_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "d"]);
    test(&mut list, &["todo", "block", "1", "2", "--on", "3"])
        .validate()
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(4),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Lock),
        ])
        .end();
}

#[test]
#[ignore = "Need to be able to record printed errors."]
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
fn new_one_blocking_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "--blocking", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
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
fn new_blocked_by_one() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "--blocked-by", "1"])
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
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
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
fn new_blocked_by_one_short() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "new", "b", "-p", "1"])
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
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
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
            Expect::Action(Action::New),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn new_in_between_blocking_pair() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "--chain"]);
    test(&mut list, &["todo", "new", "c", "-p", "1", "-b", "2"])
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
            Expect::Action(Action::New),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

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
#[test]
#[ignore = "Need to be able to record warnings."]
fn unblock_task_from_indirect_dependency() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "block", "3", "--on", "2"]);
    test(&mut list, &["todo", "block", "2", "--on", "1"]);
    test(&mut list, &["todo", "unblock", "3", "--from", "1"])
        .validate()
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
fn new_chain_three() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c", "--chain"])
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::New),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::New),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::New),
        ])
        .end();
}

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
fn edit_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "edit", "1", "--desc", "b"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}
