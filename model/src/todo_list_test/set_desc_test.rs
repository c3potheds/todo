use super::*;

#[test]
fn set_desc_existent() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert!(list.set_desc(a, "b"));
    assert_eq!(list.get(a).unwrap().desc, "b");
}

#[test]
fn set_desc_nonexistent() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.remove(a);
    assert!(!list.set_desc(a, "b"));
    assert_eq!(list.get(a), None);
}
