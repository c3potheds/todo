use chrono::DateTime;
use chrono::Utc;
use daggy::petgraph::visit::Topo;
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

pub struct Block<'a> {
    list: &'a mut TodoList,
    blocked: NodeIndex<TaskId>,
}

impl<'a> Block<'a> {
    pub fn on(self, blocking: TaskId) -> bool {
        self.list
            .graph
            .find_edge(self.list.incomplete_root, self.blocked)
            .and_then(|edge| self.list.graph.remove_edge(edge));
        self.list
            .graph
            .update_edge(NodeIndex::new(blocking), self.blocked, ())
            .is_ok()
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        let mut graph = Dag::new();
        let complete_root = graph.add_node(Task::new("__complete__"));
        let incomplete_root = graph.add_node(Task::new("__incomplete__"));
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
                // Set the completion time.
                self.graph[NodeIndex::new(id)].completion_time =
                    Some(Utc::now());
                // Remove the connection to the incomplete root.
                self.graph.remove_edge(edge);
                // Connect the checked node to the complete root.
                self.graph
                    .update_edge(self.complete_root, NodeIndex::new(id), ())
                    .unwrap();
                // Update tasks that are blocked by this task.
                self.graph
                    .children(NodeIndex::new(id))
                    .iter(&self.graph)
                    .map(|(_, n)| n)
                    .filter(|&n| {
                        // If every task that blocks this dependent task is
                        // complete, it should be updated.
                        self.graph.parents(n).iter(&self.graph).all(|(_, p)| {
                            self.graph
                                .find_edge(self.complete_root, p)
                                .is_some()
                        })
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|to_update| {
                        self.graph
                            .update_edge(self.incomplete_root, to_update, ())
                            .unwrap();
                    });
                Some(())
            })
            .is_some()
    }

    fn block_all_children(&mut self, id: TaskId) {
        self.graph
            .children(NodeIndex::new(id))
            .iter(&self.graph)
            .flat_map(|(_, n)| {
                self.graph
                    .find_edge(self.incomplete_root, n)
                    .into_iter()
                    .chain(
                        self.graph.find_edge(self.complete_root, n).into_iter(),
                    )
                    .map(move |e| (e, n))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(e, n)| {
                // Sever the connections to the incomplete and complete nodes.
                self.graph.remove_edge(e);
                // Recur on child.
                self.block_all_children(n.index());
            })
    }

    pub fn restore(&mut self, id: TaskId) -> bool {
        self.graph
            .find_edge(self.complete_root, NodeIndex::new(id))
            .and_then(|edge| {
                // Remove the connection to the complete root.
                self.graph.remove_edge(edge);
                // Connect the restored node to the incomplete root.
                self.graph
                    .update_edge(self.incomplete_root, NodeIndex::new(id), ())
                    .unwrap();
                // Update tasks that become blocked on this task.
                self.block_all_children(id);
                Some(())
            })
            .is_some()
    }

    pub fn block(&mut self, id: TaskId) -> Block {
        Block {
            list: self,
            blocked: NodeIndex::new(id),
        }
    }

    pub fn get(&self, id: TaskId) -> Option<&Task> {
        self.graph.node_weight(NodeIndex::new(id))
    }

    pub fn get_number(&self, id: TaskId) -> Option<i32> {
        self.incomplete_tasks()
            .position(|n| n == id)
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
            .or_else(|| {
                self.graph
                    .node_weight(NodeIndex::new(id))
                    .map(|_| TaskStatus::Blocked)
            })
    }

    pub fn lookup_by_number(&self, number: i32) -> Option<TaskId> {
        if number > 0 {
            self.incomplete_tasks().nth(number as usize - 1)
        } else {
            self.graph
                .children(self.complete_root)
                .iter(&self.graph)
                .nth((-number) as usize)
                .map(|(_, n)| n.index())
        }
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        let incomplete_root = self.incomplete_root;
        let complete_root = self.complete_root;
        Topo::new(&self.graph)
            .iter(&self.graph)
            .filter(move |&n| {
                // Do not include the root nodes or children of the complete
                // root, which are completed tasks.
                n != incomplete_root
                    && n != complete_root
                    && self.graph.find_edge(self.complete_root, n).is_none()
            })
            .map(|n| n.index())
    }

    pub fn complete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.graph
            .children(self.complete_root)
            .iter(&self.graph)
            .map(|(_, n)| n.index())
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
