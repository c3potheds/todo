#![allow(clippy::zero_prefixed_literal)]

use itertools::Itertools;

use {
    super::*,
    ::pretty_assertions::assert_eq,
    chrono::{TimeZone, Utc},
};

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
    let start_date = Utc.with_ymd_and_hms(2021, 06, 01, 00, 00, 00).unwrap();
    let a = list.add(NewOptions::new().desc("a").start_date(start_date));
    assert_eq!(list.get(a).unwrap().start_date, start_date);
}

#[test]
fn new_task_with_start_time_later_than_now_starts_out_snoozed() {
    let mut list = TodoList::default();
    let now = Utc.with_ymd_and_hms(2021, 05, 25, 09, 00, 00).unwrap();
    let start_date = Utc.with_ymd_and_hms(2021, 06, 01, 00, 00, 00).unwrap();
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
    let now = Utc.with_ymd_and_hms(2021, 05, 25, 09, 00, 00).unwrap();
    let start_date = Utc.with_ymd_and_hms(2021, 06, 01, 00, 00, 00).unwrap();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    let now = Utc.with_ymd_and_hms(2021, 05, 30, 09, 00, 00).unwrap();
    let unsnoozed = list.unsnooze_up_to(now);
    assert_eq!(unsnoozed.len(), 0);
    assert!(list.get(a).unwrap().is_snoozed());
}

#[test]
fn unsnooze_up_to_after_snooze_date() {
    let mut list = TodoList::default();
    let now = Utc.with_ymd_and_hms(2021, 05, 25, 09, 00, 00).unwrap();
    let start_date = Utc.with_ymd_and_hms(2021, 06, 01, 00, 00, 00).unwrap();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    let now = Utc.with_ymd_and_hms(2021, 06, 01, 09, 00, 00).unwrap();
    let unsnoozed = list.unsnooze_up_to(now);
    assert_eq!(unsnoozed.as_sorted_vec(&list), [a]);
    assert_eq!(list.status(a).unwrap(), TaskStatus::Incomplete);
}

#[test]
fn unsnooze_up_to_unsnoozes_multiple_tasks() {
    let mut list = TodoList::default();
    let now = Utc.with_ymd_and_hms(2021, 06, 01, 00, 00, 00).unwrap();
    let snooze_a = Utc.with_ymd_and_hms(2021, 06, 02, 00, 00, 00).unwrap();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    let snooze_b = Utc.with_ymd_and_hms(2021, 06, 03, 00, 00, 00).unwrap();
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(now)
            .start_date(snooze_b),
    );
    let snooze_c = Utc.with_ymd_and_hms(2021, 06, 04, 00, 00, 00).unwrap();
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .creation_time(now)
            .start_date(snooze_c),
    );
    let now = snooze_b;
    let unsnoozed = list.unsnooze_up_to(now);
    assert_eq!(unsnoozed.as_sorted_vec(&list), [a, b]);
    let now = snooze_c;
    let unsnoozed = list.unsnooze_up_to(now);
    assert_eq!(unsnoozed.as_sorted_vec(&list), [c]);
}

#[test]
fn unsnooze_updates_depth_of_adeps() -> TestResult {
    let mut list = TodoList::default();
    let now = Utc.with_ymd_and_hms(2021, 05, 25, 10, 00, 00).unwrap();
    let snooze_a = Utc.with_ymd_and_hms(2021, 05, 25, 11, 00, 00).unwrap();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    let b = list.add(NewOptions::new().desc("b").due_date(snooze_a));
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a)?;
    list.block(d).on(c)?;
    // c is first because it is unblocked an unsnoozed.
    // a is next because it's snoozed and otherwise unblocked.
    // b is blocked by a, and so appears in a deeper layer than a.
    // Likewise, d is blocked by c, and so appears in a deeper layer than c.
    // d shows up after b because b has a due date and d does not.
    assert_eq!(list.incomplete_tasks().collect::<Vec<_>>(), [c, a, b, d]);
    let now = Utc.with_ymd_and_hms(2021, 05, 25, 12, 00, 00).unwrap();
    assert_eq!(list.unsnooze_up_to(now).as_sorted_vec(&list), [a]);
    // a and b now appear before c and d, respectively, because they are in
    // the same layer, and have a due date which sorts them earlier than the
    // other tasks with no due date.
    assert_eq!(list.incomplete_tasks().collect::<Vec<_>>(), [c, a, b, d]);
    Ok(())
}

#[test]
fn check_snoozed_task() -> TestResult {
    let mut list = TodoList::default();
    let now = Utc.with_ymd_and_hms(2021, 05, 25, 13, 00, 00).unwrap();
    let snooze_a = Utc.with_ymd_and_hms(2021, 05, 25, 14, 00, 00).unwrap();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    assert_eq!(
        list.check(CheckOptions { id: a, now })?
            .as_sorted_vec(&list),
        []
    );
    assert_eq!(list.incomplete_tasks().collect::<Vec<_>>(), []);
    assert_eq!(list.complete_tasks().collect::<Vec<_>>(), [a]);
    Ok(())
}

#[test]
fn force_check_snoozed_task() -> TestResult {
    let mut list = TodoList::default();
    let now = Utc.with_ymd_and_hms(2021, 05, 25, 13, 00, 00).unwrap();
    let snooze_a = Utc.with_ymd_and_hms(2021, 05, 25, 14, 00, 00).unwrap();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    assert_eq!(
        list.force_check(CheckOptions { id: a, now })?
            .completed
            .as_sorted_vec(&list),
        [a],
    );
    assert_eq!(list.incomplete_tasks().collect::<Vec<_>>(), []);
    assert_eq!(list.complete_tasks().collect::<Vec<_>>(), [a]);
    Ok(())
}

#[test]
fn snooze_incomplete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(
        list.snooze(a, Utc.with_ymd_and_hms(2021, 05, 25, 14, 00, 00).unwrap()),
        Ok(())
    );
}

#[test]
fn snooze_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    assert_eq!(
        list.snooze(a, Utc.with_ymd_and_hms(2021, 05, 25, 15, 00, 00).unwrap()),
        Err(vec![SnoozeWarning::TaskIsComplete])
    );
    Ok(())
}

#[test]
fn snooze_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    assert_eq!(
        list.snooze(b, Utc.with_ymd_and_hms(2021, 05, 25, 15, 00, 00).unwrap()),
        Ok(())
    );
    assert_eq!(
        list.unsnooze_up_to(
            Utc.with_ymd_and_hms(2021, 05, 25, 16, 00, 00).unwrap(),
        )
        .as_sorted_vec(&list),
        [],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn snooze_task_until_after_due_date() {
    let mut list = TodoList::default();
    let due_date = Utc.with_ymd_and_hms(2021, 05, 25, 20, 00, 00).unwrap();
    let snooze = Utc.with_ymd_and_hms(2021, 05, 26, 00, 00, 00).unwrap();
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
fn snooze_task_until_after_implicit_due_date() -> TestResult {
    let mut list = TodoList::default();
    let due_date = Utc.with_ymd_and_hms(2021, 05, 25, 20, 00, 00).unwrap();
    let snooze = Utc.with_ymd_and_hms(2021, 05, 26, 00, 00, 00).unwrap();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").due_date(due_date));
    list.block(b).on(a)?;
    assert_eq!(
        list.snooze(a, snooze),
        Err(vec![SnoozeWarning::SnoozedUntilAfterDueDate {
            snoozed_until: snooze,
            due_date,
        }])
    );
    Ok(())
}

#[test]
fn snoozed_blocked_task_remains_snoozed_when_deps_completed() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.snooze(b, Utc.with_ymd_and_hms(2021, 05, 25, 16, 00, 00).unwrap())
        .unwrap();
    assert_eq!(
        list.check(CheckOptions {
            id: a,
            now: Utc.with_ymd_and_hms(2021, 05, 25, 15, 00, 00).unwrap(),
        })?
        .as_sorted_vec(&list),
        [],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn snoozed_blocked_task_unsnoozes_when_deps_completed_after_snooze_date(
) -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.snooze(b, Utc.with_ymd_and_hms(2021, 05, 25, 16, 00, 00).unwrap())
        .unwrap();
    assert_eq!(
        list.check(CheckOptions {
            id: a,
            now: Utc.with_ymd_and_hms(2021, 05, 25, 17, 00, 00).unwrap(),
        })?
        .as_sorted_vec(&list),
        [b],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    Ok(())
}

#[test]
fn unblock_does_not_unsnooze() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(
                Utc.with_ymd_and_hms(2021, 05, 25, 00, 00, 00).unwrap(),
            )
            .start_date(
                Utc.with_ymd_and_hms(2021, 05, 26, 00, 00, 00).unwrap(),
            ),
    );
    list.block(b).on(a)?;
    list.unblock(b).from(a)?;
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn remove_does_not_unsnooze() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(
                Utc.with_ymd_and_hms(2021, 05, 25, 00, 00, 00).unwrap(),
            )
            .start_date(
                Utc.with_ymd_and_hms(2021, 05, 26, 00, 00, 00).unwrap(),
            ),
    );
    list.block(b).on(a)?;
    list.remove(a);
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn block_does_not_unsnooze() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(
                Utc.with_ymd_and_hms(2021, 05, 25, 00, 00, 00).unwrap(),
            )
            .start_date(
                Utc.with_ymd_and_hms(2021, 05, 26, 00, 00, 00).unwrap(),
            ),
    );
    list.check(a)?;
    list.block(b).on(a)?;
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn check_snoozed_task_should_unsnooze() -> TestResult {
    let mut list = TodoList::default();
    let today = Utc.with_ymd_and_hms(2022, 02, 12, 00, 00, 00).unwrap();
    let now = Utc.with_ymd_and_hms(2022, 02, 12, 12, 00, 00).unwrap();
    let tomorrow = Utc.with_ymd_and_hms(2022, 02, 13, 00, 00, 00).unwrap();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(today)
            .start_date(tomorrow),
    );
    assert_eq!(list.get(a).unwrap().start_date, tomorrow);
    list.check(CheckOptions { id: a, now })?;
    assert_eq!(list.get(a).unwrap().start_date, today);
    Ok(())
}

#[test]
fn unsnooze_task_that_is_not_snoozed() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.unsnooze(a), Err(vec![UnsnoozeWarning::NotSnoozed]));
}

#[test]
fn unsnooze_task_that_is_complete() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    assert_eq!(list.unsnooze(a), Err(vec![UnsnoozeWarning::TaskIsComplete]));
    Ok(())
}

#[test]
fn unsnooze_task_that_is_snoozed() {
    let mut list = TodoList::default();
    let today = Utc.with_ymd_and_hms(2022, 02, 12, 00, 00, 00).unwrap();
    let tomorrow = Utc.with_ymd_and_hms(2022, 02, 13, 00, 00, 00).unwrap();
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
fn unsnooze_blocked_task_that_is_not_snoozed() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    assert_eq!(list.unsnooze(b), Err(vec![UnsnoozeWarning::NotSnoozed]));
    Ok(())
}

#[test]
fn unsnooze_deeply_blocked_task_that_is_not_snoozed() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.unsnooze(c), Err(vec![UnsnoozeWarning::NotSnoozed]));
    Ok(())
}

#[test]
fn snoozed_task_appears_before_task_it_blocks() -> TestResult {
    let mut list = TodoList::default();
    let now = Utc.with_ymd_and_hms(2023, 09, 30, 15, 00, 00).unwrap();
    let a = list.add(NewOptions::new().desc("a").creation_time(now));
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    let snooze = Utc.with_ymd_and_hms(2023, 10, 01, 00, 00, 00).unwrap();
    list.snooze(a, snooze).unwrap();
    assert_eq!(list.status(a), Some(TaskStatus::Blocked));
    assert_eq!(list.position(a), Some(2));
    assert_eq!(list.all_tasks().collect_vec(), [d, a, b, c]);
    Ok(())
}
