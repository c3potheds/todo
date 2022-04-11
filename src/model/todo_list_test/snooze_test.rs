#![allow(clippy::zero_prefixed_literal)]

use super::*;
use chrono::TimeZone;
use chrono::Utc;

#[test]
fn start_date_defaults_to_creation_time() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a"));
    assert_eq!(
        list.get(a).unwrap().start_date,
        list.get(a).unwrap().creation_time
    );
}

#[test]
fn set_start_date_in_new_options() {
    let mut list = TodoList::default();
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(NewOptions::new().desc("a").start_date(start_date));
    assert_eq!(list.get(a).unwrap().start_date, start_date);
}

#[test]
fn new_task_with_start_time_later_than_now_starts_out_snoozed() {
    let mut list = TodoList::default();
    let now = Utc.ymd(2021, 05, 25).and_hms(09, 00, 00);
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    assert_eq!(list.status(a).unwrap(), TaskStatus::Blocked);
}

#[test]
fn unsnooze_up_to_before_snooze_date() {
    let mut list = TodoList::default();
    let now = Utc.ymd(2021, 05, 25).and_hms(09, 00, 00);
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    let now = Utc.ymd(2021, 05, 30).and_hms(09, 00, 00);
    let unsnoozed = list.unsnooze_up_to(now);
    assert_eq!(unsnoozed.len(), 0);
    assert_eq!(list.status(a).unwrap(), TaskStatus::Blocked);
}

#[test]
fn unsnooze_up_to_after_snooze_date() {
    let mut list = TodoList::default();
    let now = Utc.ymd(2021, 05, 25).and_hms(09, 00, 00);
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    let now = Utc.ymd(2021, 06, 01).and_hms(09, 00, 00);
    let unsnoozed = list.unsnooze_up_to(now);
    itertools::assert_equal(unsnoozed.iter_sorted(&list), vec![a]);
    assert_eq!(list.status(a).unwrap(), TaskStatus::Incomplete);
}

#[test]
fn unsnooze_up_to_unsnoozes_multiple_tasks() {
    let mut list = TodoList::default();
    let now = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let snooze_a = Utc.ymd(2021, 06, 02).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    let snooze_b = Utc.ymd(2021, 06, 03).and_hms(00, 00, 00);
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(now)
            .start_date(snooze_b),
    );
    let snooze_c = Utc.ymd(2021, 06, 04).and_hms(00, 00, 00);
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .creation_time(now)
            .start_date(snooze_c),
    );
    let now = snooze_b;
    let unsnoozed = list.unsnooze_up_to(now);
    itertools::assert_equal(unsnoozed.iter_sorted(&list), vec![a, b]);
    let now = snooze_c;
    let unsnoozed = list.unsnooze_up_to(now);
    itertools::assert_equal(unsnoozed.iter_sorted(&list), vec![c]);
}

#[test]
fn unsnooze_updates_depth_of_adeps() {
    let mut list = TodoList::default();
    let now = Utc.ymd(2021, 05, 25).and_hms(10, 00, 00);
    let snooze_a = Utc.ymd(2021, 05, 25).and_hms(11, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    let b = list.add(NewOptions::new().desc("b").due_date(snooze_a));
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a).unwrap();
    list.block(d).on(c).unwrap();
    // c is first because it is unblocked an unsnoozed.
    // a is next because it's snoozed, but was added before d, which is blocked
    // by c.
    // b is blocked by a, and so appears in a deeper layer than a.
    itertools::assert_equal(list.incomplete_tasks(), vec![c, a, d, b]);
    let now = Utc.ymd(2021, 05, 25).and_hms(12, 00, 00);
    list.unsnooze_up_to(now);
    // a and b now appear before c and d, respectively, because they are in
    // the same layer, and have a due date which sorts them earlier the other
    // tasks with no due date.
    itertools::assert_equal(list.incomplete_tasks(), vec![a, c, b, d]);
}

#[test]
fn check_snoozed_task() {
    let mut list = TodoList::default();
    let now = Utc.ymd(2021, 05, 25).and_hms(13, 00, 00);
    let snooze_a = Utc.ymd(2021, 05, 25).and_hms(14, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    itertools::assert_equal(
        list.check(CheckOptions { id: a, now })
            .unwrap()
            .iter_sorted(&list),
        vec![],
    );
    itertools::assert_equal(list.incomplete_tasks(), vec![]);
    itertools::assert_equal(list.complete_tasks(), vec![a]);
}

#[test]
fn force_check_snoozed_task() {
    let mut list = TodoList::default();
    let now = Utc.ymd(2021, 05, 25).and_hms(13, 00, 00);
    let snooze_a = Utc.ymd(2021, 05, 25).and_hms(14, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    itertools::assert_equal(
        list.force_check(CheckOptions { id: a, now })
            .unwrap()
            .completed
            .iter_sorted(&list),
        vec![a],
    );
    itertools::assert_equal(list.incomplete_tasks(), vec![]);
    itertools::assert_equal(list.complete_tasks(), vec![a]);
}

#[test]
fn snooze_incomplete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(
        list.snooze(a, Utc.ymd(2021, 05, 25).and_hms(14, 00, 00)),
        Ok(())
    );
}

#[test]
fn snooze_complete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a).unwrap();
    assert_eq!(
        list.snooze(a, Utc.ymd(2021, 05, 25).and_hms(15, 00, 00)),
        Err(vec![SnoozeWarning::TaskIsComplete])
    );
}

#[test]
fn snooze_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.snooze(b, Utc.ymd(2021, 05, 25).and_hms(15, 00, 00)),
        Ok(())
    );
    itertools::assert_equal(
        list.unsnooze_up_to(Utc.ymd(2021, 05, 25).and_hms(16, 00, 00))
            .iter_sorted(&list),
        vec![],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn snooze_task_until_after_due_date() {
    let mut list = TodoList::default();
    let due_date = Utc.ymd(2021, 05, 25).and_hms(20, 00, 00);
    let snooze = Utc.ymd(2021, 05, 26).and_hms(00, 00, 00);
    let a = list.add(NewOptions::new().desc("a").due_date(due_date));
    assert_eq!(
        list.snooze(a, snooze),
        Err(vec![SnoozeWarning::SnoozedUntilAfterDueDate {
            snoozed_until: snooze,
            due_date,
        }])
    );
}

#[test]
fn snooze_task_until_after_implicit_due_date() {
    let mut list = TodoList::default();
    let due_date = Utc.ymd(2021, 05, 25).and_hms(20, 00, 00);
    let snooze = Utc.ymd(2021, 05, 26).and_hms(00, 00, 00);
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").due_date(due_date));
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.snooze(a, snooze),
        Err(vec![SnoozeWarning::SnoozedUntilAfterDueDate {
            snoozed_until: snooze,
            due_date,
        }])
    );
}

#[test]
fn snoozed_blocked_task_remains_snoozed_when_deps_completed() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.snooze(b, Utc.ymd(2021, 05, 25).and_hms(16, 00, 00))
        .unwrap();
    itertools::assert_equal(
        list.check(CheckOptions {
            id: a,
            now: Utc.ymd(2021, 05, 25).and_hms(15, 00, 00),
        })
        .unwrap()
        .iter_sorted(&list),
        vec![],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn snoozed_blocked_task_unsnoozes_when_deps_completed_after_snooze_date() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.snooze(b, Utc.ymd(2021, 05, 25).and_hms(16, 00, 00))
        .unwrap();
    itertools::assert_equal(
        list.check(CheckOptions {
            id: a,
            now: Utc.ymd(2021, 05, 25).and_hms(17, 00, 00),
        })
        .unwrap()
        .iter_sorted(&list),
        vec![b],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
}

#[test]
fn unblock_does_not_unsnooze() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(Utc.ymd(2021, 05, 25).and_hms(00, 00, 00))
            .start_date(Utc.ymd(2021, 05, 26).and_hms(00, 00, 00)),
    );
    list.block(b).on(a).unwrap();
    list.unblock(b).from(a).unwrap();
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn remove_does_not_unsnooze() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(Utc.ymd(2021, 05, 25).and_hms(00, 00, 00))
            .start_date(Utc.ymd(2021, 05, 26).and_hms(00, 00, 00)),
    );
    list.block(b).on(a).unwrap();
    list.remove(a);
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn block_does_not_unsnooze() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(Utc.ymd(2021, 05, 25).and_hms(00, 00, 00))
            .start_date(Utc.ymd(2021, 05, 26).and_hms(00, 00, 00)),
    );
    list.check(a).unwrap();
    list.block(b).on(a).unwrap();
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn check_snoozed_task_should_unsnooze() {
    let mut list = TodoList::default();
    let today = Utc.ymd(2022, 02, 12).and_hms(00, 00, 00);
    let now = Utc.ymd(2022, 02, 12).and_hms(12, 00, 00);
    let tomorrow = Utc.ymd(2022, 02, 13).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(today)
            .start_date(tomorrow),
    );
    assert_eq!(list.get(a).unwrap().start_date, tomorrow);
    list.check(CheckOptions { id: a, now }).unwrap();
    assert_eq!(list.get(a).unwrap().start_date, today);
}

#[test]
fn unsnooze_task_that_is_not_snoozed() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.unsnooze(a), Err(vec![UnsnoozeWarning::NotSnoozed]));
}

#[test]
fn unsnooze_task_that_is_complete() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a).unwrap();
    assert_eq!(list.unsnooze(a), Err(vec![UnsnoozeWarning::TaskIsComplete]));
}

#[test]
fn unsnooze_task_that_is_snoozed() {
    let mut list = TodoList::default();
    let today = Utc.ymd(2022, 02, 12).and_hms(00, 00, 00);
    let tomorrow = Utc.ymd(2022, 02, 13).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(today)
            .start_date(tomorrow),
    );
    assert_eq!(list.unsnooze(a), Ok(()));
    assert_eq!(list.get(a).unwrap().start_date, today);
}

#[test]
fn unsnooze_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    assert_eq!(list.unsnooze(b), Err(vec![UnsnoozeWarning::TaskIsBlocked]));
}

#[test]
fn unsnooze_deeply_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    assert_eq!(list.unsnooze(c), Err(vec![UnsnoozeWarning::TaskIsBlocked]));
}
