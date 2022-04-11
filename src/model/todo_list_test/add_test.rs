use super::*;

#[test]
fn add_one_task() {
    let mut list = TodoList::default();
    let a = list.add("hello, world");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), None);
}

#[test]
fn add_multiple_tasks() {
    let mut list = TodoList::default();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}
