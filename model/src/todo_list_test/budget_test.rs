#![allow(clippy::zero_prefixed_literal)]

use {
    super::*,
    chrono::{Duration, TimeZone, Utc},
};

#[test]
fn default_budget_is_zero() {
    let mut list = TodoList::default();
    let budget = DurationInSeconds(0);
    let a = list.add(NewOptions::new().desc("a"));
    assert_eq!(list.get(a).unwrap().budget, budget);
}

#[test]
fn new_task_with_budget() {
    let mut list = TodoList::default();
    let budget = DurationInSeconds::from(Duration::days(1));
    let a = list.add(NewOptions::new().desc("a").budget(budget));
    assert_eq!(list.get(a).unwrap().budget, budget);
}

#[test]
fn dep_of_task_with_budget_incorporates_budget_in_due_date() {
    let mut list = TodoList::default();
    let budget = DurationInSeconds::from(Duration::days(1));
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 22, 23, 59, 59).unwrap())
            .budget(budget),
    );
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(
            Utc.with_ymd_and_hms(2021, 04, 21, 23, 59, 59).unwrap()
        ))
    );
}

#[test]
fn chain_of_tasks_with_budgets() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").budget(Duration::days(1)));
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 22, 23, 59, 59).unwrap())
            .budget(Duration::days(1)),
    );
    list.block(b).on(a).unwrap();
    assert_eq!(list.implicit_due_date(a), Some(None));
    list.block(c).on(b).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(
            Utc.with_ymd_and_hms(2021, 04, 20, 23, 59, 59).unwrap()
        ))
    );
}

#[test]
fn set_budget_for_nonexistent_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.remove(a);
    assert_eq!(
        list.set_budget(a, Duration::days(1))
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        vec![]
    );
}

#[test]
fn set_budget_for_task_with_no_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(
        list.set_budget(a, Duration::days(1))
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        vec![a]
    );
    assert_eq!(
        list.get(a).unwrap().budget,
        DurationInSeconds(Duration::days(1).num_seconds() as u32)
    );
}

#[test]
fn set_budget_updates_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.with_ymd_and_hms(2021, 04, 22, 23, 59, 59).unwrap()),
    );
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.set_budget(b, Duration::days(1))
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        vec![a, b]
    );
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(
            Utc.with_ymd_and_hms(2021, 04, 21, 23, 59, 59).unwrap()
        ))
    );
}
