use {
    super::util::{format_task, lookup_tasks, parse_due_date},
    chrono::{DateTime, Utc},
    cli::Due,
    model::{TaskId, TaskSet, TaskStatus, TodoList},
    printing::{PrintableAppSuccess, PrintableError, PrintableResult},
};

fn show_all_tasks_with_due_dates<'list>(
    list: &'list TodoList,
    earlier_than: Option<DateTime<Utc>>,
    include_done: bool,
    include_blocked: bool,
) -> PrintableResult<'list> {
    let tasks_to_print = list
        .all_tasks()
        .filter(|&id| {
            (include_done || list.status(id) != Some(TaskStatus::Complete))
                && (include_blocked
                    || list.status(id) != Some(TaskStatus::Blocked))
        })
        .filter(|&id| match list.implicit_due_date(id) {
            Some(Some(date)) => match earlier_than {
                Some(threshold) => date <= threshold,
                None => true,
            },
            _ => false,
        })
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}

fn source_of_due_date(list: &TodoList, id: TaskId) -> TaskSet {
    let due_date = match list.get(id) {
        Some(task) => match task.implicit_due_date {
            Some(due_date) => due_date,
            None => return TaskSet::default(),
        },
        None => return TaskSet::default(),
    };
    list.transitive_adeps(id)
        .iter_sorted(list)
        .filter(|&adep| match list.implicit_due_date(adep) {
            Some(Some(adep_due_date)) => due_date == adep_due_date,
            _ => false,
        })
        .chain(std::iter::once(id))
        .collect()
}

fn show_source_of_due_dates_for_tasks<'list>(
    list: &'list TodoList,
    tasks: TaskSet,
) -> PrintableResult<'list> {
    let tasks_to_print = tasks
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            so_far | source_of_due_date(list, id)
        })
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}

fn set_due_dates<'list>(
    list: &'list mut TodoList,
    tasks: TaskSet,
    due_date: Option<DateTime<Utc>>,
    include_done: bool,
) -> PrintableResult<'list> {
    let mut mutated = false;
    let tasks_to_print = tasks
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            let affected_by_id = list.set_due_date(id, due_date);
            if affected_by_id.is_empty() {
                return so_far;
            }
            mutated = true;
            so_far | affected_by_id
        })
        .iter_sorted(list)
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}

fn show_tasks_without_due_date<'list>(
    list: &'list TodoList,
    include_done: bool,
    include_blocked: bool,
) -> PrintableResult<'list> {
    let tasks_to_print = list
        .all_tasks()
        .filter(|&id| {
            (include_done || list.status(id) != Some(TaskStatus::Complete))
                && (include_blocked
                    || list.status(id) != Some(TaskStatus::Blocked))
        })
        .filter(|&id| list.implicit_due_date(id) == Some(None))
        .map(|id| format_task(list, id));
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print.collect(),
        ..Default::default()
    })
}

pub fn run<'list>(
    list: &'list mut TodoList,
    now: DateTime<Utc>,
    cmd: &Due,
) -> PrintableResult<'list> {
    let tasks = if cmd.keys.is_empty() {
        None
    } else {
        Some(lookup_tasks(list, &cmd.keys))
    };
    let due_date = parse_due_date(now, &cmd.due).map_err(|e| vec![e])?;
    match (tasks, due_date, cmd.none) {
        (Some(tasks), Some(due_date), false) => {
            set_due_dates(list, tasks, Some(due_date), cmd.include_done)
        }
        (Some(tasks), _, true) => {
            set_due_dates(list, tasks, None, cmd.include_done)
        }
        (None, due_date, false) => show_all_tasks_with_due_dates(
            list,
            due_date,
            cmd.include_done,
            cmd.include_blocked,
        ),
        (Some(tasks), None, false) => {
            show_source_of_due_dates_for_tasks(list, tasks)
        }
        (None, Some(_), true) => Err(vec![PrintableError::ConflictingArgs((
            "due".to_string(),
            "none".to_string(),
        ))]),
        (None, None, true) => show_tasks_without_due_date(
            list,
            cmd.include_done,
            cmd.include_blocked,
        ),
    }
}
