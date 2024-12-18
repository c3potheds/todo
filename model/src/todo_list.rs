use std::borrow::Cow;
use std::collections::HashSet;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use daggy::stable_dag::StableDag;
use daggy::Walker;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;

use crate::layering::remove_first_occurrence_from_vec;
use crate::layering::Layering;
use crate::DurationInSeconds;
use crate::NewOptions;
use crate::Task;
use crate::TaskId;
use crate::TaskSet;
use crate::TaskStatus;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TodoList<'ser> {
    #[serde(borrow)]
    tasks: StableDag<Task<'ser>, ()>,
    complete: Vec<TaskId>,
    incomplete: Layering<TaskId>,
}

impl TodoList<'_> {
    fn calculate_implicit_priority(&self, id: TaskId) -> i32 {
        self.get(id)
            .into_iter()
            .map(|task| task.priority)
            .chain(
                self.adeps(id)
                    .into_iter_unsorted()
                    .map(|adep| self.get(adep).unwrap().implicit_priority),
            )
            .max()
            .unwrap_or(0)
    }

    fn calculate_implicit_due_date(&self, id: TaskId) -> Option<DateTime<Utc>> {
        self.get(id)
            .into_iter()
            .flat_map(|task| task.due_date.into_iter())
            .chain(self.adeps(id).into_iter_unsorted().flat_map(|adep| {
                self.get(adep)
                    .unwrap()
                    .implicit_due_date
                    .map(|due_date| {
                        due_date
                            - Duration::seconds(
                                self.get(adep).unwrap().budget.0 as i64,
                            )
                    })
                    .into_iter()
            }))
            .min()
    }

    fn calculate_implicit_tags(&self, id: TaskId) -> Vec<TaskId> {
        self.adeps(id)
            .iter_sorted(self)
            .map(|adep| {
                let mut tags = Vec::new();
                if let Some(adep_data) = self.get(adep) {
                    tags.extend(&adep_data.implicit_tags);
                    if adep_data.tag && !tags.contains(&adep) {
                        tags.push(adep);
                    }
                }
                tags
            })
            .fold(Vec::new(), |mut so_far, adep_tags| {
                for &tag in &adep_tags {
                    if !so_far.contains(&tag) {
                        so_far.push(tag);
                    }
                }
                so_far
            })
    }

    fn put_in_incomplete_layer(&mut self, id: TaskId, depth: usize) -> usize {
        let pos = self.incomplete.bisect_layer(&id, depth, |&a, &b| {
            use std::cmp::Ordering;
            let ta = self.get(a).unwrap();
            let tb = self.get(b).unwrap();
            let (a_is_snoozed, b_is_snoozed) = (
                ta.start_date > ta.creation_time,
                tb.start_date > tb.creation_time,
            );
            match (a_is_snoozed, b_is_snoozed) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => Ordering::Equal,
            }
            .then_with(|| ta.implicit_priority.cmp(&tb.implicit_priority))
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
            .then_with(|| {
                // If both tasks are snoozed, sort them by start date (the
                // time that they will be unsnoozed).
                if a_is_snoozed && b_is_snoozed {
                    tb.start_date.cmp(&ta.start_date)
                } else {
                    Ordering::Equal
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
    fn update_depth(
        &mut self,
        id: TaskId,
        now: Option<DateTime<Utc>>,
    ) -> Option<usize> {
        match (
            self.incomplete.depth(&id),
            self.max_depth_of_deps(id).map(|depth| depth + 1),
        ) {
            // Task is complete, put it back at the top of the complete layer.
            (None, None) => {
                remove_first_occurrence_from_vec(&mut self.complete, &id);
                self.complete.push(id);
                self.tasks[id.0].cached_status = Some(TaskStatus::Complete);
                None
            }
            // Task is complete, needs to be put into a layer.
            (None, Some(new_depth)) => {
                remove_first_occurrence_from_vec(&mut self.complete, &id);
                self.put_in_incomplete_layer(id, new_depth);
                // Set status to blocked since it has incomplete deps
                self.tasks[id.0].cached_status = Some(TaskStatus::Blocked);
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
                    // Set status to blocked since it has incomplete deps
                    self.tasks[id.0].cached_status = Some(TaskStatus::Blocked);
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
                    // Set status to blocked if still snoozed, incomplete otherwise
                    self.tasks[id.0].cached_status =
                        if self.should_keep_snoozed(id, now) {
                            Some(TaskStatus::Blocked)
                        } else {
                            Some(TaskStatus::Incomplete)
                        };
                    Some(0)
                }
            }
        }
        .inspect(|_| {
            self.adeps(id).iter_sorted(self).for_each(|adep| {
                self.update_depth(adep, None);
            });
        })
    }

    fn should_keep_snoozed(
        &self,
        id: TaskId,
        now: Option<DateTime<Utc>>,
    ) -> bool {
        let task = self.get(id).unwrap();
        match now {
            Some(now) => task.start_date > now,
            None => task.is_snoozed(),
        }
    }

    // Returns a TaskSet of affected tasks.
    fn update_implicits(&mut self, id: TaskId) -> TaskSet {
        let mut changed = false;
        let (old_priority, old_due_date, old_tags) = {
            let task = self.get(id).unwrap();
            (
                task.implicit_priority,
                task.implicit_due_date,
                task.implicit_tags.clone(),
            )
        };
        let new_priority = self.calculate_implicit_priority(id);
        let new_due_date = self.calculate_implicit_due_date(id);
        let new_tags = self.calculate_implicit_tags(id);
        {
            if let Some(task) = self.tasks.node_weight_mut(id.0) {
                if old_priority != new_priority {
                    task.implicit_priority = new_priority;
                    changed = true;
                }
                if old_due_date != new_due_date {
                    task.implicit_due_date = new_due_date;
                    changed = true;
                }
                if old_tags != new_tags {
                    task.implicit_tags = new_tags;
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

impl<'ser> TodoList<'ser> {
    pub fn add<T: Into<NewOptions<'ser>>>(&mut self, task: T) -> TaskId {
        let task = Task::new(task.into());
        let id = TaskId(self.tasks.add_node(task));
        self.put_in_incomplete_layer(id, 0);
        id
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ForceChecked {
    pub completed: TaskSet,
    pub unblocked: TaskSet,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CheckError {
    #[error("task is already complete")]
    TaskIsAlreadyComplete,
    #[error("task is blocked by incomplete deps")]
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

impl TodoList<'_> {
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
        let task = &mut self.tasks[options.id.0];
        task.completion_time = Some(options.now);
        // It's legal to complete a task that's snoozed, but reset the snoozed
        // date to the task's creation time.
        if task.start_date > options.now {
            task.start_date = task.creation_time;
        }
        task.cached_status = Some(TaskStatus::Complete);
        if let Some(depth) = self.incomplete.depth(&options.id) {
            assert!(depth == 0);
            self.incomplete.remove_from_layer(&options.id, depth);
            self.complete.push(options.id);
            // Update adeps.
            return Ok(self
                .adeps(options.id)
                .iter_sorted(self)
                // Update the depth of all adeps, and collect the ones that
                // become unblocked.
                .filter(|&adep| {
                    self.update_depth(adep, Some(options.now)) == Some(0)
                })
                .collect::<Vec<_>>()
                .into_iter()
                // Unsnooze any adeps that are snoozed whose start date is
                // before the current time.
                .inspect(|&adep| {
                    if !self.should_keep_snoozed(adep, Some(options.now)) {
                        self.tasks[adep.0].start_date =
                            self.tasks[adep.0].creation_time;
                    }
                })
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
            result.unblocked = result.unblocked - TaskSet::of(options.id);
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

#[derive(Debug, PartialEq, Eq)]
pub struct ForceRestored {
    pub restored: TaskSet,
    pub blocked: TaskSet,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum RestoreError {
    #[error("task is already incomplete")]
    TaskIsAlreadyIncomplete,
    #[error("restoring task would implicitly restore complete adeps")]
    WouldRestore(TaskSet),
}

impl TodoList<'_> {
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
        let task = &mut self.tasks[id.0];
        task.completion_time = None;
        task.cached_status = Some(TaskStatus::Incomplete);
        self.put_in_incomplete_layer(id, 0);
        remove_first_occurrence_from_vec(&mut self.complete, &id);
        // Update adeps.
        Ok(self
            .adeps(id)
            .iter_sorted(self)
            .filter(|&adep| self.update_depth(adep, None) == Some(1))
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

pub struct Block<'a, 'ser> {
    list: &'a mut TodoList<'ser>,
    blocked: TaskId,
}

impl<'ser> TodoList<'ser> {
    pub fn block(&mut self, id: TaskId) -> Block<'_, 'ser> {
        Block {
            list: self,
            blocked: id,
        }
    }
}

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("trying to block task on self")]
    WouldBlockOnSelf,
    #[error("would create a dependency cycle")]
    WouldCycle(daggy::WouldCycle<()>),
}

impl From<daggy::WouldCycle<()>> for BlockError {
    fn from(err: daggy::WouldCycle<()>) -> Self {
        BlockError::WouldCycle(err)
    }
}

fn get_complete_adeps(list: &TodoList, id: TaskId) -> TaskSet {
    list.adeps(id)
        .into_iter_unsorted()
        .filter(|&adep| list.status(adep) == Some(TaskStatus::Complete))
        .collect()
}

fn get_incomplete_adeps(list: &TodoList, id: TaskId) -> TaskSet {
    list.adeps(id)
        .into_iter_unsorted()
        .filter(|&adep| list.status(adep) != Some(TaskStatus::Complete))
        .collect()
}

impl Block<'_, '_> {
    fn update_depth_of_blocked_and_get_implicitly_restored_adeps(
        &mut self,
    ) -> TaskSet {
        // If the blocked task is snoozed, then just update its depth, and do
        // not bother looking for implicitly restored adeps because a snoozed
        // task can not have had complete adeps.
        let was_blocked_complete =
            self.list.status(self.blocked) == Some(TaskStatus::Complete);
        if !was_blocked_complete {
            self.list.update_depth(self.blocked, None);
            return TaskSet::default();
        }
        // Find the intersection of adeps that were complete before the
        // update and adeps that are not complete after the update.
        get_complete_adeps(self.list, self.blocked)
            & match self.list.update_depth(self.blocked, None) {
                Some(_) => get_incomplete_adeps(self.list, self.blocked),
                None => TaskSet::default(),
            }
    }

    pub fn on(mut self, blocking: TaskId) -> Result<TaskSet, BlockError> {
        if blocking == self.blocked {
            return Err(BlockError::WouldBlockOnSelf);
        }
        self.list
            .tasks
            .update_edge(blocking.0, self.blocked.0, ())?;
        let extra_affected =
            self.update_depth_of_blocked_and_get_implicitly_restored_adeps();
        Ok(extra_affected
            | self.list.update_implicits(blocking)
            | TaskSet::of(self.blocked)
            | TaskSet::of(blocking))
    }
}

pub struct Unblock<'a, 'ser> {
    list: &'a mut TodoList<'ser>,
    blocked: TaskId,
}

impl<'ser> TodoList<'ser> {
    pub fn unblock(&mut self, blocked: TaskId) -> Unblock<'_, 'ser> {
        Unblock {
            list: self,
            blocked,
        }
    }
}

#[derive(Debug, Error)]
pub enum UnblockError {
    #[error("trying to unblock task from self")]
    WouldUnblockFromSelf,
    #[error("was not directly blocking")]
    WasNotDirectlyBlocking,
}

impl Unblock<'_, '_> {
    pub fn from(self, blocking: TaskId) -> Result<TaskSet, UnblockError> {
        if blocking == self.blocked {
            return Err(UnblockError::WouldUnblockFromSelf);
        }
        match self.list.tasks.find_edge(blocking.0, self.blocked.0) {
            Some(e) => self.list.tasks.remove_edge(e),
            None => return Err(UnblockError::WasNotDirectlyBlocking),
        };
        self.list.update_depth(self.blocked, None);
        Ok(self.list.update_implicits(blocking)
            | TaskSet::of(self.blocked)
            | TaskSet::of(blocking))
    }
}

#[derive(Debug, Error)]
pub enum PuntError {
    #[error("cannot punt complete task")]
    TaskIsComplete,
}

impl TodoList<'_> {
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

impl<'ser> TodoList<'ser> {
    pub fn get(&self, id: TaskId) -> Option<&Task> {
        self.tasks.node_weight(id.0)
    }

    pub fn set_desc<S: Into<Cow<'ser, str>>>(
        &mut self,
        id: TaskId,
        desc: S,
    ) -> TaskSet {
        self.tasks
            .node_weight_mut(id.0)
            .map(|task| {
                task.desc = desc.into();
                task.tag
            })
            .map(|tag| {
                if tag {
                    self.deps(id).iter_sorted(self).fold(
                        TaskSet::of(id),
                        |so_far, dep| {
                            so_far
                                | self.update_implicits(dep)
                                | TaskSet::of(dep)
                        },
                    )
                } else {
                    TaskSet::of(id)
                }
            })
            .unwrap_or_default()
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

    pub fn set_tag(&mut self, id: TaskId, tag: bool) -> TaskSet {
        match self.tasks.node_weight_mut(id.0) {
            Some(task) => {
                if task.tag == tag {
                    return TaskSet::default();
                }
                task.tag = tag;
                self.deps(id)
                    .iter_sorted(self)
                    .fold(TaskSet::of(id), |so_far, dep| {
                        so_far | self.update_implicits(dep)
                    })
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
        let task = self.tasks.node_weight(id.0)?;
        if let Some(cached) = task.cached_status {
            return Some(cached);
        }
        if self.complete.contains(&id) {
            return Some(TaskStatus::Complete);
        }
        if task.is_snoozed() {
            return Some(TaskStatus::Blocked);
        }
        if self.incomplete.depth(&id)? == 0 {
            Some(TaskStatus::Incomplete)
        } else {
            Some(TaskStatus::Blocked)
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

    pub fn unsnooze_up_to(&mut self, now: DateTime<Utc>) -> TaskSet {
        self.incomplete_tasks()
            .filter(|&id| {
                let task = self.get(id).unwrap();
                task.start_date <= now && task.is_snoozed()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .inspect(|id| {
                self.unsnooze(*id).unwrap();
            })
            .collect()
    }

    /// Returns the antidependencies of the removed task. These antidependencies
    /// are automatically blocked on the dependencies of the removed task.
    pub fn remove(&mut self, id: TaskId) -> TaskSet {
        // Explicitly unblock the adeps from the removed task and the removed
        // task from its deps. Although removing the node from the graph is
        // sufficient to update the edges, it doesn't update the implicits of
        // the deps or adeps, and if we do not do that, we can get stale ids
        // in the list of implicit tags or stale values for priorities or due
        // dates. The worst consequence of such stale data can be task ids
        // of removed tasks getting re-used by new tasks later on and suddenly
        // adding tags to tasks that shouldn't have them.
        let deps = self.deps(id);
        let affected_after_unblocking_from_deps = deps
            .iter_sorted(self)
            .fold(TaskSet::default(), |so_far, dep| {
                so_far | self.unblock(id).from(dep).unwrap()
            });
        let adeps = self.adeps(id);
        let affected_after_unblocking_adeps = adeps
            .iter_sorted(self)
            .fold(TaskSet::default(), |so_far, adep| {
                so_far | self.unblock(adep).from(id).unwrap()
            });
        // If a task is nestled between deps and adeps, maintain the structure
        // of the graph by blocking the adeps on each of the deps.
        // E.g. if we remove b from (a <- b <- c), then we get (a <- c).
        deps.product(&adeps, self).for_each(|(dep, adep)| {
            // It should not be possible to cause a cycle when blocking an
            // adep on a dep because there would already be a cycle if so.
            self.block(adep).on(dep).unwrap();
        });
        if self.incomplete.contains(&id) {
            self.incomplete
                .remove_from_layer(&id, self.incomplete.depth(&id).unwrap());
        } else if self.complete.contains(&id) {
            remove_first_occurrence_from_vec(&mut self.complete, &id);
        };
        self.tasks.remove_node(id.0);
        adeps.iter_sorted(self).for_each(|adep| {
            self.update_depth(adep, None);
        });
        (affected_after_unblocking_adeps | affected_after_unblocking_from_deps)
            - TaskSet::of(id)
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum SnoozeWarning {
    #[error("task not found")]
    TaskNotFound { id: TaskId },
    #[error("task is complete")]
    TaskIsComplete,
    #[error("task is already snoozed until a later time")]
    TaskIsAlreadySnoozed {
        current_snooze: DateTime<Utc>,
        requested_snooze: DateTime<Utc>,
    },
    #[error("snoozed until after due date")]
    SnoozedUntilAfterDueDate {
        snoozed_until: DateTime<Utc>,
        due_date: DateTime<Utc>,
    },
}

impl TodoList<'_> {
    pub fn snooze(
        &mut self,
        id: TaskId,
        start_date: DateTime<Utc>,
    ) -> Result<(), Vec<SnoozeWarning>> {
        let mut warnings = vec![];
        if let Some(depth) =
            match (self.incomplete.depth(&id), self.tasks.node_weight_mut(id.0))
            {
                (Some(depth), Some(task)) => {
                    let mut depth_to_move_to = None;
                    if task.start_date >= start_date {
                        warnings.push(SnoozeWarning::TaskIsAlreadySnoozed {
                            current_snooze: task.start_date,
                            requested_snooze: start_date,
                        });
                    } else {
                        task.start_date = start_date;
                        depth_to_move_to = Some(depth);
                    }
                    if let Some(due_date) = task.implicit_due_date {
                        if task.start_date > due_date {
                            warnings.push(
                                SnoozeWarning::SnoozedUntilAfterDueDate {
                                    snoozed_until: task.start_date,
                                    due_date,
                                },
                            );
                        }
                    }
                    depth_to_move_to
                }
                (_, None) => {
                    return Err(vec![SnoozeWarning::TaskNotFound { id }])
                }
                (None, _) => return Err(vec![SnoozeWarning::TaskIsComplete]),
            }
        {
            self.incomplete.remove_from_layer(&id, depth);
            self.put_in_incomplete_layer(id, depth);
        }
        if !warnings.is_empty() {
            return Err(warnings);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnsnoozeWarning {
    TaskIsComplete,
    NotSnoozed,
}

impl TodoList<'_> {
    pub fn unsnooze(&mut self, id: TaskId) -> Result<(), Vec<UnsnoozeWarning>> {
        let status = self.status(id).unwrap();
        if status == TaskStatus::Complete {
            return Err(vec![UnsnoozeWarning::TaskIsComplete]);
        }
        let task = self.tasks.node_weight_mut(id.0).unwrap();
        if !task.is_snoozed() {
            return Err(vec![UnsnoozeWarning::NotSnoozed]);
        }
        // Reset the start date to the creation time.
        task.start_date = task.creation_time;
        self.punt(id).map_err(|e| {
            // This is redundant but I'd rather not have a panicking code path
            // by using unwrap().
            vec![match e {
                PuntError::TaskIsComplete => UnsnoozeWarning::TaskIsComplete,
            }]
        })?;
        Ok(())
    }
}

impl TodoList<'_> {
    pub fn clean(&mut self) -> TaskSet {
        let incomplete_tasks: TaskSet = self
            .incomplete_tasks()
            .filter(|&id| self.get(id).is_some())
            .collect();
        incomplete_tasks.iter_sorted(self).for_each(|id| {
            self.update_depth(id, None);
            self.punt(id).unwrap();
        });
        incomplete_tasks
    }
}
