use chrono::Utc;
use todo_testing::ymdhms;

use super::*;

#[test]
fn status_of_incomplete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
    Ok(())
}

#[test]
fn status_is_persisted_incomplete() -> TestResult {
    use serde_json;
    let mut list = TodoList::default();
    let a = list.add("a");

    // Get status to cache it
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));

    // Serialize and deserialize
    let json = serde_json::to_string(&list).unwrap();
    let list: TodoList = serde_json::from_str(&json).unwrap();

    // Status should still be cached
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
    Ok(())
}

#[test]
fn status_is_persisted_blocked() -> TestResult {
    use serde_json;
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;

    // Get status to cache it
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));

    // Serialize and deserialize
    let json = serde_json::to_string(&list).unwrap();
    let list: TodoList = serde_json::from_str(&json).unwrap();

    // Status should still be cached
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn status_is_persisted_snoozed() -> TestResult {
    use serde_json;
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(ymdhms(2024, 12, 15, 12, 00, 00)),
    );
    list.snooze(a, ymdhms(2024, 12, 15, 16, 00, 00)).unwrap(); // 4 hours later

    // Get status to cache it
    assert_eq!(list.status(a), Some(TaskStatus::Blocked));

    // Serialize and deserialize
    let json = serde_json::to_string(&list).unwrap();
    let list: TodoList = serde_json::from_str(&json).unwrap();

    // Status should still be cached
    assert_eq!(list.status(a), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn status_is_persisted_blocked_and_snoozed() -> TestResult {
    use serde_json;
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(ymdhms(2024, 12, 15, 12, 00, 00)),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(ymdhms(2024, 12, 15, 12, 00, 00)),
    );
    list.snooze(a, ymdhms(2024, 12, 15, 16, 00, 00)).unwrap(); // 4 hours later
    list.block(b).on(a)?;

    // Get status to cache it
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));

    // Serialize and deserialize
    let json = serde_json::to_string(&list).unwrap();
    let list: TodoList = serde_json::from_str(&json).unwrap();

    // Status should still be cached
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn status_is_persisted_complete() -> TestResult {
    use serde_json;
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;

    // Get status to cache it
    assert_eq!(list.status(a), Some(TaskStatus::Complete));

    // Serialize and deserialize
    let json = serde_json::to_string(&list).unwrap();
    let list: TodoList = serde_json::from_str(&json).unwrap();

    // Status should still be cached
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
    Ok(())
}

#[test]
fn status_of_snoozed_task_after_deps_completed() -> TestResult {
    use chrono::Duration;
    let mut list = TodoList::default();
    let now = Utc::now();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.snooze(b, now + Duration::hours(1)).unwrap();
    // Two hours later, complete a
    list.check(CheckOptions {
        id: a,
        now: now + Duration::hours(2),
    })?;
    // b should be unblocked since both the snooze and the dep are done
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    Ok(())
}

#[test]
fn status_of_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
    Ok(())
}

#[test]
fn status_of_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}

#[test]
fn task_becomes_blocked_if_dependency_is_restored() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.check(a)?;
    list.restore(a)?;
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    Ok(())
}
