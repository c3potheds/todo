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
    assert_eq!(list.get(a).desc, "a");
    assert_eq!(list.get(b).desc, "b");
}

#[test]
fn get_completed_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.check(a);
    list.check(b);
    assert_eq!(list.get(a).desc, "a");
    assert_eq!(list.get(b).desc, "b");
}

#[test]
fn add_one_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("hello, world"));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&a));
    assert_eq!(tasks.next(), None);
}

#[test]
fn add_multiple_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&a));
    assert_eq!(tasks.next(), Some(&b));
    assert_eq!(tasks.next(), Some(&c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_first_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    list.check(a);
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&b));
    assert_eq!(tasks.next(), Some(&c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_second_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    list.check(b);
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&a));
    assert_eq!(tasks.next(), Some(&c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_third_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    list.check(c);
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&a));
    assert_eq!(tasks.next(), Some(&b));
    assert_eq!(tasks.next(), None);
}

#[test]
fn complete_task_shows_up_in_complete_list() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a);
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(&a));
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
    assert_eq!(complete_tasks.next(), Some(&a));
    assert_eq!(complete_tasks.next(), Some(&c));
    assert_eq!(complete_tasks.next(), None);
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(&b));
    assert_eq!(incomplete_tasks.next(), None);
}

// Serializes, then deserializes the list, and makes sure the reserialized
// version is equal to the original.
fn reload(list: &TodoList) {
    let serialized = serde_json::to_string(&list).unwrap();
    let deserialized = serde_json::from_str::<TodoList>(&serialized).unwrap();
    assert_eq!(list, &deserialized);
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
fn empty_from_json() {
    let list = TodoList::new();
    let json =
        json!({"tasks": [], "incomplete_tasks": [], "complete_tasks": []});
    assert_eq!(serde_json::from_value::<TodoList>(json).unwrap(), list);
}

#[test]
fn single_task_from_json() {
    let mut list = TodoList::new();
    list.add(Task::new("check me out"));
    let json = json!({
        "tasks": [{"desc": "check me out"}],
        "incomplete_tasks": [0],
        "complete_tasks": [],
    });
    assert_eq!(serde_json::from_value::<TodoList>(json).unwrap(), list);
}

#[test]
fn three_tasks_from_json() {
    let mut list = TodoList::new();
    list.add(Task::new("three"));
    list.add(Task::new("blind"));
    list.add(Task::new("mice"));
    let json = json!({
        "tasks": [
            {"desc": "three"},
            {"desc": "blind"},
            {"desc": "mice"},
        ],
        "incomplete_tasks": [0, 1, 2],
        "complete_tasks": [],
    });
    assert_eq!(serde_json::from_value::<TodoList>(json).unwrap(), list);
}

#[test]
fn todo_list_parse_fails_from_empty_object() {
    let json = json!({});
    assert!(serde_json::from_value::<TodoList>(json).is_err());
}

#[test]
fn todo_list_parse_fails_missing_tasks_key() {
    let json = json!({"wrong_key": "hi"});
    assert!(serde_json::from_value::<TodoList>(json).is_err());
}

#[test]
fn todo_list_parse_fails_from_garbage() {
    assert!(serde_json::from_str::<TodoList>("garbage").is_err());
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
