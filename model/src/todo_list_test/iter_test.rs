#![allow(clippy::zero_prefixed_literal)]

use {
    super::*,
    ::pretty_assertions::assert_eq,
    chrono::{TimeZone, Utc},
};

#[test]
fn complete_task_shows_up_in_complete_list() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
    Ok(())
}

#[test]
fn iterate_multiple_complete_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a)?;
    list.check(c)?;
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(c));
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn incomplete_tasks_includes_blocked_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn blocked_task_comes_after_all_unblocked_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(a).on(b)?;
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn all_tasks_when_all_are_incomplete() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, b, c]);
    Ok(())
}

#[test]
fn all_tasks_when_all_are_complete() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a)?;
    list.check(b)?;
    list.check(c)?;
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, b, c]);
    Ok(())
}

#[test]
fn all_tasks_when_some_are_complete_and_some_are_blocked() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, b, c]);
    Ok(())
}

#[test]
fn sort_by_priority_then_due_date() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .priority(2)
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 13, 00, 00).unwrap()),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .priority(1)
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 11, 00, 00).unwrap()),
    );
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .priority(2)
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 00, 00).unwrap()),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [c, a, b]);
    Ok(())
}
