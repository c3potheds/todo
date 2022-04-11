#![allow(clippy::zero_prefixed_literal)]

use super::*;
use chrono::TimeZone;
use chrono::Utc;

#[test]
fn complete_task_shows_up_in_complete_list() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
fn iterate_multiple_complete_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a).expect("Could not check a");
    list.check(c).expect("Could not check c");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(c));
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn incomplete_tasks_includes_blocked_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn blocked_task_comes_after_all_unblocked_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(a).on(b).expect("Could not block a on b");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn all_tasks_when_all_are_incomplete() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn all_tasks_when_all_are_complete() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn all_tasks_when_some_are_complete_and_some_are_blocked() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a).expect("Could not check a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn sort_by_priority_then_due_date() {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .priority(2)
            .due_date(Utc.ymd(2021, 04, 11).and_hms(13, 00, 00)),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .priority(1)
            .due_date(Utc.ymd(2021, 04, 11).and_hms(11, 00, 00)),
    );
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .priority(2)
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 00, 00)),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![c, a, b]);
}
