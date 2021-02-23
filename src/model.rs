use daggy::Dag;
use daggy::NodeIndex;
use daggy::Walker;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

pub type TaskId = usize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    graph: Dag<Task, (), TaskId>,
    complete_root: NodeIndex<TaskId>,
    incomplete_root: NodeIndex<TaskId>,
}

impl Task {
    pub fn new<S: Into<String>>(desc: S) -> Task {
        Task { desc: desc.into() }
    }
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
            .add_parent(self.incomplete_root, (), task)
            .1
            .index()
    }

    pub fn check(&mut self, id: TaskId) -> bool {
        self.graph
            .find_edge(NodeIndex::new(id), self.incomplete_root)
            .and_then(|edge| {
                self.graph.remove_edge(edge);
                self.graph
                    .update_edge(NodeIndex::new(id), self.complete_root, ())
                    .ok()
            })
            .is_some()
    }

    pub fn restore(&mut self, id: TaskId) -> bool {
        self.graph
            .find_edge(NodeIndex::new(id), self.complete_root)
            .and_then(|edge| {
                self.graph.remove_edge(edge);
                self.graph
                    .update_edge(NodeIndex::new(id), self.incomplete_root, ())
                    .ok()
            })
            .is_some()
    }

    pub fn get(&self, id: TaskId) -> Option<&Task> {
        self.graph.node_weight(NodeIndex::new(id))
    }

    pub fn get_number(&self, id: TaskId) -> Option<i32> {
        self.graph
            .parents(self.incomplete_root)
            .iter(&self.graph)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .position(|(_, n)| n.index() == id)
            .map(|pos| pos as i32 + 1)
            .or_else(|| {
                self.graph
                    .parents(self.complete_root)
                    .iter(&self.graph)
                    .position(|(_, n)| n.index() == id)
                    .map(|pos| -(pos as i32))
            })
    }

    pub fn lookup_by_number(&self, number: i32) -> Option<TaskId> {
        if number > 0 {
            self.graph
                .parents(self.incomplete_root)
                .iter(&self.graph)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .nth(number as usize - 1)
                .map(|(_, n)| n.index())
        } else {
            self.graph
                .parents(self.complete_root)
                .iter(&self.graph)
                .nth((-number) as usize)
                .map(|(_, n)| n.index())
        }
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.graph
            .parents(self.incomplete_root)
            .iter(&self.graph)
            .map(|(_, n)| n.index())
            // The children() method appears to iterate in the reverse order.
            // This means we need to collect the children and reverse-iterate.
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }

    pub fn complete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.graph
            .parents(self.complete_root)
            .iter(&self.graph)
            .map(|(_, n)| n.index())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
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
