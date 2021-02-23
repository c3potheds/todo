use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

pub type TaskId = usize;

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

    pub fn restore(&mut self, id: TaskId) -> bool {
        if !self.complete_tasks.contains(&id) {
            return false;
        }
        self.complete_tasks.retain(|x| x != &id);
        self.incomplete_tasks.push(id);
        true
    }

    pub fn get(&self, id: TaskId) -> &Task {
        return &self.tasks[id];
    }

    pub fn get_number(&self, id: TaskId) -> Option<i32> {
        return self
            .incomplete_tasks
            .iter()
            .position(|&item| item == id)
            .map(|pos| (pos + 1) as i32)
            .or_else(|| {
                self.complete_tasks
                    .iter()
                    .position(|&item| item == id)
                    .map(|pos| 1 - ((self.complete_tasks.len() - pos) as i32))
            });
    }

    pub fn lookup_by_number(&self, number: i32) -> Option<&TaskId> {
        if number > 0 {
            self.incomplete_tasks.get((number - 1) as usize)
        } else if self.complete_tasks.len() == 0 {
            None
        } else {
            self.complete_tasks
                .get(self.complete_tasks.len() - (-number) as usize - 1)
        }
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = &TaskId> {
        self.incomplete_tasks.iter()
    }

    pub fn complete_tasks(&self) -> impl DoubleEndedIterator<Item = &TaskId> {
        self.complete_tasks.iter()
    }
}

pub enum LoadError {
    IoError(std::io::Error),
    DeserializeError(serde_json::Error),
}

impl From<std::io::Error> for LoadError {
    fn from(src: std::io::Error) -> Self {
        Self::IoError(src)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(src: serde_json::Error) -> Self {
        Self::DeserializeError(src)
    }
}

pub fn load<P>(path: P) -> Result<TodoList, LoadError>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

#[derive(Debug)]
pub enum SaveError {
    IoError(std::io::Error),
    SerializeError(serde_json::Error),
}

impl From<std::io::Error> for SaveError {
    fn from(src: std::io::Error) -> Self {
        Self::IoError(src)
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(src: serde_json::Error) -> Self {
        Self::SerializeError(src)
    }
}

pub fn save<P>(path: P, model: &TodoList) -> Result<(), SaveError>
where
    P: AsRef<Path>,
{
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    Ok(serde_json::to_writer(writer, model)?)
}
