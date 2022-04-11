use super::*;

#[test]
fn num_incomplete_tasks() {
    let mut list = TodoList::default();
    assert_eq!(list.num_incomplete_tasks(), 0);
    let a = list.add("a");
    assert_eq!(list.num_incomplete_tasks(), 1);
    list.check(a).unwrap();
    assert_eq!(list.num_incomplete_tasks(), 0);
}

#[test]
fn num_complete_tasks() {
    let mut list = TodoList::default();
    assert_eq!(list.num_complete_tasks(), 0);
    let a = list.add("a");
    assert_eq!(list.num_complete_tasks(), 0);
    list.check(a).unwrap();
    assert_eq!(list.num_complete_tasks(), 1);
}
