use chrono::DateTime;
use chrono::Utc;
use daggy::Dag;
use daggy::NodeIndex;
use daggy::Walker;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

pub type TaskId = usize;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TaskStatus {
    Complete,
    Incomplete,
    Blocked,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    pub desc: String,
    pub creation_time: Option<DateTime<Utc>>,
    pub completion_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    graph: Dag<Task, (), TaskId>,
    complete_root: NodeIndex<TaskId>,
    incomplete_root: NodeIndex<TaskId>,
}

impl Task {
    pub fn new<S: Into<String>>(desc: S) -> Task {
        Task {
            desc: desc.into(),
            creation_time: Some(Utc::now()),
            completion_time: None,
        }
    }
}

fn reversed<I: Iterator>(iter: I) -> impl Iterator<Item = I::Item> {
    iter.collect::<Vec<_>>().into_iter().rev()
}

impl TodoList {
    pub fn new() -> TodoList {
        let mut graph = Dag::new();
        let complete_root = graph.add_node(Task::new(""));
        let incomplete_root = graph.add_node(Task::new(""));
        TodoList {
            graph: graph,
            complete_root: complete_root,
            incomplete_root: incomplete_root,
        }
    }

    pub fn add(&mut self, task: Task) -> TaskId {
        self.graph
            .add_child(self.incomplete_root, (), task)
            .1
            .index()
    }

    pub fn check(&mut self, id: TaskId) -> bool {
        self.graph
            .find_edge(self.incomplete_root, NodeIndex::new(id))
            .and_then(|edge| {
                self.graph[NodeIndex::new(id)].completion_time =
                    Some(Utc::now());
                self.graph.remove_edge(edge);
                self.graph
                    .update_edge(self.complete_root, NodeIndex::new(id), ())
                    .ok()
            })
            .is_some()
    }

    pub fn restore(&mut self, id: TaskId) -> bool {
        self.graph
            .find_edge(self.complete_root, NodeIndex::new(id))
            .and_then(|edge| {
                self.graph[NodeIndex::new(id)].completion_time = None;
                self.graph.remove_edge(edge);
                self.graph
                    .update_edge(self.incomplete_root, NodeIndex::new(id), ())
                    .ok()
            })
            .is_some()
    }

    pub fn get(&self, id: TaskId) -> Option<&Task> {
        self.graph.node_weight(NodeIndex::new(id))
    }

    pub fn get_number(&self, id: TaskId) -> Option<i32> {
        reversed(self.graph.children(self.incomplete_root).iter(&self.graph))
            .position(|(_, n)| n.index() == id)
            .map(|pos| pos as i32 + 1)
            .or_else(|| {
                self.graph
                    .children(self.complete_root)
                    .iter(&self.graph)
                    .position(|(_, n)| n.index() == id)
                    .map(|pos| -(pos as i32))
            })
    }

    pub fn get_status(&self, id: TaskId) -> Option<TaskStatus> {
        self.graph
            .find_edge(self.complete_root, NodeIndex::new(id))
            .map(|_| TaskStatus::Complete)
            .or_else(|| {
                self.graph
                    .find_edge(self.incomplete_root, NodeIndex::new(id))
                    .map(|_| TaskStatus::Incomplete)
            })
    }

    pub fn lookup_by_number(&self, number: i32) -> Option<TaskId> {
        if number > 0 {
            reversed(
                self.graph.children(self.incomplete_root).iter(&self.graph),
            )
            .nth(number as usize - 1)
            .map(|(_, n)| n.index())
        } else {
            self.graph
                .children(self.complete_root)
                .iter(&self.graph)
                .nth((-number) as usize)
                .map(|(_, n)| n.index())
        }
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        reversed(
            self.graph
                .children(self.incomplete_root)
                .iter(&self.graph)
                .map(|(_, n)| n.index()),
        )
    }

    pub fn complete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        reversed(
            self.graph
                .children(self.complete_root)
                .iter(&self.graph)
                .map(|(_, n)| n.index()),
        )
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
