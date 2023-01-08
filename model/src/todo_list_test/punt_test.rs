use super::*;

use ::pretty_assertions::assert_eq;

#[test]
fn punt_only_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.punt(a)?;
    assert_eq!(list.position(a), Some(1));
    Ok(())
}

#[test]
fn punt_first_of_three_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.punt(a)?;
    assert_eq!(list.incomplete_tasks().collect::<Vec<_>>(), [b, c, a]);
    Ok(())
}

#[test]
fn cannot_punt_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a)?;
    list.punt(a)
        .expect_err("Punting a complete task should be an error");
    Ok(())
}

#[test]
fn punt_blocked_task_moves_to_end_of_layer() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(a)?;
    list.punt(b)?;
    assert_eq!(list.incomplete_tasks().collect::<Vec<_>>(), [a, c, b]);
    Ok(())
}
