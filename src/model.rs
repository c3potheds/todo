use chrono::DateTime;
use chrono::Utc;
use daggy::Dag;
use daggy::NodeIndex;
use daggy::Walker;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct TaskId(NodeIndex);

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

impl Task {
    pub fn new<S: Into<String>>(desc: S) -> Task {
        Task {
            desc: desc.into(),
            creation_time: Some(Utc::now()),
            completion_time: None,
        }
    }
}

fn remove_first_occurrence_from_vec<T: PartialEq>(
    vec: &mut Vec<T>,
    data: &T,
) -> Option<T> {
    vec.iter().position(|x| x == data).map(|i| vec.remove(i))
}

#[derive(Debug, Deserialize, Serialize)]
struct Layering<T: Copy + Eq + Hash> {
    layers: Vec<Vec<T>>,
    depth: HashMap<T, usize>,
}

impl<T> Layering<T>
where
    T: Copy + Eq + Hash,
{
    fn layer(&mut self, layer: usize) -> &mut Vec<T> {
        while self.layers.len() <= layer {
            self.layers.push(Vec::new());
        }
        &mut self.layers[layer]
    }

    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            depth: HashMap::new(),
        }
    }

    pub fn put_in_layer(&mut self, data: T, layer: usize) -> bool {
        self.layer(layer).push(data);
        self.depth.insert(data, layer);
        true
    }

    pub fn remove_from_layer(&mut self, data: &T, layer: usize) -> bool {
        remove_first_occurrence_from_vec(&mut self.layers[layer], data)
            .map(|_| self.depth.remove(data))
            .is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.layers.iter().flat_map(|layer| layer.iter())
    }

    pub fn contains(&self, data: &T) -> bool {
        self.depth.contains_key(data)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    tasks: Dag<Task, ()>,
    complete: Vec<TaskId>,
    incomplete: Layering<TaskId>,
}

impl TodoList {
    fn max_depth_of_dependencies(&self, id: TaskId) -> Option<usize> {
        self.dependencies(id)
            .into_iter()
            .flat_map(|dep| {
                self.incomplete.depth.get(&dep).into_iter().copied()
            })
            .max()
    }

    fn update_depth(&mut self, id: TaskId) {
        if match (
            self.incomplete.depth.get(&id).copied(),
            self.max_depth_of_dependencies(id).map(|depth| depth + 1),
        ) {
            // Task is complete, doesn't need to change
            (None, None) => false,
            // Task is complete, needs to be put into a layer.
            (None, Some(new_depth)) => {
                remove_first_occurrence_from_vec(&mut self.complete, &id);
                self.incomplete.put_in_layer(id, new_depth);
                true
            }
            // Task is incomplete and has some incomplete dependencies.
            (Some(old_depth), Some(new_depth)) => {
                if old_depth == new_depth {
                    // If depth doesn't need to change, no-op.
                    false
                } else {
                    // Depth changed and adeps need to update.
                    self.incomplete.remove_from_layer(&id, old_depth);
                    self.incomplete.put_in_layer(id, new_depth);
                    true
                }
            }
            // Task is incomplete, with no incomplete dependencies, so should go
            // to depth 0.
            (Some(old_depth), None) => {
                if old_depth == 0 {
                    false
                } else {
                    self.incomplete.remove_from_layer(&id, old_depth);
                    self.incomplete.put_in_layer(id, 0);
                    true
                }
            }
        } {
            self.antidependencies(id)
                .into_iter()
                .for_each(|adep| self.update_depth(adep));
        }
    }

    fn dependencies(&self, id: TaskId) -> Vec<TaskId> {
        self.tasks
            .parents(id.0)
            .iter(&self.tasks)
            .map(|(_, n)| TaskId(n))
            .collect()
    }

    fn antidependencies(&self, id: TaskId) -> Vec<TaskId> {
        self.tasks
            .children(id.0)
            .iter(&self.tasks)
            .map(|(_, n)| TaskId(n))
            .collect()
    }
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Dag::new(),
            complete: Vec::new(),
            incomplete: Layering::new(),
        }
    }

    pub fn add(&mut self, task: Task) -> TaskId {
        let id = TaskId(self.tasks.add_node(task));
        self.incomplete.put_in_layer(id, 0);
        id
    }
}

#[derive(Debug)]
pub enum CheckError {
    TaskIsAlreadyComplete,
    TaskIsBlockedBy(Vec<TaskId>),
}

impl TodoList {
    pub fn check(&mut self, id: TaskId) -> Result<(), CheckError> {
        if self.complete.contains(&id) {
            return Err(CheckError::TaskIsAlreadyComplete);
        }
        let deps = self.dependencies(id);
        let incomplete_deps: Vec<_> = deps
            .iter()
            .copied()
            .filter(|dep| self.incomplete.contains(dep))
            .collect();
        if incomplete_deps.len() > 0 {
            return Err(CheckError::TaskIsBlockedBy(incomplete_deps));
        }
        self.tasks[id.0].completion_time = Some(Utc::now());
        self.incomplete.remove_from_layer(&id, 0);
        self.complete.push(id);
        // Update antidependencies.
        self.antidependencies(id)
            .into_iter()
            .for_each(|adep| self.update_depth(adep));
        Ok(())
    }
}

#[derive(Debug)]
pub enum RestoreError {
    TaskIsAlreadyIncomplete,
    WouldRestore(Vec<TaskId>),
}

impl TodoList {
    pub fn restore(&mut self, id: TaskId) -> Result<(), RestoreError> {
        if !self.complete.contains(&id) {
            return Err(RestoreError::TaskIsAlreadyIncomplete);
        }
        let adeps = self.antidependencies(id);
        self.tasks[id.0].completion_time = None;
        self.incomplete.put_in_layer(id, 0);
        remove_first_occurrence_from_vec(&mut self.complete, &id);
        // Update antidependencies.
        adeps.into_iter().for_each(|adep| self.update_depth(adep));
        Ok(())
    }
}

pub struct Block<'a> {
    list: &'a mut TodoList,
    blocked: TaskId,
}

impl TodoList {
    pub fn block(&mut self, id: TaskId) -> Block {
        Block {
            list: self,
            blocked: id,
        }
    }
}

#[derive(Debug)]
pub enum BlockError {
    WouldCycle(daggy::WouldCycle<()>),
}

impl From<daggy::WouldCycle<()>> for BlockError {
    fn from(err: daggy::WouldCycle<()>) -> Self {
        BlockError::WouldCycle(err)
    }
}

impl<'a> Block<'a> {
    pub fn on(self, blocking: TaskId) -> Result<(), BlockError> {
        self.list
            .tasks
            .update_edge(blocking.0, self.blocked.0, ())?;
        self.list.update_depth(self.blocked);
        Ok(())
    }
}

impl TodoList {
    pub fn get(&self, id: TaskId) -> Option<&Task> {
        self.tasks.node_weight(id.0)
    }

    pub fn get_number(&self, id: TaskId) -> Option<i32> {
        self.incomplete
            .iter()
            .position(|&x| x == id)
            .map(|pos| (pos as i32) + 1)
            .or_else(|| {
                self.complete
                    .iter()
                    .rev()
                    .position(|&x| x == id)
                    .map(|pos| -(pos as i32))
            })
    }

    pub fn get_status(&self, id: TaskId) -> Option<TaskStatus> {
        if self.complete.contains(&id) {
            return Some(TaskStatus::Complete);
        }
        match self.incomplete.depth.get(&id) {
            Some(0) => Some(TaskStatus::Incomplete),
            Some(_) => Some(TaskStatus::Blocked),
            _ => None,
        }
    }

    pub fn lookup_by_number(&self, number: i32) -> Option<TaskId> {
        if number <= 0 {
            self.complete_tasks().nth(-(number) as usize)
        } else {
            self.incomplete_tasks().nth((number - 1) as usize)
        }
    }

    pub fn incomplete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.incomplete.iter().copied()
    }

    pub fn complete_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.complete.iter().copied().rev()
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
