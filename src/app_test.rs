use app::*;
use cli::Options;
use model::TodoList;
use printing::Expect;
use printing::FakePrinter;
use std::ffi::OsString;
use structopt::StructOpt;

fn test<I>(list: &mut TodoList, args: I) -> FakePrinter
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let mut printer = FakePrinter::new();
    todo(list, &mut printer, &Options::from_iter_safe(args).unwrap());
    printer
}

#[test]
fn new_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"])
        .validate()
        .printed(&[Expect::Desc("a"), Expect::Number(1)]);
}

#[test]
fn new_multiple_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"])
        .validate()
        .printed(&[Expect::Desc("a"), Expect::Number(1)])
        .printed(&[Expect::Desc("b"), Expect::Number(2)])
        .printed(&[Expect::Desc("c"), Expect::Number(3)])
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
        .printed(&[Expect::Desc("a"), Expect::Number(1)])
        .printed(&[Expect::Desc("b"), Expect::Number(2)])
        .printed(&[Expect::Desc("c"), Expect::Number(3)])
        .end();
}

#[test]
fn check_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1"])
        .validate()
        .printed(&[Expect::Desc("a"), Expect::Number(0)])
        .end();
}

#[test]
fn status_after_check_multiple_tasks() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "2", "3"])
        .validate()
        .printed(&[Expect::Desc("b"), Expect::Number(-1)])
        .printed(&[Expect::Desc("c"), Expect::Number(0)])
        .end();
    test(&mut list, &["todo"])
        .validate()
        .printed(&[Expect::Desc("a"), Expect::Number(1)])
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
        .printed(&[Expect::Desc("b"), Expect::Number(0)])
        .end();
}

#[test]
fn log_after_multiple_tasks_completed() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "check", "1", "3"]);
    test(&mut list, &["todo", "log"])
        .validate()
        .printed(&[Expect::Desc("c"), Expect::Number(0)])
        .printed(&[Expect::Desc("a"), Expect::Number(-1)])
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
        .printed(&[Expect::Desc("b"), Expect::Number(2)])
        .end();
}
