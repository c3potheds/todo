use super::*;

#[test]
fn status_of_incomplete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
}

#[test]
fn status_of_complete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
}

#[test]
fn status_of_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn task_becomes_blocked_if_dependency_is_restored() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    list.restore(a).expect("Could not restore a");
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}
