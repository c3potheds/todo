use std::collections::BTreeSet;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::TaskId;
use crate::TaskStatus;
use crate::TodoList;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
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
        Some(self.cmp(other))
    }
}

impl Ord for TaskIdWithPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position.cmp(&other.position)
    }
}

impl TaskSet {
    pub fn of(id: TaskId) -> Self {
        TaskSet {
            ids: HashSet::from_iter(std::iter::once(id)),
        }
    }

    pub fn push(&mut self, id: TaskId) {
        self.ids.insert(id);
    }

    /// Iterates the set in an arbitrary order. Careful when using this; it may
    /// cause non-determinism. It is more efficient than iterating in sorted
    /// order.
    pub fn iter_unsorted(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.ids.iter().copied()
    }

    pub fn into_iter_unsorted(self) -> impl Iterator<Item = TaskId> {
        self.ids.into_iter()
    }

    pub fn contains(&self, id: TaskId) -> bool {
        self.ids.contains(&id)
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Iterates the set in sorted order, where the ordering is defined by the
    /// position in the list.
    pub fn iter_sorted(
        &self,
        list: &TodoList,
    ) -> impl DoubleEndedIterator<Item = TaskId> {
        self.ids
            .iter()
            .flat_map(|&id| {
                list.position(id)
                    .map(|pos| TaskIdWithPosition { id, position: pos })
                    .into_iter()
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|task_id_with_pos| task_id_with_pos.id)
    }

    pub fn include_done(self, list: &TodoList, include_done: bool) -> Self {
        if include_done {
            self
        } else {
            self.partition_done(list).1
        }
    }

    pub fn partition_done(self, list: &TodoList) -> (Self, Self) {
        let (done, not_done): (HashSet<_>, HashSet<_>) = self
            .ids
            .into_iter()
            .partition(|&id| list.status(id) == Some(TaskStatus::Complete));
        (Self { ids: done }, Self { ids: not_done })
    }

    pub fn as_sorted_vec(&self, list: &TodoList) -> Vec<TaskId> {
        self.iter_sorted(list).collect()
    }
}

impl TaskSet {
    pub fn product(
        &self,
        other: &Self,
        list: &TodoList,
    ) -> impl Iterator<Item = (TaskId, TaskId)> {
        use itertools::Itertools;
        self.iter_sorted(list)
            .cartesian_product(other.iter_sorted(list).collect::<Vec<_>>())
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
        let mut ids = self.ids;
        ids.extend(other.ids);
        Self { ids }
    }
}

impl std::ops::BitAnd for TaskSet {
    type Output = TaskSet;
    fn bitand(self, other: Self) -> Self::Output {
        let mut ids = self.ids;
        ids.retain(|&id| other.ids.contains(&id));
        Self { ids }
    }
}

impl std::ops::Sub for TaskSet {
    type Output = TaskSet;
    fn sub(self, other: TaskSet) -> Self::Output {
        let mut ids = self.ids;
        ids.retain(|&id| !other.ids.contains(&id));
        Self { ids }
    }
}
