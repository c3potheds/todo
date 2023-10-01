use {
    super::util::{format_task, lookup_tasks},
    todo_cli::Priority,
    todo_model::{TaskId, TaskSet, TaskStatus, TodoList},
    todo_printing::{PrintableAppSuccess, PrintableResult},
};

fn set_priority<'list>(
    list: &'list mut TodoList,
    tasks: TaskSet,
    priority: i32,
    include_done: bool,
) -> PrintableResult<'list> {
    let mut mutated = false;
    let tasks_to_print = tasks
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            let affected_by_id = list.set_priority(id, priority);
            if affected_by_id.is_empty() {
                return so_far;
            }
            mutated = true;
            so_far | affected_by_id
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
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

fn show_source_of_priority_for_tasks<'list>(
    list: &'list TodoList,
    tasks: TaskSet,
    include_done: bool,
) -> PrintableResult<'list> {
    let tasks_to_print = tasks
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            so_far | source_of_priority(list, id) | TaskSet::of(id)
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}

fn show_all_tasks_with_priority<'list>(
    list: &'list TodoList,
    priority: i32,
    include_done: bool,
) -> PrintableResult<'list> {
    let tasks_to_print = list
        .all_tasks()
        .filter(|&id| {
            include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .filter(|&id| match list.implicit_priority(id) {
            Some(p) => p >= priority,
            None => false,
        })
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Priority,
) -> PrintableResult<'list> {
    let tasks = if cmd.keys.is_empty() {
        None
    } else {
        Some(lookup_tasks(list, &cmd.keys))
    };
    let priority = cmd.priority;
    match (tasks, priority) {
        (Some(tasks), Some(priority)) => {
            set_priority(list, tasks, priority, cmd.include_done)
        }
        (Some(tasks), None) => {
            show_source_of_priority_for_tasks(list, tasks, cmd.include_done)
        }
        (None, Some(priority)) => {
            show_all_tasks_with_priority(list, priority, cmd.include_done)
        }
        (None, None) => show_all_tasks_with_priority(list, 1, cmd.include_done),
    }
}
