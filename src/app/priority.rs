use app::util::format_task;
use app::util::lookup_tasks;
use chrono::DateTime;
use chrono::Utc;
use cli::Priority;
use model::TaskId;
use model::TaskSet;
use model::TaskStatus;
use model::TodoList;
use printing::TodoPrinter;

fn set_priority(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    tasks: Vec<TaskId>,
    priority: i32,
    include_done: bool,
) {
    tasks
        .into_iter()
        .flat_map(|id| list.set_priority(id, priority).into_iter_unsorted())
        .collect::<TaskSet>()
        .iter_sorted(list)
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .for_each(|id| printer.print_task(&format_task(list, id, now)));
}

fn show_source_of_priority_for_tasks(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    tasks: Vec<TaskId>,
    include_done: bool,
) {
    tasks
        .into_iter()
        .flat_map(|id| {
            let priority = match list.implicit_priority(id) {
                Some(p) => p,
                None => return TaskSet::new().into_iter_unsorted(),
            };
            list.transitive_adeps(id)
                .iter_sorted(list)
                .filter(|&adep| match list.implicit_priority(adep) {
                    Some(p) => p == priority,
                    None => false,
                })
                .chain(std::iter::once(id))
                .collect::<TaskSet>()
                .into_iter_unsorted()
        })
        .collect::<TaskSet>()
        .iter_sorted(list)
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .for_each(|id| {
            printer.print_task(&format_task(list, id, now));
        })
}

fn show_all_tasks_with_priority(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    priority: i32,
    include_done: bool,
) {
    list.all_tasks()
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .filter(|&id| match list.implicit_priority(id) {
            Some(p) => p >= priority,
            None => false,
        })
        .for_each(|id| {
            printer.print_task(&format_task(list, id, now));
        })
}

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Priority,
) {
    let tasks = if cmd.keys.is_empty() {
        None
    } else {
        Some(lookup_tasks(list, &cmd.keys))
    };
    let priority = cmd.priority;
    match (tasks, priority) {
        (Some(tasks), Some(priority)) => {
            set_priority(list, printer, now, tasks, priority, cmd.include_done)
        }
        (Some(tasks), None) => show_source_of_priority_for_tasks(
            list,
            printer,
            now,
            tasks,
            cmd.include_done,
        ),
        (None, Some(priority)) => show_all_tasks_with_priority(
            list,
            printer,
            now,
            priority,
            cmd.include_done,
        ),
        (None, None) => show_all_tasks_with_priority(
            list,
            printer,
            now,
            1,
            cmd.include_done,
        ),
    }
}
