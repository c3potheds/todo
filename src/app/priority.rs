use app::util::format_task;
use app::util::lookup_tasks;
use cli::Priority;
use model::TaskId;
use model::TaskSet;
use model::TaskStatus;
use model::TodoList;
use printing::TodoPrinter;

fn set_priority(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    tasks: TaskSet,
    priority: i32,
    include_done: bool,
) {
    tasks
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.set_priority(id, priority)
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .for_each(|id| printer.print_task(&format_task(list, id)));
}

fn source_of_priority(list: &TodoList, id: TaskId) -> TaskSet {
    let priority = match list.implicit_priority(id) {
        Some(p) => p,
        None => return TaskSet::default(),
    };
    list.transitive_adeps(id)
        .iter_sorted(list)
        .take_while(|&adep| match list.implicit_priority(adep) {
            Some(p) => p == priority,
            None => false,
        })
        .collect()
}

fn show_source_of_priority_for_tasks(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    tasks: TaskSet,
    include_done: bool,
) {
    tasks
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            so_far | source_of_priority(list, id) | TaskSet::of(id)
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .for_each(|id| {
            printer.print_task(&format_task(list, id));
        })
}

fn show_all_tasks_with_priority(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
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
            printer.print_task(&format_task(list, id));
        })
}

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
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
            set_priority(list, printer, tasks, priority, cmd.include_done)
        }
        (Some(tasks), None) => show_source_of_priority_for_tasks(
            list,
            printer,
            tasks,
            cmd.include_done,
        ),
        (None, Some(priority)) => show_all_tasks_with_priority(
            list,
            printer,
            priority,
            cmd.include_done,
        ),
        (None, None) => {
            show_all_tasks_with_priority(list, printer, 1, cmd.include_done)
        }
    }
}
