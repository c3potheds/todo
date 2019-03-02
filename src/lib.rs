type TaskId = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub description: String,
}

pub struct TodoList {
    next_id: TaskId,
    tasks: Vec<(TaskId, Task)>,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task {
            description: description,
        }
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            next_id: 0,
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Task) -> TaskId {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push((id, task));
        id
    }

    pub fn check(&mut self, id: TaskId) {
        self.tasks.retain(|(task_id, _task)| *task_id != id);
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter().map(|(_id, task)| task)
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

}
