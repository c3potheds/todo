use {
    super::util::{format_task, lookup_tasks, parse_due_date_or_print_error},
    chrono::{DateTime, Utc},
    cli::Due,
    model::{TaskId, TaskSet, TaskStatus, TodoList},
    printing::{PrintableError, TodoPrinter},
};

fn show_all_tasks_with_due_dates(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    earlier_than: Option<DateTime<Utc>>,
    include_done: bool,
) {
    list.all_tasks()
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .filter(|&id| match list.implicit_due_date(id) {
            Some(Some(date)) => match earlier_than {
                Some(threshold) => date <= threshold,
                None => true,
            },
            _ => false,
        })
        .for_each(|id| printer.print_task(&format_task(list, id)));
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

fn show_source_of_due_dates_for_tasks(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    tasks: TaskSet,
) {
    tasks
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            so_far | source_of_due_date(list, id)
        })
        .iter_sorted(list)
        .for_each(|id| printer.print_task(&format_task(list, id)));
}

fn set_due_dates(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    tasks: TaskSet,
    due_date: Option<DateTime<Utc>>,
    include_done: bool,
) -> bool {
    let mut mutated = false;
    tasks
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
        .for_each(|id| {
            printer.print_task(&format_task(list, id));
        });
    mutated
}

fn show_tasks_without_due_date(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    include_done: bool,
) {
    list.all_tasks()
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .filter(|&id| list.implicit_due_date(id) == Some(None))
        .for_each(|id| {
            printer.print_task(&format_task(list, id));
        });
}

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Due,
) -> bool {
    let tasks = if cmd.keys.is_empty() {
        None
    } else {
        Some(lookup_tasks(list, &cmd.keys))
    };
    let due_date = match parse_due_date_or_print_error(now, &cmd.due, printer) {
        Ok(due_date) => due_date,
        Err(_) => {
            return false;
        }
    };
    match (tasks, due_date, cmd.none) {
        (Some(tasks), Some(due_date), false) => set_due_dates(
            list,
            printer,
            tasks,
            Some(due_date),
            cmd.include_done,
        ),
        (Some(tasks), _, true) => {
            set_due_dates(list, printer, tasks, None, cmd.include_done)
        }
        (None, due_date, false) => {
            show_all_tasks_with_due_dates(
                list,
                printer,
                due_date,
                cmd.include_done,
            );
            false
        }
        (Some(tasks), None, false) => {
            show_source_of_due_dates_for_tasks(list, printer, tasks);
            false
        }
        (None, Some(_), true) => {
            printer.print_error(&PrintableError::ConflictingArgs((
                "due".to_string(),
                "none".to_string(),
            )));
            false
        }
        (None, None, true) => {
            show_tasks_without_due_date(list, printer, cmd.include_done);
            false
        }
    }
}
