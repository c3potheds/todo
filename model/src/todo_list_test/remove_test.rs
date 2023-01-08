use super::*;

use ::pretty_assertions::assert_eq;

#[test]
fn remove_task_does_not_invalidate_task_ids() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let adeps = list.remove(a);
    assert_eq!(list.get(a), None);
    assert_eq!(list.status(a), None);
    assert_eq!(list.get(b).unwrap().desc, "b");
    assert_eq!(list.get(c).unwrap().desc, "c");
    assert_eq!(adeps.as_sorted_vec(&list), []);
}

#[test]
fn remove_task_updates_depth_of_adeps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    let adeps = list.remove(a);
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    assert_eq!(adeps.as_sorted_vec(&list), [b]);
}

#[test]
fn remove_task_attaches_deps_to_adeps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let adeps = list.remove(b);
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, c]);
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    assert_eq!(adeps.as_sorted_vec(&list), [c]);
}

#[test]
fn remove_task_attaches_all_deps_to_adeps() {
    #![allow(clippy::many_single_char_names)]
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    let e = list.add("e");
    list.block(c).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.block(d).on(c).unwrap();
    list.block(e).on(c).unwrap();
    let adeps = list.remove(c);
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, b, d, e]);
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(d), Some(TaskStatus::Blocked));
    assert_eq!(list.status(e), Some(TaskStatus::Blocked));
    assert_eq!(adeps.as_sorted_vec(&list), [d, e]);
}
