use super::*;

fn all_tasks_ordered(list: &TodoList) -> Vec<TaskId> {
    list.complete_tasks()
        .chain(list.incomplete_tasks())
        .collect::<Vec<_>>()
}

// Serializes, then deserializes the list, and makes sure the reserialized
// version is equal to the original.
fn reload(list: &TodoList) {
    let serialized = serde_json::to_string(&list).unwrap();
    let deserialized = serde_json::from_str::<TodoList>(&serialized).unwrap();
    assert_eq!(all_tasks_ordered(list), all_tasks_ordered(&deserialized));
}

#[test]
fn reload_empty() {
    let list = TodoList::default();
    reload(&list);
}

#[test]
fn reload_single_task() {
    let mut list = TodoList::default();
    list.add("pass this test");
    reload(&list);
}

#[test]
fn reload_three_tasks() {
    let mut list = TodoList::default();
    list.add("first");
    list.add("second");
    list.add("third");
    reload(&list);
}
