use super::*;
use ::pretty_assertions::assert_eq;

#[test]
fn set_desc_existent() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.set_desc(a, "b"), TaskSet::of(a));
    assert_eq!(list.get(a).unwrap().desc, "b");
}

#[test]
fn set_desc_nonexistent() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.remove(a);
    assert!(list.set_desc(a, "b").is_empty());
    assert_eq!(list.get(a), None);
}

#[test]
fn set_desc_of_tag_updates_deps() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let x = list.add("x");
    list.set_tag(x, true);
    list.block(x).on(a)?;
    list.block(x).on(b)?;
    assert_eq!(list.set_desc(x, "y").as_sorted_vec(&list), [a, b, x]);
    Ok(())
}
