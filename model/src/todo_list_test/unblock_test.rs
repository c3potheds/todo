use super::*;

#[test]
fn unlbock_task_from_self_is_error() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.unblock(a)
        .from(a)
        .expect_err("Unblocking a task from itself is nonsensical");
    Ok(())
}

#[test]
fn unblock_task_from_task_that_does_not_block_it() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.unblock(b)
        .from(a)
        .expect_err("Shouldn't be able to unblock b from a");
    Ok(())
}

#[test]
fn unblock_task_from_blocking_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.unblock(b).from(a)?;
    Ok(())
}

#[test]
fn unblock_task_from_indirectly_blocking_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    list.unblock(c)
        .from(a)
        .expect_err("Shouldn't be able to unblock c from a");
    Ok(())
}

#[test]
fn newly_unblocked_task_has_incomplete_status() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.unblock(b).from(a)?;
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    Ok(())
}

#[test]
fn unblocked_task_is_still_blocked_if_it_has_remaining_dependencies(
) -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a)?;
    list.block(c).on(b)?;
    list.unblock(c).from(a)?;
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    assert_eq!(list.position(c), Some(3));
    Ok(())
}

#[test]
fn partially_unblocked_task_moves_to_lowest_possible_layer() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a)?;
    list.block(c).on(a)?;
    list.block(c).on(b)?;
    list.block(d).on(b)?;
    list.unblock(c).from(b)?;
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(d));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn unblock_returns_affected_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    let affected = list
        .unblock(b)
        .from(a)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![a, b]);
    Ok(())
}

#[test]
fn unblock_returns_affected_tasks_priority_update() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.set_priority(c, 1);
    let affected = list
        .unblock(c)
        .from(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![c, a, b]);
    Ok(())
}

#[test]
fn unblock_does_not_return_unaffected_tasks_priority_update() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.set_priority(a, 1);
    list.set_priority(c, 1);
    let affected = list
        .unblock(c)
        .from(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![c, b]);
    Ok(())
}
