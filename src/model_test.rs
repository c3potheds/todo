use model::*;

#[test]
fn no_tasks() {
    let list = TodoList::new();
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), None);
}

#[test]
fn add_one_task() {
    let mut list = TodoList::new();
    let task = Task::new("hello, world".to_string());
    list.add(task.clone());
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&task));
    assert_eq!(tasks.next(), None);
}

#[test]
fn add_multiple_tasks() {
    let mut list = TodoList::new();
    let t1 = Task::new("walk the dog".to_string());
    let t2 = Task::new("do the dishes".to_string());
    let t3 = Task::new("take out the trash".to_string());
    list.add(t1.clone());
    list.add(t2.clone());
    list.add(t3.clone());
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&t1));
    assert_eq!(tasks.next(), Some(&t2));
    assert_eq!(tasks.next(), Some(&t3));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_first_task() {
    let mut list = TodoList::new();
    let t1 = Task::new("walk the dog".to_string());
    let t2 = Task::new("do the dishes".to_string());
    let t3 = Task::new("take out the trash".to_string());
    let id1 = list.add(t1.clone());
    list.add(t2.clone());
    list.add(t3.clone());
    list.check(id1);
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&t2));
    assert_eq!(tasks.next(), Some(&t3));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_second_task() {
    let mut list = TodoList::new();
    let t1 = Task::new("walk the dog".to_string());
    let t2 = Task::new("do the dishes".to_string());
    let t3 = Task::new("take out the trash".to_string());
    list.add(t1.clone());
    let id2 = list.add(t2.clone());
    list.add(t3.clone());
    list.check(id2);
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&t1));
    assert_eq!(tasks.next(), Some(&t3));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_third_task() {
    let mut list = TodoList::new();
    let t1 = Task::new("walk the dog".to_string());
    let t2 = Task::new("do the dishes".to_string());
    let t3 = Task::new("take out the trash".to_string());
    list.add(t1.clone());
    list.add(t2.clone());
    let id3 = list.add(t3.clone());
    list.check(id3);
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(&t1));
    assert_eq!(tasks.next(), Some(&t2));
    assert_eq!(tasks.next(), None);
}

#[test]
fn empty_to_json() {
    let list = TodoList::new();
    assert_eq!(
        serde_json::to_string(&list).unwrap(),
        json!({"tasks": []}).to_string()
    );
}

#[test]
fn single_task_to_json() {
    let mut list = TodoList::new();
    list.add(Task::new("pass this test".to_string()));
    assert_eq!(
        serde_json::to_string(&list).unwrap(),
        json!({"tasks": [{"desc": "pass this test"}]}).to_string()
    );
}

#[test]
fn three_tasks_to_json() {
    let mut list = TodoList::new();
    list.add(Task::new("first".to_string()));
    list.add(Task::new("second".to_string()));
    list.add(Task::new("third".to_string()));
    assert_eq!(
        serde_json::to_string(&list).unwrap(),
        json!({"tasks": [
            {"desc": "first"},
            {"desc": "second"},
            {"desc": "third"},
        ]})
        .to_string()
    );
}

#[test]
fn empty_from_json() {
    let list = TodoList::new();
    let json = json!({"tasks": []});
    assert_eq!(
        serde_json::from_str::<TodoList>(&json.to_string()).unwrap(),
        list
    );
}

#[test]
fn single_task_from_json() {
    let mut list = TodoList::new();
    list.add(Task::new("check me out".to_string()));
    let json = json!({"tasks": [{"desc": "check me out"}]});
    assert_eq!(
        serde_json::from_str::<TodoList>(&json.to_string()).unwrap(),
        list
    );
}

#[test]
fn three_tasks_from_json() {
    let mut list = TodoList::new();
    list.add(Task::new("three".to_string()));
    list.add(Task::new("blind".to_string()));
    list.add(Task::new("mice".to_string()));
    let json = json!({"tasks": [
        {"desc": "three"},
        {"desc": "blind"},
        {"desc": "mice"},
    ]});
    assert_eq!(
        serde_json::from_str::<TodoList>(&json.to_string()).unwrap(),
        list
    );
}

#[test]
fn todo_list_parse_fails_from_empty_object() {
    let json = json!({});
    assert!(serde_json::from_str::<TodoList>(&json.to_string()).is_err());
}

#[test]
fn todo_list_parse_fails_missing_tasks_key() {
    let json = json!({"wrong_key": "hi"});
    assert!(serde_json::from_str::<TodoList>(&json.to_string()).is_err());
}

#[test]
fn todo_list_parse_fails_from_garbage() {
    assert!(serde_json::from_str::<TodoList>("garbage").is_err());
}
