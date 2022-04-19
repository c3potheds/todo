use super::*;

#[test]
fn no_tasks() {
    let list = TodoList::default();
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), None);
}
