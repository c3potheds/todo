use super::*;

#[test]
fn restore_incomplete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert!(list.restore(a).is_err());
    assert_eq!(list.position(a), Some(1));
    Ok(())
}

#[test]
fn restore_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    list.restore(a)?;
    assert_eq!(list.position(a), Some(1));
    Ok(())
}

#[test]
fn restore_complete_task_to_nonempty_list() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a)?;
    list.restore(a)?;
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(c), Some(2));
    assert_eq!(list.position(a), Some(3));
    Ok(())
}

#[test]
fn cannot_restore_task_with_complete_adeps() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.check(a).unwrap();
    list.check(b).unwrap();
    assert_eq!(
        list.restore(a),
        Err(RestoreError::WouldRestore(TaskSet::of(b)))
    );
    Ok(())
}

#[test]
fn complete_task_becomes_blocked_if_dependency_is_force_restored() -> TestResult
{
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.check(a)?;
    list.check(b)?;
    list.force_restore(a)?;
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn complete_task_becomes_blocked_if_transitive_dependency_is_force_restored(
) -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    list.check(a)?;
    list.check(b)?;
    list.check(c)?;
    list.force_restore(a)?;
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn force_restore_returns_newly_blocked_tasks_on_success() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    list.check(a).unwrap();
    list.check(b).unwrap();
    list.check(c).unwrap();
    let result = list.force_restore(a).unwrap();
    itertools::assert_equal(result.restored.iter_sorted(&list), vec![a, b, c]);
    itertools::assert_equal(result.blocked.iter_sorted(&list), vec![b, c]);
    Ok(())
}

#[test]
fn force_restore_already_incomplete() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(
        list.force_restore(a),
        Err(RestoreError::TaskIsAlreadyIncomplete)
    );
    Ok(())
}
