use super::*;

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
    itertools::assert_equal(adeps.iter_sorted(&list), vec![]);
}

#[test]
fn remove_task_updates_depth_of_adeps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    let adeps = list.remove(a);
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    itertools::assert_equal(adeps.iter_sorted(&list), vec![b]);
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
    itertools::assert_equal(list.all_tasks(), vec![a, c]);
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    itertools::assert_equal(adeps.iter_sorted(&list), vec![c]);
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
    itertools::assert_equal(list.all_tasks(), vec![a, b, d, e]);
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(d), Some(TaskStatus::Blocked));
    assert_eq!(list.status(e), Some(TaskStatus::Blocked));
    itertools::assert_equal(adeps.iter_sorted(&list), vec![d, e]);
}
