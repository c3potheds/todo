use super::*;

#[test]
fn status_of_incomplete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
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
