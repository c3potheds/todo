use super::*;

#[test]
fn number_of_incomplete_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    assert_eq!(list.position(a), Some(1));
    assert_eq!(list.position(b), Some(2));
    assert_eq!(list.position(c), Some(3));
    Ok(())
}

#[test]
fn number_of_complete_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a)?;
    list.check(b)?;
    list.check(c)?;
    assert_eq!(list.position(c), Some(0));
    assert_eq!(list.position(b), Some(-1));
    assert_eq!(list.position(a), Some(-2));
    Ok(())
}

#[test]
fn number_of_task_updates_when_predecessor_completes() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a)?;
    assert_eq!(list.position(a), Some(0));
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(c), Some(2));
    Ok(())
}
