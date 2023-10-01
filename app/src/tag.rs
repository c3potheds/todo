use {
    super::util::{format_task, lookup_tasks},
    todo_cli::Tag,
    todo_model::{TaskSet, TaskStatus, TodoList},
    todo_printing::{Action, PrintableAppSuccess, PrintableResult},
};

fn print_all_tags<'list>(
    list: &'list TodoList,
    include_done: bool,
) -> PrintableResult<'list> {
    let tasks_to_print = list
        .all_tasks()
        .filter(|&id| {
            if let (Some(data), Some(status)) = (list.get(id), list.status(id))
            {
                return data.tag
                    && (include_done || status != TaskStatus::Complete);
            }
            false
        })
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}

fn mark_tasks(
    list: &mut TodoList,
    tasks_to_mark: &TaskSet,
    tag: bool,
) -> TaskSet {
    tasks_to_mark
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.set_tag(id, tag)
        })
}

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Tag,
) -> PrintableResult<'list> {
    if cmd.keys.is_empty() && cmd.unmark.is_empty() {
        return print_all_tags(list, cmd.include_done);
    }
    let tasks_to_mark = lookup_tasks(list, &cmd.keys);
    let tasks_to_unmark = lookup_tasks(list, &cmd.unmark);
    let mut mutated = false;
    let tasks_to_print = (mark_tasks(list, &tasks_to_mark, true)
        | mark_tasks(list, &tasks_to_unmark, false))
    .include_done(list, cmd.include_done)
    .iter_sorted(list)
    .map(|id| {
        format_task(list, id).action(
            if tasks_to_mark.contains(id) || tasks_to_unmark.contains(id) {
                mutated = true;
                Action::Select
            } else {
                Action::None
            },
        )
    })
    .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}
