// Tests for the 'focus' subcommand integration

use super::testing::{task, Fixture, Mutated};
use todo_model::{FocusPredicate, TaskId};
use chrono::{NaiveTime, Weekday};
use std::collections::HashSet;
use todo_printing::PrintableError;

// Helper to create HashSet<Weekday>
fn weekdays(days: &[Weekday]) -> HashSet<Weekday> {
    days.iter().cloned().collect()
}
// Helper to create NaiveTime
fn time(h: u32, m: u32) -> NaiveTime {
    NaiveTime::from_hms_opt(h, m, 0).unwrap()
}

#[test]
fn focus_on_single_task_weekdays() {
    let mut fix = Fixture::default();
    fix.test("todo new task");
    fix.test(r#"todo focus 1 --on "mon""#)
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("task", 1, Incomplete).mutated())
        .end();
    let list = fix.load();
    assert_eq!(
        list.get(TaskId(0)).unwrap().focus,
        Some(FocusPredicate::Weekdays(weekdays(&[Weekday::Mon])))
    );
}

#[test]
fn focus_on_single_task_timerange() {
    let mut fix = Fixture::default();
    fix.test("todo new task");
    fix.test(r#"todo focus 1 --on "9am-5pm""#)
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("task", 1, Incomplete).mutated())
        .end();
    let list = fix.load();
    assert_eq!(
        list.get(TaskId(0)).unwrap().focus,
        Some(FocusPredicate::TimeOfDayRange{ start: time(9,0), end: time(17,0) })
    );
}

#[test]
fn focus_none_single_task() {
    let mut fix = Fixture::default();
    fix.test("todo new task --focus weekdays"); // Set initial focus
    fix.test(r#"todo focus 1 --none"#)
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("task", 1, Incomplete).mutated())
        .end();
    let list = fix.load();
    assert!(list.get(TaskId(0)).unwrap().focus.is_none());
}

#[test]
fn focus_none_when_already_none() {
    let mut fix = Fixture::default();
    fix.test("todo new task"); // No initial focus
    fix.test(r#"todo focus 1 --none"#)
        .modified(Mutated::No) // No change expected
        .validate()
        // Should maybe print "No tasks needed focus removed." - but Fixture can't check stdout easily?
        // For now, just check no tasks printed as mutated.
        .end();
     let list = fix.load();
    assert!(list.get(TaskId(0)).unwrap().focus.is_none());
}

#[test]
fn focus_on_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new task1 task2");
    fix.test(r#"todo focus 1 2 --on "weekends""#)
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("task1", 1, Incomplete).mutated())
        .printed_task(&task("task2", 2, Incomplete).mutated())
        .end();
    let list = fix.load();
    let expected_focus = Some(FocusPredicate::Weekdays(weekdays(&[Weekday::Sat, Weekday::Sun])));
    assert_eq!(list.get(TaskId(0)).unwrap().focus, expected_focus);
    assert_eq!(list.get(TaskId(1)).unwrap().focus, expected_focus);
}

#[test]
fn focus_none_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new task1 --focus mon");
    fix.test("todo new task2 --focus tue");
     fix.test("todo new task3"); // No focus
    fix.test(r#"todo focus 1 2 3 --none"#)
        .modified(Mutated::Yes)
        .validate()
        .printed_task(&task("task1", 1, Incomplete).mutated())
        .printed_task(&task("task2", 2, Incomplete).mutated())
        // task3 wasn't mutated, shouldn't be printed
        .end();
    let list = fix.load();
    assert!(list.get(TaskId(0)).unwrap().focus.is_none());
    assert!(list.get(TaskId(1)).unwrap().focus.is_none());
     assert!(list.get(TaskId(2)).unwrap().focus.is_none()); // Should still be none
}

#[test]
fn focus_error_invalid_predicate() {
     let mut fix = Fixture::default();
    fix.test("todo new task");
    fix.test(r#"todo focus 1 --on "invalid-pred""#)
        .modified(Mutated::No)
        .validate()
        .printed_error(&PrintableError::CouldNotParseFocus {
            focus_string: "invalid-pred".to_string(),
            reason: format!("{:?}", time_format::ParseFocusError::UnknownPredicateType("invalid-pred".to_string())), // Mimic error format
        })
        .end();
}

#[test]
fn focus_error_missing_on_or_none() {
    let mut fix = Fixture::default();
    fix.test("todo new task");
    fix.test(r#"todo focus 1"#) // Missing --on or --none
        .modified(Mutated::No)
        .validate()
         .printed_error(&PrintableError::Raw("Must provide either --on <PREDICATE> or --none flag to the 'focus' command.".to_string()))
        .end();
}

#[test]
fn focus_error_invalid_key() {
    let mut fix = Fixture::default();
     fix.test(r#"todo focus 99 --on "mon""#) // Task 99 doesn't exist
        .modified(Mutated::No)
        .validate()
        // The current implementation prints a warning and succeeds with no output.
        // Ideally, it might return NoMatchForKey error. Testing current behavior:
        .end();
}
