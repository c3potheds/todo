use app::util::format_task;
use app::util::lookup_tasks;
use chrono::DateTime;
use chrono::Local;
use chrono::Utc;
use cli::Due;
use model::TaskId;
use model::TaskSet;
use model::TaskStatus;
use model::TodoList;
use printing::PrintableError;
use printing::TodoPrinter;

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

fn show_source_of_due_dates_for_tasks(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    tasks: Vec<TaskId>,
) {
    tasks
        .into_iter()
        .flat_map(|id| {
            let due_date = match list.get(id) {
                Some(task) => match task.implicit_due_date {
                    Some(due_date) => due_date,
                    None => return TaskSet::new().into_iter_unsorted(),
                },
                None => return TaskSet::new().into_iter_unsorted(),
            };
            list.transitive_adeps(id)
                .iter_sorted(list)
                .filter(|&adep| match list.implicit_due_date(adep) {
                    Some(Some(adep_due_date)) => due_date == adep_due_date,
                    _ => false,
                })
                .chain(std::iter::once(id))
                .collect::<TaskSet>()
                .into_iter_unsorted()
        })
        .collect::<TaskSet>()
        .iter_sorted(list)
        .for_each(|id| printer.print_task(&format_task(list, id)));
}

fn set_due_dates(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    tasks: Vec<TaskId>,
    due_date: Option<DateTime<Utc>>,
    include_done: bool,
) {
    tasks
        .into_iter()
        .flat_map(|id| list.set_due_date(id, due_date).into_iter_unsorted())
        .collect::<TaskSet>()
        .iter_sorted(list)
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .for_each(|id| {
            printer.print_task(&format_task(list, id));
        });
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
) {
    let tasks = if cmd.keys.is_empty() {
        None
    } else {
        Some(lookup_tasks(list, &cmd.keys))
    };
    let due_date = if cmd.due.is_empty() {
        None
    } else {
        let date_string = cmd.due.join(" ");
        match ::time_format::parse_time(
            Local,
            now.with_timezone(&Local),
            &date_string,
        ) {
            Ok(threshold) => Some(threshold.with_timezone(&Utc)),
            Err(_) => {
                printer.print_error(&PrintableError::CannotParseDueDate {
                    cannot_parse: date_string.clone(),
                });
                return;
            }
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
        (None, due_date, false) => show_all_tasks_with_due_dates(
            list,
            printer,
            due_date,
            cmd.include_done,
        ),
        (Some(tasks), None, false) => {
            show_source_of_due_dates_for_tasks(list, printer, tasks)
        }
        (None, Some(_), true) => {
            printer.print_error(&PrintableError::ConflictingArgs((
                "due".to_string(),
                "none".to_string(),
            )));
        }
        (None, None, true) => {
            show_tasks_without_due_date(list, printer, cmd.include_done)
        }
    }
}
