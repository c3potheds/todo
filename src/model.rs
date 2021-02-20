type TaskId = usize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    pub desc: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl Task {
    pub fn new(desc: String) -> Task {
        Task { desc: desc }
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
