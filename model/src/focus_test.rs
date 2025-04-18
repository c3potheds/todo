// Unit tests for focus-related logic in the model

use crate::*; // Import items from model crate root (lib.rs)
use chrono::{Duration, NaiveTime, TimeZone, Utc, Weekday};
use std::collections::HashSet;

// Helper to create HashSet<Weekday>
fn weekdays(days: &[Weekday]) -> HashSet<Weekday> {
    days.iter().cloned().collect()
}

// Helper to create NaiveTime
fn time(h: u32, m: u32) -> NaiveTime {
    NaiveTime::from_hms_opt(h, m, 0).unwrap()
}
fn time_s(h: u32, m: u32, s: u32) -> NaiveTime {
    NaiveTime::from_hms_opt(h, m, s).unwrap()
}

// Helper to create DateTime<Utc> for specific date/time
fn datetime(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(y, mo, d, h, mi, s).unwrap()
}

#[test]
fn test_check_predicate() {
    // Weekday checks
    let weekdays_only = FocusPredicate::Weekdays(weekdays(&[Weekday::Mon, Weekday::Wed, Weekday::Fri]));
    assert!(check_predicate(&weekdays_only, datetime(2023, 10, 30, 10, 0, 0))); // Monday
    assert!(!check_predicate(&weekdays_only, datetime(2023, 10, 31, 10, 0, 0))); // Tuesday
    assert!(check_predicate(&weekdays_only, datetime(2023, 11, 1, 10, 0, 0))); // Wednesday
    assert!(!check_predicate(&weekdays_only, datetime(2023, 11, 4, 10, 0, 0))); // Saturday

    // Time range checks (standard)
    let time_9_to_5 = FocusPredicate::TimeOfDayRange { start: time(9, 0), end: time(17, 0) };
    assert!(check_predicate(&time_9_to_5, datetime(2023, 10, 30, 9, 0, 0))); // Exactly 9:00
    assert!(check_predicate(&time_9_to_5, datetime(2023, 10, 30, 12, 0, 0))); // Noon
    assert!(!check_predicate(&time_9_to_5, datetime(2023, 10, 30, 17, 0, 0))); // Exactly 17:00 (exclusive end)
    assert!(!check_predicate(&time_9_to_5, datetime(2023, 10, 30, 8, 59, 59))); // Before start
    assert!(!check_predicate(&time_9_to_5, datetime(2023, 10, 30, 17, 0, 1))); // After end

    // Time range checks (wrap-around)
    let time_10pm_to_2am = FocusPredicate::TimeOfDayRange { start: time(22, 0), end: time(2, 0) };
    assert!(check_predicate(&time_10pm_to_2am, datetime(2023, 10, 30, 22, 0, 0))); // Exactly 10pm
    assert!(check_predicate(&time_10pm_to_2am, datetime(2023, 10, 30, 23, 59, 59))); // Before midnight
    assert!(check_predicate(&time_10pm_to_2am, datetime(2023, 10, 31, 0, 0, 0))); // Midnight
    assert!(check_predicate(&time_10pm_to_2am, datetime(2023, 10, 31, 1, 59, 59))); // Before 2am
    assert!(!check_predicate(&time_10pm_to_2am, datetime(2023, 10, 31, 2, 0, 0))); // Exactly 2am (exclusive end)
    assert!(!check_predicate(&time_10pm_to_2am, datetime(2023, 10, 30, 21, 59, 59))); // Before start
    assert!(!check_predicate(&time_10pm_to_2am, datetime(2023, 10, 31, 14, 0, 0))); // Outside wrap range

    // DateTime range checks
    let dt_range = FocusPredicate::DateTimeRange {
        start: datetime(2023, 11, 1, 9, 0, 0),
        end: datetime(2023, 11, 1, 17, 0, 0),
    };
    assert!(check_predicate(&dt_range, datetime(2023, 11, 1, 9, 0, 0))); // Start edge
    assert!(check_predicate(&dt_range, datetime(2023, 11, 1, 12, 0, 0))); // Middle
    assert!(!check_predicate(&dt_range, datetime(2023, 11, 1, 17, 0, 0))); // End edge (exclusive)
    assert!(!check_predicate(&dt_range, datetime(2023, 11, 1, 8, 59, 59))); // Before start
    assert!(!check_predicate(&dt_range, datetime(2023, 10, 31, 12, 0, 0))); // Different day
}

 #[test]
 fn test_set_focus() {
     let mut list = TodoList::default();
     let task_id = list.add("Task 1");

     // 1. Initially, focus should be None
     assert!(list.get(task_id).unwrap().focus.is_none());

     // 2. Set a focus predicate (Weekdays)
     let weekdays_pred = FocusPredicate::Weekdays(weekdays(&[Weekday::Mon]));
     let affected1 = list.set_focus(task_id, Some(weekdays_pred.clone()));
     assert_eq!(affected1, TaskSet::of(task_id)); // Should return the affected task ID
     assert_eq!(list.get(task_id).unwrap().focus, Some(weekdays_pred.clone()));

     // 3. Set a different focus predicate (TimeRange)
     let time_pred = FocusPredicate::TimeOfDayRange { start: time(9, 0), end: time(17, 0) };
     let affected2 = list.set_focus(task_id, Some(time_pred.clone()));
     assert_eq!(affected2, TaskSet::of(task_id));
     assert_eq!(list.get(task_id).unwrap().focus, Some(time_pred.clone()));

    // 4. Set the same focus predicate again (should not report change)
     let affected3 = list.set_focus(task_id, Some(time_pred.clone()));
     assert!(affected3.is_empty()); // No change, so empty TaskSet
     assert_eq!(list.get(task_id).unwrap().focus, Some(time_pred));


     // 5. Remove focus
     let affected4 = list.set_focus(task_id, None);
     assert_eq!(affected4, TaskSet::of(task_id));
     assert!(list.get(task_id).unwrap().focus.is_none());

     // 6. Remove focus again (should not report change)
      let affected5 = list.set_focus(task_id, None);
      assert!(affected5.is_empty()); // No change
      assert!(list.get(task_id).unwrap().focus.is_none());

     // 7. Set focus on non-existent task (should do nothing)
     let invalid_task_id = TaskId::new(); // Assuming TaskId::new() gives an ID not in list
     let affected6 = list.set_focus(invalid_task_id, Some(weekdays_pred));
     assert!(affected6.is_empty());

 }

 #[test]
 fn test_is_effectively_in_focus() {
     let mut list = TodoList::default();

     // === Basic Cases ===
     let task_no_focus = list.add("No focus");
     let task_weekdays = list.add("Weekdays focus");
     let task_time = list.add("Time focus");
     let task_never = list.add("Never focus"); // Time range that never matches

     list.set_focus(task_weekdays, Some(FocusPredicate::Weekdays(weekdays(&[Weekday::Mon]))));
     list.set_focus(task_time, Some(FocusPredicate::TimeOfDayRange { start: time(9,0), end: time(17,0) }));
     list.set_focus(task_never, Some(FocusPredicate::TimeOfDayRange { start: time(1,0), end: time(1,0) })); // Start == End -> never matches

     let monday_noon = datetime(2023, 10, 30, 12, 0, 0); // Monday
     let tuesday_noon = datetime(2023, 10, 31, 12, 0, 0); // Tuesday
     let monday_8am = datetime(2023, 10, 30, 8, 0, 0); // Monday outside time range
     let monday_6pm = datetime(2023, 10, 30, 18, 0, 0); // Monday outside time range

     assert!(list.is_effectively_in_focus(task_no_focus, monday_noon)); // No focus is always true (base case)
     assert!(list.is_effectively_in_focus(task_weekdays, monday_noon)); // Matches weekday
     assert!(!list.is_effectively_in_focus(task_weekdays, tuesday_noon)); // Doesn't match weekday
     assert!(list.is_effectively_in_focus(task_time, monday_noon));    // Matches time
     assert!(!list.is_effectively_in_focus(task_time, monday_8am));    // Doesn't match time (before)
     assert!(!list.is_effectively_in_focus(task_time, monday_6pm));    // Doesn't match time (after)
     assert!(!list.is_effectively_in_focus(task_never, monday_noon));   // Never in focus

     // === Inheritance Cases ===
     let a = list.add("A"); // Focus: Mon
     let b = list.add("B"); // Focus: 9-5
     let c = list.add("C"); // No focus
     let d = list.add("D"); // Focus: Weekdays

     list.set_focus(a, Some(FocusPredicate::Weekdays(weekdays(&[Weekday::Mon]))));
     list.set_focus(b, Some(FocusPredicate::TimeOfDayRange { start: time(9,0), end: time(17,0) }));
     list.set_focus(d, Some(FocusPredicate::Weekdays(weekdays(&[Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri]))));


     // Chain: a -> b -> c -> d
     list.block(b).on(a).unwrap();
     list.block(c).on(b).unwrap();
     list.block(d).on(c).unwrap();

     // Test C (no focus itself, depends on A and B)
     assert!(list.is_effectively_in_focus(c, monday_noon));   // A=Mon(ok), B=9-5(ok) -> C is ok
     assert!(!list.is_effectively_in_focus(c, tuesday_noon));  // A=Mon(fail) -> C fails
     assert!(!list.is_effectively_in_focus(c, monday_8am));   // B=9-5(fail) -> C fails

     // Test D (focus weekdays, depends on A, B, C)
     assert!(list.is_effectively_in_focus(d, monday_noon));   // A=ok, B=ok, C=ok, D=weekday(ok) -> D is ok
     assert!(!list.is_effectively_in_focus(d, tuesday_noon));  // A=fail -> D fails
     assert!(!list.is_effectively_in_focus(d, monday_8am));   // B=fail -> D fails
     // Let's check Saturday - D's own predicate fails
     let saturday_noon = datetime(2023, 11, 4, 12, 0, 0);
     // Even if A and B were in focus (which they aren't necessarily on Sat), D's own predicate fails
     assert!(!list.is_effectively_in_focus(d, saturday_noon)); // D=weekday(fail) -> D fails

     // Conflicting focus (A=Mon, C=Tue) -> C should never be in focus
     let mut list2 = TodoList::default();
     let a2 = list2.add("A2");
     let c2 = list2.add("C2");
     list2.set_focus(a2, Some(FocusPredicate::Weekdays(weekdays(&[Weekday::Mon]))));
     list2.set_focus(c2, Some(FocusPredicate::Weekdays(weekdays(&[Weekday::Tue]))));
     list2.block(c2).on(a2).unwrap();

     assert!(!list2.is_effectively_in_focus(c2, monday_noon)); // C2=Tue(fail)
     assert!(!list2.is_effectively_in_focus(c2, tuesday_noon)); // A2=Mon(fail)

 }
