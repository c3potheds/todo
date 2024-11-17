#![allow(clippy::zero_prefixed_literal)]

use ::pretty_assertions::assert_eq;
use chrono::TimeZone;
use chrono::Utc;

use super::*;

#[test]
fn check_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    list.check(a)
        .expect_err("Shouldn't have been able to check a");
    Ok(())
}

#[test]
fn checked_task_has_completion_time() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    assert!(list.get(a).unwrap().completion_time.is_some());
    Ok(())
}

#[test]
fn completion_time_of_completed_task_does_not_update_if_checked() -> TestResult
{
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    let original_completion_time =
        list.get(a).unwrap().completion_time.unwrap();
    list.check(a)
        .expect_err("Shouldn't have been able to check a");
    let new_completion_time = list.get(a).unwrap().completion_time.unwrap();
    assert_eq!(original_completion_time, new_completion_time);
    Ok(())
}

#[test]
fn check_by_options_uses_injected_completion_time() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let now = Utc.with_ymd_and_hms(2021, 03, 26, 04, 27, 00).unwrap();
    list.check(CheckOptions { id: a, now }).unwrap();
    assert_eq!(list.get(a).unwrap().completion_time, Some(now));
    Ok(())
}

#[test]
fn check_first_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    list.check(a)?;
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
    Ok(())
}

#[test]
fn check_second_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    list.check(b)?;
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
    Ok(())
}

#[test]
fn check_third_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    list.check(c)?;
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), None);
    Ok(())
}

#[test]
fn cannot_check_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    assert_eq!(list.check(b), Err(CheckError::TaskIsBlockedBy(vec![a])));
    Ok(())
}

#[test]
fn can_check_task_whose_dependency_is_complete() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.check(a)?;
    list.check(b)?;
    Ok(())
}

#[test]
fn can_check_task_whose_dependencies_are_complete() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a)?;
    list.block(c).on(b)?;
    list.check(a)?;
    list.check(b)?;
    list.check(c)?;
    Ok(())
}

#[test]
fn force_check_incomplete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let result = list.force_check(a).unwrap();
    assert_eq!(result.completed.as_sorted_vec(&list), [a]);
    assert_eq!(result.unblocked.as_sorted_vec(&list), []);
    Ok(())
}

#[test]
fn force_check_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let result = list.force_check(b).unwrap();
    assert_eq!(result.completed.as_sorted_vec(&list), [a, b]);
    assert_eq!(result.unblocked.as_sorted_vec(&list), [c]);
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
    assert_eq!(list.status(b), Some(TaskStatus::Complete));
    assert_eq!(list.status(c), Some(TaskStatus::Incomplete));
    Ok(())
}

#[test]
fn force_check_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a).unwrap();
    assert_eq!(list.force_check(a), Err(CheckError::TaskIsAlreadyComplete));
    Ok(())
}
