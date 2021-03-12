use chrono::DateTime;
use chrono::Utc;
use daggy::stable_dag::StableDag;
use daggy::NodeIndex;
use daggy::Walker;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::Read;
use std::io::Write;
use std::iter::FromIterator;

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
    tasks: StableDag<Task, ()>,
    complete: Vec<TaskId>,
    incomplete: Layering<TaskId>,
}

#[derive(Debug)]
pub struct TaskSet {
    ids: HashSet<TaskId>,
}

#[derive(PartialEq, Eq)]
struct TaskIdWithPosition {
    position: i32,
    id: TaskId,
}

impl PartialOrd for TaskIdWithPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl Ord for TaskIdWithPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position.cmp(&other.position)
    }
}

impl TaskSet {
    /// Iterates the set in an arbitrary order. Careful when using this; it may
    /// cause non-determinism. It is more efficient than iterating in sorted
    /// order.
    pub fn iter_unsorted(self) -> impl Iterator<Item = TaskId> {
        self.ids.into_iter()
    }

    /// Iterates the set in sorted order, where the ordering is defined by the
    /// position in the list.
    pub fn iter_sorted(self, list: &TodoList) -> impl Iterator<Item = TaskId> {
        self.ids
            .into_iter()
            .flat_map(|id| {
                list.position(id)
                    .map(|pos| TaskIdWithPosition {
                        id: id,
                        position: pos,
                    })
                    .into_iter()
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|task_id_with_pos| task_id_with_pos.id)
    }
}

impl FromIterator<TaskId> for TaskSet {
    fn from_iter<I: IntoIterator<Item = TaskId>>(iter: I) -> Self {
        Self {
            ids: iter.into_iter().collect(),
        }
    }
}

impl std::ops::BitOr for TaskSet {
    type Output = TaskSet;
    fn bitor(self, other: Self) -> Self::Output {
        TaskSet {
            ids: &self.ids | &other.ids,
        }
    }
}

impl TodoList {
    fn max_depth_of_deps(&self, id: TaskId) -> Option<usize> {
        self.deps(id)
            .iter_unsorted()
            .flat_map(|dep| {
                self.incomplete.depth.get(&dep).into_iter().copied()
            })
            .max()
    }

    fn update_depth(&mut self, id: TaskId) -> Option<usize> {
        match (
            self.incomplete.depth.get(&id).copied(),
            self.max_depth_of_deps(id).map(|depth| depth + 1),
        ) {
            // Task is complete, doesn't need to change
            (None, None) => None,
            // Task is complete, needs to be put into a layer.
            (None, Some(new_depth)) => {
                remove_first_occurrence_from_vec(&mut self.complete, &id);
                self.incomplete.put_in_layer(id, new_depth);
                Some(new_depth)
            }
            // Task is incomplete and has some incomplete deps.
            (Some(old_depth), Some(new_depth)) => {
                if old_depth == new_depth {
                    // If depth doesn't need to change, no-op.
                    None
                } else {
                    // Depth changed and adeps need to update.
                    self.incomplete.remove_from_layer(&id, old_depth);
                    self.incomplete.put_in_layer(id, new_depth);
                    Some(new_depth)
                }
            }
            // Task is incomplete, with no incomplete deps, so should go
            // to depth 0.
            (Some(old_depth), None) => {
                if old_depth == 0 {
                    None
                } else {
                    self.incomplete.remove_from_layer(&id, old_depth);
                    self.incomplete.put_in_layer(id, 0);
                    Some(0)
                }
            }
        }
        .map(|new_depth| {
            self.adeps(id).iter_sorted(&self).for_each(|adep| {
                self.update_depth(adep);
            });
            new_depth
        })
    }

    pub fn deps(&self, id: TaskId) -> TaskSet {
        self.tasks
            .parents(id.0)
            .iter(&self.tasks)
            .map(|(_, n)| TaskId(n))
            .collect()
    }

    pub fn adeps(&self, id: TaskId) -> TaskSet {
        self.tasks
            .children(id.0)
            .iter(&self.tasks)
            .map(|(_, n)| TaskId(n))
            .collect()
    }

    pub fn transitive_deps(&self, id: TaskId) -> TaskSet {
        self.deps(id)
            | self
                .deps(id)
                .iter_unsorted()
                .flat_map(|dep| self.transitive_deps(dep).iter_unsorted())
                .collect()
    }

    pub fn transitive_adeps(&self, id: TaskId) -> TaskSet {
        self.adeps(id)
            | self
                .adeps(id)
                .iter_unsorted()
                .flat_map(|adep| self.transitive_adeps(adep).iter_unsorted())
                .collect()
    }
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: StableDag::new(),
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
    pub fn check(&mut self, id: TaskId) -> Result<HashSet<TaskId>, CheckError> {
        if self.complete.contains(&id) {
            return Err(CheckError::TaskIsAlreadyComplete);
        }
        let deps = self.deps(id);
        let incomplete_deps: Vec<_> = deps
            .iter_sorted(&self)
            .filter(|dep| self.incomplete.contains(dep))
            .collect();
        if incomplete_deps.len() > 0 {
            return Err(CheckError::TaskIsBlockedBy(incomplete_deps));
        }
        self.tasks[id.0].completion_time = Some(Utc::now());
        self.incomplete.remove_from_layer(&id, 0);
        self.complete.push(id);
        // Update adeps.
        Ok(self
            .adeps(id)
            .iter_sorted(&self)
            .filter(|&adep| self.update_depth(adep) == Some(0))
            .collect())
    }
}

#[derive(Debug)]
pub enum RestoreError {
    TaskIsAlreadyIncomplete,
    WouldRestore(Vec<TaskId>),
}

impl TodoList {
    pub fn restore(
        &mut self,
        id: TaskId,
    ) -> Result<HashSet<TaskId>, RestoreError> {
        if !self.complete.contains(&id) {
            return Err(RestoreError::TaskIsAlreadyIncomplete);
        }
        let adeps = self.adeps(id);
        self.tasks[id.0].completion_time = None;
        self.incomplete.put_in_layer(id, 0);
        remove_first_occurrence_from_vec(&mut self.complete, &id);
        // Update adeps.
        Ok(adeps
            .iter_sorted(&self)
            .filter(|&adep| self.update_depth(adep) == Some(0))
            .collect())
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
    WouldBlockOnSelf,
    WouldCycle(daggy::WouldCycle<()>),
}

impl From<daggy::WouldCycle<()>> for BlockError {
    fn from(err: daggy::WouldCycle<()>) -> Self {
        BlockError::WouldCycle(err)
    }
}

impl<'a> Block<'a> {
    pub fn on(self, blocking: TaskId) -> Result<(), BlockError> {
        if blocking == self.blocked {
            return Err(BlockError::WouldBlockOnSelf);
        }
        self.list
            .tasks
            .update_edge(blocking.0, self.blocked.0, ())?;
        self.list.update_depth(self.blocked);
        Ok(())
    }
}

pub struct Unblock<'a> {
    list: &'a mut TodoList,
    blocked: TaskId,
}

impl TodoList {
    pub fn unblock(&mut self, blocked: TaskId) -> Unblock {
        Unblock {
            list: self,
            blocked: blocked,
        }
    }
}

#[derive(Debug)]
pub enum UnblockError {
    WouldUnblockFromSelf,
    WasNotDirectlyBlocking,
}

impl<'a> Unblock<'a> {
    pub fn from(self, blocking: TaskId) -> Result<(), UnblockError> {
        if blocking == self.blocked {
            return Err(UnblockError::WouldUnblockFromSelf);
        }
        match self.list.tasks.find_edge(blocking.0, self.blocked.0) {
            Some(e) => self.list.tasks.remove_edge(e),
            None => return Err(UnblockError::WasNotDirectlyBlocking),
        };
        self.list.update_depth(self.blocked);
        Ok(())
    }
}

#[derive(Debug)]
pub enum PuntError {
    TaskIsComplete,
}

impl TodoList {
    pub fn punt(&mut self, id: TaskId) -> Result<(), PuntError> {
        match self.incomplete.depth.get(&id) {
            Some(&depth) => {
                self.incomplete.remove_from_layer(&id, depth);
                self.incomplete.put_in_layer(id, depth);
                Ok(())
            }
            None => Err(PuntError::TaskIsComplete),
        }
    }
}

impl TodoList {
    pub fn get(&self, id: TaskId) -> Option<&Task> {
        self.tasks.node_weight(id.0)
    }

    pub fn get_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.node_weight_mut(id.0)
    }

    pub fn position(&self, id: TaskId) -> Option<i32> {
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

    pub fn status(&self, id: TaskId) -> Option<TaskStatus> {
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

    pub fn all_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.complete.iter().copied().chain(self.incomplete_tasks())
    }
}

#[derive(Debug)]
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

pub fn load<R>(reader: R) -> Result<TodoList, LoadError>
where
    R: Read,
{
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

pub fn save<W>(writer: W, model: &TodoList) -> Result<(), SaveError>
where
    W: Write,
{
    Ok(serde_json::to_writer(writer, model)?)
}
