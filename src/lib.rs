#[macro_use]
extern crate json;

use json::JsonValue;
use std::convert::From;
use std::slice::Iter;

trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

type TaskId = usize;

#[derive(Debug, PartialEq)]
enum JsonType {
    Array,
    Dict,
    // Num,
    // Bool,
    // Text,
}

#[derive(Debug, PartialEq)]
enum FromJsonError {
    MissingField(String),
    WrongType(JsonType),
}

fn json_get_string(
    json: &JsonValue,
    field: &str,
) -> Result<String, FromJsonError> {
    match json[field].as_str() {
        Some(value) => Ok(value.to_string()),
        None => Err(FromJsonError::MissingField(field.to_string())),
    }
}

fn json_get_array<'a>(
    json: &'a JsonValue,
    field: &str,
) -> Result<Iter<'a, JsonValue>, FromJsonError> {
    if !json.is_object() {
        return Err(FromJsonError::WrongType(JsonType::Dict));
    }
    if !json.has_key(field) {
        return Err(FromJsonError::MissingField(field.to_string()));
    }
    if !json[field].is_array() {
        return Err(FromJsonError::WrongType(JsonType::Array));
    }
    Ok(json[field].members())
}

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub desc: String,
}

#[derive(Debug, PartialEq)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl Task {
    pub fn new(desc: String) -> Task {
        Task { desc: desc }
    }
}

impl<'a> From<&'a Task> for JsonValue {
    fn from(task: &'a Task) -> Self {
        object!("desc" => *task.desc)
    }
}

impl<'a> TryFrom<&'a JsonValue> for Task {
    type Error = FromJsonError;
    fn try_from(json: &'a JsonValue) -> Result<Self, Self::Error> {
        Ok(Self {
            desc: json_get_string(json, "desc")?,
        })
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: Task) -> TaskId {
        self.tasks.push(task);
        self.tasks.len() - 1
    }

    pub fn check(&mut self, id: TaskId) {
        self.tasks.remove(id);
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }
}

impl<'a> From<&'a TodoList> for JsonValue {
    fn from(list: &'a TodoList) -> Self {
        object!(
            "tasks" => (
                list.tasks.iter().map(JsonValue::from).collect::<Vec<_>>()
            ),
        )
    }
}

impl<'a> TryFrom<&'a JsonValue> for TodoList {
    type Error = FromJsonError;

    fn try_from(json: &'a JsonValue) -> Result<Self, Self::Error> {
        Ok(Self {
            tasks: json_get_array(json, "tasks")?
                .flat_map(Task::try_from)
                .collect(),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
            JsonValue::from(&list),
            object!(
                "tasks" => array![],
            )
        );
    }

    #[test]
    fn single_task_to_json() {
        let mut list = TodoList::new();
        list.add(Task::new("pass this test".to_string()));
        assert_eq!(
            JsonValue::from(&list),
            object!(
                "tasks" => array![
                    object!("desc" => "pass this test")
                ],
            )
        );
    }

    #[test]
    fn three_tasks_to_json() {
        let mut list = TodoList::new();
        list.add(Task::new("first".to_string()));
        list.add(Task::new("second".to_string()));
        list.add(Task::new("third".to_string()));
        assert_eq!(
            JsonValue::from(&list),
            object!(
                "tasks" => array![
                    object!("desc" => "first"),
                    object!("desc" => "second"),
                    object!("desc" => "third")
                ],
            )
        );
    }

    #[test]
    fn empty_from_json() {
        let list = TodoList::new();
        let json = object!(
            "tasks" => array!(),
        );
        assert_eq!(TodoList::try_from(&json).unwrap(), list);
    }

    #[test]
    fn single_task_from_json() {
        let mut list = TodoList::new();
        list.add(Task::new("check me out".to_string()));
        let json = object!(
            "tasks" => array!(
                object!("desc" => "check me out")
            ),
        );
        assert_eq!(TodoList::try_from(&json).unwrap(), list);
    }

    #[test]
    fn three_tasks_from_json() {
        let mut list = TodoList::new();
        list.add(Task::new("three".to_string()));
        list.add(Task::new("blind".to_string()));
        list.add(Task::new("mice".to_string()));
        let json = object!(
            "tasks" => array!(
                object!("desc" => "three"),
                object!("desc" => "blind"),
                object!("desc" => "mice")
            ),
        );
        assert_eq!(TodoList::try_from(&json).unwrap(), list);
    }

    #[test]
    fn todo_list_parse_fails_from_emtpy_object() {
        let json = object!();
        assert_eq!(
            TodoList::try_from(&json),
            Err(FromJsonError::MissingField("tasks".to_string()))
        );
    }

    #[test]
    fn todo_list_parse_fails_from_array() {
        let json = array![];
        assert_eq!(
            TodoList::try_from(&json),
            Err(FromJsonError::WrongType(JsonType::Dict))
        );
    }

    #[test]
    fn todo_list_parse_fails_missing_tasks_key() {
        let json = object!("wrong_key" => "hi");
        assert_eq!(
            TodoList::try_from(&json),
            Err(FromJsonError::MissingField("tasks".to_string()))
        );
    }
}
