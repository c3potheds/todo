#![allow(clippy::zero_prefixed_literal)]

use chrono::TimeZone;
use chrono::Utc;

use super::*;

#[test]
fn set_due_date_simple() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let affected = list.set_due_date(
        a,
        Some(Utc.with_ymd_and_hms(2021, 04, 13, 17, 00, 00).unwrap()),
    );
    assert_eq!(
        list.get(a).unwrap().due_date,
        Some(Utc.with_ymd_and_hms(2021, 04, 13, 17, 00, 00).unwrap())
    );
    assert_eq!(
        list.get(a).unwrap().implicit_due_date,
        Some(Utc.with_ymd_and_hms(2021, 04, 13, 17, 00, 00).unwrap())
    );
    assert_eq!(affected.iter_sorted(&list).collect::<Vec<_>>(), vec![a]);
}

#[test]
fn set_due_date_returns_transitively_affected_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    list.block(b).on(a).unwrap();
    let affected = list.set_due_date(
        c,
        Some(Utc.with_ymd_and_hms(2021, 04, 13, 17, 00, 00).unwrap()),
    );
    assert_eq!(
        affected.iter_sorted(&list).collect::<Vec<_>>(),
        vec![a, b, c]
    );
}

#[test]
fn set_due_date_excludes_unaffected_tasks() {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 13, 16, 00, 00).unwrap()),
    );
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    list.block(b).on(a).unwrap();
    let affected = list.set_due_date(
        c,
        Some(Utc.with_ymd_and_hms(2021, 04, 13, 17, 00, 00).unwrap()),
    );
    assert_eq!(affected.iter_sorted(&list).collect::<Vec<_>>(), vec![b, c]);
}

#[test]
fn get_due_date() {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 08, 23, 59, 59).unwrap()),
    );
    assert_eq!(
        list.get(a).unwrap().due_date.unwrap(),
        Utc.with_ymd_and_hms(2021, 04, 08, 23, 59, 59).unwrap(),
    );
}

#[test]
fn due_date_from_new_options() {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 09, 12, 00, 00).unwrap()),
    );
    assert_eq!(
        list.get(a).unwrap().due_date.unwrap(),
        Utc.with_ymd_and_hms(2021, 04, 09, 12, 00, 00).unwrap()
    );
}

#[test]
fn sort_by_explicit_due_date() {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 26, 00).unwrap()),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 25, 00).unwrap()),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![b, a]);
}

#[test]
fn sort_keeps_task_with_earlier_due_date_first() {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 26, 00).unwrap()),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 30, 00).unwrap()),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![a, b]);
}

#[test]
fn sort_puts_task_with_due_date_before_task_without_due_date() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 25, 00).unwrap()),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![b, a]);
}

#[test]
fn sort_by_implicit_due_date() {
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 30, 00).unwrap()),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 12, 00, 00).unwrap()),
    );
    list.block(b).on(a).unwrap();
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 11, 00, 00).unwrap()),
    );
    let d = list.add(
        NewOptions::new()
            .desc("d")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 13, 00, 00).unwrap()),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![c, a, d, b]);
}
#[test]
fn implicit_due_date_of_task_with_no_adeps_or_due_date() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.implicit_due_date(a), Some(None));
}

#[test]
fn implicit_due_date_of_nonexistent_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.remove(a);
    assert_eq!(list.implicit_due_date(a), None);
}

#[test]
fn implicit_due_date_is_earliest_due_date_of_adeps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 19, 00, 00).unwrap()),
    );
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 20, 00, 00).unwrap()),
    );
    let d = list.add(
        NewOptions::new()
            .desc("d")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 18, 00, 00).unwrap()),
    );
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    list.block(d).on(a).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(
            Utc.with_ymd_and_hms(2021, 04, 11, 18, 00, 00).unwrap()
        ))
    );
}

#[test]
fn implicit_due_date_is_earliest_due_date_of_transitive_adeps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 19, 00, 00).unwrap()),
    );
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 20, 00, 00).unwrap()),
    );
    let d = list.add(
        NewOptions::new()
            .desc("d")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 11, 18, 00, 00).unwrap()),
    );
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    list.block(d).on(b).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(
            Utc.with_ymd_and_hms(2021, 04, 11, 18, 00, 00).unwrap()
        ))
    );
}
