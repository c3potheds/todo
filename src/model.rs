type TaskId = usize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    pub desc: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TodoList {
    tasks: Vec<Task>,
    incomplete_tasks: Vec<TaskId>,
    complete_tasks: Vec<TaskId>,
}

impl Task {
    pub fn new<S: Into<String>>(desc: S) -> Task {
        Task { desc: desc.into() }
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
            incomplete_tasks: Vec::new(),
            complete_tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Task) -> TaskId {
        self.tasks.push(task);
        let id = self.tasks.len() - 1;
        self.incomplete_tasks.push(id);
        id
    }

    pub fn check(&mut self, id: TaskId) {
        self.complete_tasks.push(id);
        self.incomplete_tasks.retain(|x| x != &id);
    }

    pub fn get(&self, id: TaskId) -> &Task {
        return &self.tasks[id];
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = &TaskId> {
        self.incomplete_tasks.iter()
    }

    pub fn complete_tasks(&self) -> impl Iterator<Item = &TaskId> {
        self.complete_tasks.iter()
    }
}
