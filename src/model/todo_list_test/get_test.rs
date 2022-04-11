use super::*;

#[test]
fn get_incomplete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}

#[test]
fn get_completed_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}
