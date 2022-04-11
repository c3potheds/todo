use super::*;

#[test]
fn ordering_of_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.position(a), Some(1));
    assert_eq!(list.position(b), Some(2));
}

#[test]
fn blocked_task_appears_after_task_that_blocks_it() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(a).on(b).expect("Could not block a on b");
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
}

#[test]
fn cannot_block_blocking_task_on_task_it_blocks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(a).on(b).expect("Could not block a on b");
    assert!(list.block(b).on(a).is_err());
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
}

#[test]
fn cannot_block_on_self() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.block(a)
        .on(a)
        .expect_err("Shouldn't be able to block a task on itself.");
}

#[test]
fn chained_blocking() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(a).on(b).expect("Could not block a on b");
    list.block(b).on(c).expect("Could not block b on c");
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
}

#[test]
fn indirect_blocking_cycle() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    assert!(list.block(a).on(c).is_err());
    // Make sure the status is consistent.
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn block_returns_affected_tasks() {
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
}

#[test]
fn block_returns_affected_task_priority_update() {
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
}

#[test]
fn block_does_not_return_unaffected_task_priority_update() {
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
}

#[test]
fn block_blocked_task_on_other_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(b).on(c).expect("Could not block b on c");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn block_complete_task_on_previously_complete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.block(b).on(a).expect("Could not block b on a");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(b));
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
fn block_complete_task_on_later_complete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.block(a).on(b).expect("Could not block a on b");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), Some(b));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
fn block_complete_task_affects_complete_adeps() {
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
    itertools::assert_equal(res.iter_sorted(&list), vec![a, b, c]);
}

#[test]
fn block_blocked_task_on_task_with_higher_depth() {
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
    itertools::assert_equal(res.iter_sorted(&list), vec![c, e]);
    itertools::assert_equal(list.incomplete_tasks(), vec![a, d, b, c, e]);
}
