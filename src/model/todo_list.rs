use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use daggy::stable_dag::StableDag;
use daggy::Walker;
use model::remove_first_occurrence_from_vec;
use model::Layering;
use model::NewOptions;
use model::Task;
use model::TaskId;
use model::DurationInSeconds;
use model::TaskSet;
use model::TaskStatus;
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TodoList {
    tasks: StableDag<Task, ()>,
    complete: Vec<TaskId>,
    incomplete: Layering<TaskId>,
}

impl TodoList {
    fn calculate_implicit_priority(&self, id: TaskId) -> i32 {
        self.get(id)
            .into_iter()
            .map(|task| task.priority)
            .chain(
                self.adeps(id)
                    .into_iter_unsorted()
                    .map(|adep| self.calculate_implicit_priority(adep)),
            )
            .max()
            .unwrap_or(0)
    }

    fn calculate_implicit_due_date(&self, id: TaskId) -> Option<DateTime<Utc>> {
        self.get(id)
            .into_iter()
            .flat_map(|task| task.due_date.into_iter())
            .chain(self.adeps(id).into_iter_unsorted().flat_map(|adep| {
                self.calculate_implicit_due_date(adep).map(|due_date| {
                    due_date
                        - Duration::seconds(
                            self.get(adep).unwrap().budget.0 as i64,
                        )
                })
            }))
            .min()
    }

    fn put_in_incomplete_layer(&mut self, id: TaskId, depth: usize) -> usize {
        let pos = self.incomplete.bisect_layer(&id, depth, |&a, &b| {
            use std::cmp::Ordering;
            let ta = self.get(a).unwrap();
            let tb = self.get(b).unwrap();
            ta.implicit_priority
                .cmp(&tb.implicit_priority)
                .then_with(|| {
                    match (ta.implicit_due_date, tb.implicit_due_date) {
                        // Put lower due dates first.
                        (Some(a_date), Some(b_date)) => b_date.cmp(&a_date),
                        // A task with a due date appears before a task without one.
                        (Some(_), None) => Ordering::Greater,
                        (None, Some(_)) => Ordering::Less,
                        (None, None) => Ordering::Equal,
                    }
                })
        });
        self.incomplete.put_in_layer(id, depth, pos);
        pos
    }

    fn max_depth_of_deps(&self, id: TaskId) -> Option<usize> {
        self.deps(id)
            .into_iter_unsorted()
            .flat_map(|dep| self.incomplete.depth(&dep).into_iter())
            .max()
    }

    /// Recalculates the depth by adding 1 to the max depth of the task's deps.
    /// Returns Some with the new depth if a change was made, None otherwise.
    fn update_depth(&mut self, id: TaskId) -> Option<usize> {
        match (
            self.incomplete.depth(&id),
            self.max_depth_of_deps(id).map(|depth| depth + 1),
        ) {
            // Task is complete, doesn't need to change
            (None, None) => None,
            // Task is complete, needs to be put into a layer.
            (None, Some(new_depth)) => {
                remove_first_occurrence_from_vec(&mut self.complete, &id);
                self.put_in_incomplete_layer(id, new_depth);
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
                    self.put_in_incomplete_layer(id, new_depth);
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
                    self.put_in_incomplete_layer(id, 0);
                    Some(0)
                }
            }
        }
        .map(|new_depth| {
            self.adeps(id).iter_sorted(self).for_each(|adep| {
                self.update_depth(adep);
            });
            new_depth
        })
    }

    fn should_keep_snoozed(
        &self,
        id: TaskId,
        now: Option<DateTime<Utc>>,
    ) -> bool {
        let task = self.get(id).unwrap();
        let start_date = task.start_date;
        let creation_time = task.creation_time;
        match self.incomplete.depth(&id) {
            Some(depth) => {
                depth == 1
                    && match now {
                        Some(now) => start_date > now,
                        None => start_date > creation_time,
                    }
            }
            None => true,
        }
    }

    // Returns a TaskSet of affected tasks.
    pub fn update_implicits(&mut self, id: TaskId) -> TaskSet {
        let mut changed = false;
        let (old_priority, old_due_date) = {
            let task = self.get(id).unwrap();
            (task.implicit_priority, task.implicit_due_date)
        };
        let new_priority = self.calculate_implicit_priority(id);
        let new_due_date = self.calculate_implicit_due_date(id);
        {
            if let Some(mut task) = self.tasks.node_weight_mut(id.0) {
                if old_priority != new_priority {
                    task.implicit_priority = new_priority;
                    changed = true;
                }
                if old_due_date != new_due_date {
                    task.implicit_due_date = new_due_date;
                    changed = true;
                }
            }
        }
        if !changed {
            return TaskSet::default();
        }
        self.punt(id).unwrap_or_default();
        self.deps(id)
            .iter_sorted(self)
            .flat_map(|dep| self.update_implicits(dep).into_iter_unsorted())
            .chain(std::iter::once(id))
            .collect()
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

    fn transitive_deps_impl(&self, visited: &mut HashSet<TaskId>, id: TaskId) {
        if !visited.insert(id) {
            return;
        }
        self.deps(id)
            .iter_sorted(self)
            .for_each(|dep| self.transitive_deps_impl(visited, dep));
    }

    pub fn transitive_deps(&self, id: TaskId) -> TaskSet {
        let mut visited = HashSet::new();
        self.transitive_deps_impl(&mut visited, id);
        visited.remove(&id);
        visited.into_iter().collect()
    }

    fn transitive_adeps_impl(&self, visited: &mut HashSet<TaskId>, id: TaskId) {
        if !visited.insert(id) {
            return;
        }
        self.adeps(id)
            .iter_sorted(self)
            .for_each(|dep| self.transitive_adeps_impl(visited, dep));
    }

    pub fn transitive_adeps(&self, id: TaskId) -> TaskSet {
        let mut visited = HashSet::new();
        self.transitive_adeps_impl(&mut visited, id);
        visited.remove(&id);
        visited.into_iter().collect()
    }

    pub fn implicit_priority(&self, id: TaskId) -> Option<i32> {
        self.get(id).map(|task| task.implicit_priority)
    }

    pub fn implicit_due_date(
        &self,
        id: TaskId,
    ) -> Option<Option<DateTime<Utc>>> {
        self.get(id).map(|task| task.implicit_due_date)
    }
}

impl TodoList {
    pub fn add<T: Into<NewOptions>>(&mut self, task: T) -> TaskId {
        let task = Task::new(task.into());
        let snooze = task.start_date > task.creation_time;
        let id = TaskId(self.tasks.add_node(task));
        self.put_in_incomplete_layer(id, if snooze { 1 } else { 0 });
        id
    }
}

#[derive(Debug, PartialEq)]
pub struct ForceChecked {
    pub completed: TaskSet,
    pub unblocked: TaskSet,
}

#[derive(Debug, PartialEq)]
pub enum CheckError {
    TaskIsAlreadyComplete,
    TaskIsBlockedBy(Vec<TaskId>),
}

#[derive(Clone, Copy)]
pub struct CheckOptions {
    pub id: TaskId,
    pub now: DateTime<Utc>,
}

impl From<TaskId> for CheckOptions {
    fn from(id: TaskId) -> Self {
        Self {
            id,
            now: Utc::now(),
        }
    }
}

impl TodoList {
    /// Marks the task with the given id as complete. If successful, returns a
    /// set of tasks that became unblocked, if any.
    pub fn check<Options: Into<CheckOptions>>(
        &mut self,
        options: Options,
    ) -> Result<TaskSet, CheckError> {
        let options = options.into();
        if self.complete.contains(&options.id) {
            return Err(CheckError::TaskIsAlreadyComplete);
        }
        let deps = self.deps(options.id);
        let incomplete_deps: Vec<_> = deps
            .iter_sorted(self)
            .filter(|dep| self.incomplete.contains(dep))
            .collect();
        if !incomplete_deps.is_empty() {
            return Err(CheckError::TaskIsBlockedBy(incomplete_deps));
        }
        self.tasks[options.id.0].completion_time = Some(options.now);
        // It's legal to complete a task that's snoozed, but reset the snoozed
        // date to the task's creation time.
        if self.tasks[options.id.0].start_date > options.now {
            self.tasks[options.id.0].start_date =
                self.tasks[options.id.0].creation_time;
        }
        if let Some(depth) = self.incomplete.depth(&options.id) {
            assert!(depth == 0 || depth == 1);
            self.incomplete.remove_from_layer(&options.id, depth);
            self.complete.push(options.id);
            // Update adeps.
            return Ok(self
                .adeps(options.id)
                .iter_sorted(self)
                // Do not update the depth of snoozed adeps if they should still
                // be snoozed and if the checked task was in layer 0 (i.e.
                // was itself unsnoozed).
                .filter(|&adep| {
                    !self.should_keep_snoozed(adep, Some(options.now))
                })
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|&adep| self.update_depth(adep) == Some(0))
                .collect());
        }
        panic!("Checked task didn't have a depth.");
    }

    pub fn force_check<Options: Into<CheckOptions>>(
        &mut self,
        options: Options,
    ) -> Result<ForceChecked, CheckError> {
        let options = options.into();
        let check_result = self.check(options.id);
        if let Err(CheckError::TaskIsBlockedBy(blocked_by)) = &check_result {
            let mut result = blocked_by.iter().copied().fold(
                ForceChecked {
                    completed: TaskSet::default(),
                    unblocked: TaskSet::default(),
                },
                |result, dep| match self.force_check(dep) {
                    Ok(ForceChecked {
                        completed,
                        unblocked,
                    }) => ForceChecked {
                        completed,
                        unblocked: result.unblocked | unblocked,
                    },
                    Err(CheckError::TaskIsAlreadyComplete) => result,
                    Err(CheckError::TaskIsBlockedBy(_)) => panic!(
                        "force_check() should never return TaskIsBlockedBy"
                    ),
                },
            );
            result.unblocked = &result.unblocked - &TaskSet::of(options.id);
            match self.check(options) {
                Ok(newly_unblocked) => Ok(ForceChecked {
                    completed: result.completed
                        | std::iter::once(options.id).collect(),
                    unblocked: result.unblocked | newly_unblocked,
                }),
                Err(_) => panic!(
                    "check() should always succeed once deps are checked"
                ),
            }
        } else {
            Ok(ForceChecked {
                completed: std::iter::once(options.id).collect(),
                unblocked: check_result?,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ForceRestored {
    pub restored: TaskSet,
    pub blocked: TaskSet,
}

#[derive(Debug, PartialEq)]
pub enum RestoreError {
    TaskIsAlreadyIncomplete,
    WouldRestore(TaskSet),
}

impl TodoList {
    /// Marks a complete task as incomplete. If successful, returns a set of
    /// tasks that become blocked, if any.
    pub fn restore(&mut self, id: TaskId) -> Result<TaskSet, RestoreError> {
        if !self.complete.contains(&id) {
            return Err(RestoreError::TaskIsAlreadyIncomplete);
        }
        let complete_adeps: TaskSet = self
            .adeps(id)
            .into_iter_unsorted()
            .filter(|adep| self.complete.contains(adep))
            .collect();
        if !complete_adeps.is_empty() {
            return Err(RestoreError::WouldRestore(complete_adeps));
        }
        self.tasks[id.0].completion_time = None;
        self.put_in_incomplete_layer(id, 0);
        remove_first_occurrence_from_vec(&mut self.complete, &id);
        // Update adeps.
        Ok(self
            .adeps(id)
            .iter_sorted(self)
            .filter(|&adep| self.update_depth(adep) == Some(1))
            .collect())
    }

    /// Marks a task as incomplete. If any transitive antidependencies are
    /// complete, they are also marked as incomplete. If the task is already
    /// incomplete, returns ResoreError::TaskIsAlreadyIncomplete, but never
    /// returns WouldRestore.
    pub fn force_restore(
        &mut self,
        id: TaskId,
    ) -> Result<ForceRestored, RestoreError> {
        let restore_result = self.restore(id);
        if let Err(RestoreError::WouldRestore(would_restore)) = &restore_result
        {
            let result = would_restore.iter_sorted(self).fold(
                ForceRestored {
                    restored: TaskSet::default(),
                    blocked: TaskSet::default(),
                },
                |result, adep| match self.force_restore(adep) {
                    Ok(ForceRestored { restored, blocked }) => ForceRestored {
                        restored: result.restored | restored,
                        blocked: result.blocked | blocked,
                    },
                    Err(RestoreError::TaskIsAlreadyIncomplete) => result,
                    Err(RestoreError::WouldRestore(_)) => panic!(
                        "force_restore() should never return WouldRestore"
                    ),
                },
            );
            match self.restore(id) {
                Ok(newly_blocked) => Ok(ForceRestored {
                    restored: result.restored | std::iter::once(id).collect(),
                    blocked: result.blocked | newly_blocked,
                }),
                Err(_) => panic!(concat!(
                    "restore() should always work after ",
                    "force-restoring all adeps"
                )),
            }
        } else {
            Ok(ForceRestored {
                restored: std::iter::once(id).collect(),
                blocked: restore_result?,
            })
        }
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
    pub fn on(self, blocking: TaskId) -> Result<TaskSet, BlockError> {
        if blocking == self.blocked {
            return Err(BlockError::WouldBlockOnSelf);
        }
        self.list
            .tasks
            .update_edge(blocking.0, self.blocked.0, ())?;
        if !self.list.should_keep_snoozed(self.blocked, None)
            || self.list.status(self.blocked) == Some(TaskStatus::Complete)
        {
            self.list.update_depth(self.blocked);
        }
        Ok(self.list.update_implicits(blocking)
            | TaskSet::of(self.blocked)
            | TaskSet::of(blocking))
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
            blocked,
        }
    }
}

#[derive(Debug)]
pub enum UnblockError {
    WouldUnblockFromSelf,
    WasNotDirectlyBlocking,
}

impl<'a> Unblock<'a> {
    pub fn from(self, blocking: TaskId) -> Result<TaskSet, UnblockError> {
        if blocking == self.blocked {
            return Err(UnblockError::WouldUnblockFromSelf);
        }
        match self.list.tasks.find_edge(blocking.0, self.blocked.0) {
            Some(e) => self.list.tasks.remove_edge(e),
            None => return Err(UnblockError::WasNotDirectlyBlocking),
        };
        if !self.list.should_keep_snoozed(self.blocked, None) {
            self.list.update_depth(self.blocked);
        }
        Ok(self.list.update_implicits(blocking)
            | TaskSet::of(self.blocked)
            | TaskSet::of(blocking))
    }
}

#[derive(Debug)]
pub enum PuntError {
    TaskIsComplete,
}

impl TodoList {
    pub fn punt(&mut self, id: TaskId) -> Result<(), PuntError> {
        match self.incomplete.depth(&id) {
            Some(depth) => {
                self.incomplete.remove_from_layer(&id, depth);
                self.put_in_incomplete_layer(id, depth);
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

    pub fn set_desc(&mut self, id: TaskId, desc: &str) -> bool {
        self.tasks
            .node_weight_mut(id.0)
            .map(|task| task.desc = desc.to_string())
            .is_some()
    }

    pub fn set_priority(&mut self, id: TaskId, priority: i32) -> TaskSet {
        match self.tasks.node_weight_mut(id.0) {
            Some(task) => {
                task.priority = priority;
                self.update_implicits(id)
            }
            None => TaskSet::default(),
        }
    }

    pub fn set_due_date(
        &mut self,
        id: TaskId,
        due_date: Option<DateTime<Utc>>,
    ) -> TaskSet {
        match self.tasks.node_weight_mut(id.0) {
            Some(task) => {
                task.due_date = due_date;
                self.update_implicits(id)
            }
            None => TaskSet::default(),
        }
    }

    pub fn set_budget<D>(&mut self, id: TaskId, budget: D) -> TaskSet
    where
        D: Into<DurationInSeconds>,
    {
        match self.tasks.node_weight_mut(id.0) {
            Some(task) => {
                task.budget = budget.into();
                self.deps(id)
                    .iter_sorted(self)
                    .flat_map(|dep| {
                        self.update_implicits(dep).into_iter_unsorted()
                    })
                    .chain(std::iter::once(id))
                    .collect()
            }
            None => TaskSet::default(),
        }
    }

    pub fn position(&self, id: TaskId) -> Option<i32> {
        self.incomplete
            .position(&id)
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
        match self.incomplete.depth(&id) {
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

    pub fn num_incomplete_tasks(&self) -> usize {
        self.incomplete.len()
    }

    pub fn num_complete_tasks(&self) -> usize {
        self.complete.len()
    }

    pub fn unsnooze_up_to(&mut self, now: DateTime<Utc>) -> TaskSet {
        self.incomplete_tasks()
            .filter(|&id| {
                self.get(id).unwrap().start_date <= now
                    && self.status(id).unwrap() == TaskStatus::Blocked
                    && self.max_depth_of_deps(id).is_none()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|id| {
                let old_depth = self.incomplete.depth(&id).unwrap();
                self.incomplete.remove_from_layer(&id, old_depth);
                self.put_in_incomplete_layer(id, 0);
                self.adeps(id).iter_sorted(self).for_each(|adep| {
                    self.update_depth(adep);
                });
                id
            })
            .collect()
    }

    /// Returns the antidependencies of the removed task. These antidependencies
    /// are automatically blocked on the dependencies of the removed task.
    pub fn remove(&mut self, id: TaskId) -> TaskSet {
        if self.incomplete.contains(&id) {
            self.incomplete
                .remove_from_layer(&id, self.incomplete.depth(&id).unwrap());
        } else if self.complete.contains(&id) {
            remove_first_occurrence_from_vec(&mut self.complete, &id);
        };
        // If a task is nestled between deps and adeps, maintain the structure
        // of the graph by blocking the adeps on each of the deps.
        // E.g. if we remove b from (a <- b <- c), then we get (a <- c).
        let deps = self.deps(id);
        let adeps = self.adeps(id);
        deps.product(&adeps, self).for_each(|(dep, adep)| {
            // It should not be possible to cause a cycle when blocking an
            // adep on a dep because there would already be a cycle if so.
            self.block(adep).on(dep).unwrap();
        });
        self.tasks.remove_node(id.0);
        adeps
            .iter_sorted(self)
            .filter(|&adep| !self.should_keep_snoozed(adep, None))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|adep| {
                self.update_depth(adep);
            });
        adeps
    }
}

#[derive(Debug, PartialEq)]
pub enum SnoozeWarning {
    TaskIsComplete,
    SnoozedUntilAfterDueDate {
        snoozed_until: DateTime<Utc>,
        due_date: DateTime<Utc>,
    },
}

impl TodoList {
    pub fn snooze(
        &mut self,
        id: TaskId,
        start_date: DateTime<Utc>,
    ) -> Result<(), Vec<SnoozeWarning>> {
        match self.incomplete.depth(&id) {
            Some(depth) => {
                if depth == 0 {
                    self.incomplete.remove_from_layer(&id, 0);
                    self.put_in_incomplete_layer(id, 1);
                }
                self.tasks.node_weight_mut(id.0).unwrap().start_date =
                    start_date;
                if let Some(due_date) = self.get(id).unwrap().implicit_due_date
                {
                    if start_date > due_date {
                        return Err(vec![
                            SnoozeWarning::SnoozedUntilAfterDueDate {
                                snoozed_until: start_date,
                                due_date,
                            },
                        ]);
                    }
                }
                Ok(())
            }
            None => Err(vec![SnoozeWarning::TaskIsComplete]),
        }
    }
}