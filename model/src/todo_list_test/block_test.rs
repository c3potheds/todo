use ::pretty_assertions::assert_eq;

use super::*;

#[test]
fn ordering_of_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    assert_eq!(list.position(a), Some(1));
    assert_eq!(list.position(b), Some(2));
    Ok(())
}

#[test]
fn blocked_task_appears_after_task_that_blocks_it() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(a).on(b)?;
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
    Ok(())
}

#[test]
fn cannot_block_blocking_task_on_task_it_blocks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(a).on(b)?;
    assert!(list.block(b).on(a).is_err());
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
    Ok(())
}

#[test]
fn cannot_block_on_self() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.block(a)
        .on(a)
        .expect_err("Shouldn't be able to block a task on itself.");
    Ok(())
}

#[test]
fn chained_blocking() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(a).on(b)?;
    list.block(b).on(c)?;
    let mut incomplete_tasks = list.incomplete_tasks();
    let mut next = incomplete_tasks.next().unwrap();
    assert_eq!(list.get(next).unwrap().desc, "c");
    assert_eq!(list.status(next).unwrap(), TaskStatus::Incomplete);
    next = incomplete_tasks.next().unwrap();
    assert_eq!(list.get(next).unwrap().desc, "b");
    assert_eq!(list.status(next).unwrap(), TaskStatus::Blocked);
    next = incomplete_tasks.next().unwrap();
    assert_eq!(list.get(next).unwrap().desc, "a");
    assert_eq!(list.status(next).unwrap(), TaskStatus::Blocked);
    Ok(())
}

#[test]
fn indirect_blocking_cycle() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert!(list.block(a).on(c).is_err());
    // Make sure the status is consistent.
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn block_returns_affected_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    let affected = list
        .block(c)
        .on(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![b, c]);
    Ok(())
}

#[test]
fn block_returns_affected_task_priority_update() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.set_priority(c, 1);
    list.block(b).on(a).unwrap();
    let affected = list
        .block(c)
        .on(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![a, b, c]);
    Ok(())
}

#[test]
fn block_does_not_return_unaffected_task_priority_update() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.set_priority(a, 1);
    list.set_priority(c, 1);
    list.block(b).on(a).unwrap();
    let affected = list
        .block(c)
        .on(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![b, c]);
    Ok(())
}

#[test]
fn block_blocked_task_on_other_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(a)?;
    list.block(b).on(c)?;
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
    Ok(())
}

#[test]
fn block_complete_task_on_previously_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.check(a)?;
    list.check(b)?;
    list.block(b).on(a)?;
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(b));
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
    Ok(())
}

#[test]
fn block_complete_task_on_later_complete_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.check(a)?;
    list.check(b)?;
    list.block(a).on(b)?;
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), Some(b));
    assert_eq!(complete_tasks.next(), None);
    Ok(())
}

#[test]
fn block_complete_task_affects_complete_adeps() -> TestResult {
    // If 'b' blocks 'c' and both are complete, and we block 'b' on an
    // incomplete task 'a', then both 'b' and 'c' are implicitly incomplete and
    // should be in the set of affected tasks.
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    list.check(b).unwrap();
    list.check(c).unwrap();
    let res = list.block(b).on(a).unwrap();
    assert_eq!(res.as_sorted_vec(&list), [a, b, c]);
    Ok(())
}

#[test]
fn block_blocked_task_on_task_with_higher_depth() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    let e = list.add("e");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.block(e).on(d).unwrap();
    let res = list.block(e).on(c).unwrap();
    assert_eq!(res.as_sorted_vec(&list), [c, e]);
    assert_eq!(list.incomplete_tasks().collect::<Vec<_>>(), [a, d, b, c, e]);
    Ok(())
}
