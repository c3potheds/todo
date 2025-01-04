use chrono::DateTime;
use chrono::Local;
use chrono::Timelike;
use chrono::Utc;
use todo_cli::Due;
use todo_model::TaskId;
use todo_model::TaskSet;
use todo_model::TaskStatus;
use todo_model::TodoList;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableError;
use todo_printing::PrintableResult;

use super::util::format_task;
use super::util::lookup_tasks;

fn show_all_tasks_with_due_dates<'list>(
    list: &'list TodoList,
    earlier_than: DateTime<Utc>,
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
            Some(Some(date)) => date <= earlier_than,
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
    let due_date = match list.get(id).unwrap().implicit_due_date {
        Some(due_date) => due_date,
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
                // If set_due_date() returns an empty set, that means no change
                // occurred. But we should still print the task that we're
                // setting the due date of even though it didn't change.
                return so_far | TaskSet::of(id);
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

// Fast-forwards the date-time to the end of the day in the local time zone.
fn end_of_day(now: DateTime<Utc>) -> DateTime<Utc> {
    now.with_timezone(&Local)
        .with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
        .with_nanosecond(999_999_999)
        .unwrap()
        .with_timezone(&Utc)
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
    let due_date = cmd.due;
    match (tasks, due_date, cmd.none) {
        (Some(tasks), Some(due_date), false) => {
            set_due_dates(list, tasks, Some(due_date), cmd.include_done)
        }
        (Some(tasks), _, true) => {
            set_due_dates(list, tasks, None, cmd.include_done)
        }
        (None, due_date, false) => show_all_tasks_with_due_dates(
            list,
            due_date.unwrap_or_else(|| end_of_day(now)),
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
