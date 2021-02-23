use model::*;

#[test]
fn no_tasks() {
    let list = TodoList::new();
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), None);
}

#[test]
fn get_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}

#[test]
fn get_completed_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.check(a);
    list.check(b);
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}

#[test]
fn add_one_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("hello, world"));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), None);
}

#[test]
fn add_multiple_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_nonexistent_task() {
    let mut list = TodoList::new();
    assert!(!list.check(0));
}

#[test]
fn check_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    assert!(list.check(a));
    assert!(!list.check(a));
}

#[test]
fn check_first_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    assert!(list.check(a));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_second_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    assert!(list.check(b));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_third_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    assert!(list.check(c));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), None);
}

#[test]
fn complete_task_shows_up_in_complete_list() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a);
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
fn iterate_multiple_complete_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a);
    list.check(c);
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), Some(c));
    assert_eq!(complete_tasks.next(), None);
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

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
    let list = TodoList::new();
    reload(&list);
}

#[test]
fn reload_single_task() {
    let mut list = TodoList::new();
    list.add(Task::new("pass this test"));
    reload(&list);
}

#[test]
fn reload_three_tasks() {
    let mut list = TodoList::new();
    list.add(Task::new("first"));
    list.add(Task::new("second"));
    list.add(Task::new("third"));
    reload(&list);
}

#[test]
fn number_of_nonexistent_task() {
    let list = TodoList::new();
    assert_eq!(list.get_number(0), None);
}

#[test]
fn number_of_incomplete_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    assert_eq!(list.get_number(a), Some(1));
    assert_eq!(list.get_number(b), Some(2));
    assert_eq!(list.get_number(c), Some(3));
}

#[test]
fn number_of_complete_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a);
    list.check(b);
    list.check(c);
    assert_eq!(list.get_number(c), Some(0));
    assert_eq!(list.get_number(b), Some(-1));
    assert_eq!(list.get_number(a), Some(-2));
}

#[test]
fn number_of_task_updates_when_predecessor_completes() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a);
    assert_eq!(list.get_number(a), Some(0));
    assert_eq!(list.get_number(b), Some(1));
    assert_eq!(list.get_number(c), Some(2));
}

#[test]
fn existent_incomplete_task_by_number() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    assert_eq!(list.lookup_by_number(1), Some(a));
    assert_eq!(list.lookup_by_number(2), Some(b));
    assert_eq!(list.lookup_by_number(3), Some(c));
}

#[test]
fn nonexistent_incomplete_task_by_number() {
    let list = TodoList::new();
    assert_eq!(list.lookup_by_number(1), None);
}

#[test]
fn existent_complete_task_by_number() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a);
    list.check(b);
    list.check(c);
    assert_eq!(list.lookup_by_number(0), Some(c));
    assert_eq!(list.lookup_by_number(-1), Some(b));
    assert_eq!(list.lookup_by_number(-2), Some(a));
}

#[test]
fn nonexistent_complete_task_by_number() {
    let list = TodoList::new();
    assert_eq!(list.lookup_by_number(0), None);
}

#[test]
fn lookup_by_number_is_inverse_of_get_number() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.add(Task::new("d"));
    let e = list.add(Task::new("e"));
    list.check(a);
    list.check(c);
    list.check(e);
    for id in list.incomplete_tasks().chain(list.complete_tasks()) {
        let number = list.get_number(id).unwrap();
        let id_from_number = list.lookup_by_number(number).unwrap();
        assert_eq!(id_from_number, id);
    }
}

#[test]
fn restore_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    assert!(!list.restore(a));
    assert_eq!(list.get_number(a), Some(1));
}

#[test]
fn restore_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a);
    assert!(list.restore(a));
    assert_eq!(list.get_number(a), Some(1));
}

#[test]
fn restore_complete_task_to_nonempty_list() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a);
    assert!(list.restore(a));
    assert_eq!(list.get_number(b), Some(1));
    assert_eq!(list.get_number(c), Some(2));
    assert_eq!(list.get_number(a), Some(3));
}
