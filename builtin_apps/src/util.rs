use std::collections::HashMap;
use {
    chrono::{DateTime, Duration, Local, Utc},
    std::convert::TryFrom,
    todo_lookup_key::Key,
    todo_model::{DurationInSeconds, TaskId, TaskSet, TaskStatus, TodoList},
    todo_printing::{
        BriefPrintableTask, Plicit, PrintableError, PrintableTask, Status,
    },
};

fn to_printing_status(status: TaskStatus) -> Status {
    match status {
        TaskStatus::Incomplete => Status::Incomplete,
        TaskStatus::Blocked => Status::Blocked,
        TaskStatus::Complete => Status::Complete,
    }
}

struct WrappedPrintableTask<'a>(PrintableTask<'a>);

fn wrap(task: PrintableTask) -> WrappedPrintableTask {
    WrappedPrintableTask(task)
}

impl<'a> WrappedPrintableTask<'a> {
    fn add_deps_if_necessary(
        self,
        list: &TodoList,
        id: TaskId,
        status: TaskStatus,
    ) -> WrappedPrintableTask<'a> {
        if status != TaskStatus::Blocked {
            return self;
        }
        let deps = list.transitive_deps(id);
        if deps.is_empty() {
            return self;
        }
        // Incomplete deps are deps that can be completed now (i.e. neither
        // complete nor blocked).
        let incomplete = deps
            .iter_unsorted()
            .filter(|&dep| list.status(dep) == Some(TaskStatus::Incomplete))
            .count();
        wrap(self.0.deps_stats(incomplete, deps.len()))
    }

    fn add_adeps_if_necessary(
        self,
        list: &TodoList,
        id: TaskId,
        status: TaskStatus,
    ) -> WrappedPrintableTask<'a> {
        if status != TaskStatus::Incomplete {
            return self;
        }
        let adeps = list.transitive_adeps(id);
        if adeps.is_empty() {
            return self;
        }
        // Unlockable adeps are tasks that would be unlocked if this
        // task were completed. In other words, the adep is unlockable
        // if this task is the only incomplete dependency of the adep.
        let unlockable = adeps
            .iter_unsorted()
            .filter(|&adep| {
                list.deps(adep)
                    .iter_unsorted()
                    .filter(|&dep| dep != id)
                    .all(|dep| list.status(dep) == Some(TaskStatus::Complete))
            })
            .count();
        wrap(self.0.adeps_stats(unlockable, adeps.len()))
    }

    fn unwrap(self) -> PrintableTask<'a> {
        self.0
    }
}

pub fn format_task<'list>(
    list: &'list TodoList<'_>,
    id: TaskId,
) -> PrintableTask<'list> {
    match (
        list.get(id),
        list.position(id),
        list.status(id),
        list.implicit_priority(id),
        list.implicit_due_date(id),
    ) {
        (
            Some(task),
            Some(pos),
            Some(status),
            Some(implicit_priority),
            Some(implicit_due_date),
        ) => {
            let mut result =
                PrintableTask::new(&task.desc, pos, to_printing_status(status));
            if implicit_priority != 0 {
                result =
                    result.priority(if implicit_priority == task.priority {
                        Plicit::Explicit(task.priority)
                    } else {
                        Plicit::Implicit(implicit_priority)
                    });
            }
            result = match (status, implicit_due_date, task.completion_time) {
                (
                    TaskStatus::Complete,
                    Some(due_date),
                    Some(completion_time),
                ) => result.punctuality(completion_time - due_date),
                (_, Some(due_date), _) => {
                    result.due_date(if task.due_date == Some(due_date) {
                        Plicit::Explicit(due_date)
                    } else {
                        Plicit::Implicit(due_date)
                    })
                }
                _ => result,
            };
            if task.budget.0 > 0 {
                result = result.budget(Duration::seconds(task.budget.0.into()));
            }
            if task.start_date > task.creation_time {
                result = result.start_date(task.start_date);
            }
            if task.tag {
                result = result.as_tag();
            }
            for tag_id in TaskSet::from_iter(task.implicit_tags.iter().cloned())
                .iter_sorted(list)
                .rev()
            {
                if let Some(tag_data) = list.get(tag_id) {
                    result = result.tag(&tag_data.desc);
                }
            }
            wrap(result)
                .add_deps_if_necessary(list, id, status)
                .add_adeps_if_necessary(list, id, status)
                .unwrap()
        }
        _ => panic!("Failed to get task info for id {:?}", id),
    }
}

pub fn format_task_brief(list: &TodoList, id: TaskId) -> BriefPrintableTask {
    BriefPrintableTask::new(
        list.position(id).unwrap(),
        to_printing_status(list.status(id).unwrap()),
    )
}

pub fn format_tasks_brief(
    list: &TodoList,
    tasks: &TaskSet,
) -> Vec<BriefPrintableTask> {
    tasks
        .iter_sorted(list)
        .map(|id| format_task_brief(list, id))
        .collect()
}

pub fn lookup_task(list: &TodoList, key: &Key) -> TaskSet {
    match key {
        Key::ByNumber(n) => list.lookup_by_number(*n).into_iter().collect(),
        Key::ByName(ref name) => list
            .all_tasks()
            .filter(|&id| {
                list.get(id).filter(|task| &task.desc == name).is_some()
            })
            .collect(),
        Key::ByRange(start, end) => list
            .all_tasks()
            .filter(|&id| {
                list.position(id)
                    .filter(|pos| start <= pos && pos <= end)
                    .is_some()
            })
            .collect(),
    }
}

pub fn lookup_tasks<'a>(
    list: &'a TodoList,
    keys: impl IntoIterator<Item = &'a Key>,
) -> TaskSet {
    keys.into_iter().fold(TaskSet::default(), |so_far, key| {
        so_far | lookup_task(list, key)
    })
}

pub fn lookup_tasks_by_keys<'a>(
    list: &'a TodoList,
    keys: impl IntoIterator<Item = &'a Key>,
) -> HashMap<&'a Key, TaskSet> {
    keys.into_iter()
        .map(|key| (key, lookup_task(list, key)))
        .collect()
}

fn any_tasks_are_complete(
    list: &TodoList,
    mut tasks: impl Iterator<Item = TaskId>,
) -> bool {
    tasks.any(|id| list.status(id) == Some(TaskStatus::Complete))
}

pub fn should_include_done(
    from_cmdline: bool,
    list: &TodoList,
    tasks: impl IntoIterator<Item = TaskId>,
) -> bool {
    from_cmdline || any_tasks_are_complete(list, tasks.into_iter())
}

pub fn parse_due_date(
    now: DateTime<Utc>,
    chunks: &[String],
) -> Result<Option<DateTime<Utc>>, PrintableError> {
    if chunks.is_empty() {
        return Ok(None);
    }
    let due_date_string = chunks.join(" ");
    match ::todo_time_format::parse_time(
        Local,
        now.with_timezone(&Local),
        &due_date_string,
        ::todo_time_format::Snap::ToEnd,
    ) {
        Ok(due_date) => Ok(Some(due_date.with_timezone(&Utc))),
        Err(_) => Err(PrintableError::CannotParseDueDate {
            cannot_parse: due_date_string.to_string(),
        }),
    }
}

pub fn parse_budget(
    chunks: &[String],
) -> Result<DurationInSeconds, PrintableError> {
    let budget_string = chunks.join(" ");
    if budget_string == "0" || budget_string.is_empty() {
        return Ok(DurationInSeconds::default());
    }
    match humantime::parse_duration(&budget_string) {
        Ok(duration) => {
            Ok(DurationInSeconds(match u32::try_from(duration.as_secs()) {
                Ok(secs) => secs,
                Err(_) => {
                    return Err(PrintableError::DurationIsTooLong {
                        duration: duration.as_secs(),
                        string_repr: budget_string,
                    })
                }
            }))
        }
        Err(_) => Err(PrintableError::CannotParseDuration {
            cannot_parse: budget_string.clone(),
        }),
    }
}

pub fn parse_snooze_date(
    now: DateTime<Utc>,
    chunks: &[String],
) -> Result<Option<DateTime<Utc>>, PrintableError> {
    let date_string = chunks.join(" ");
    if date_string.is_empty() || date_string.is_empty() {
        return Ok(None);
    }
    match ::todo_time_format::parse_time(
        Local,
        now.with_timezone(&Local),
        &date_string,
        ::todo_time_format::Snap::ToStart,
    ) {
        Ok(snooze_date) => Ok(Some(snooze_date.with_timezone(&Utc))),
        Err(_) => Err(PrintableError::CannotParseDueDate {
            cannot_parse: date_string.to_string(),
        }),
    }
}
