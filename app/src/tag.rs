use {
    super::util::{format_task, lookup_tasks},
    cli::Tag,
    model::{TaskSet, TaskStatus, TodoList},
    printing::{Action, TodoPrinter},
};

fn print_all_tags(
    list: &TodoList,
    printer: &mut dyn TodoPrinter,
    include_done: bool,
) {
    list.all_tasks()
        .filter(|&id| {
            if let (Some(data), Some(status)) = (list.get(id), list.status(id))
            {
                return data.tag
                    && (include_done || status != TaskStatus::Complete);
            }
            false
        })
        .for_each(|id| {
            let task = format_task(list, id);
            printer.print_task(&task.action(Action::None));
        });
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

pub fn run(list: &mut TodoList, printer: &mut dyn TodoPrinter, cmd: &Tag) {
    if cmd.keys.is_empty() && cmd.unmark.is_empty() {
        print_all_tags(list, printer, cmd.include_done);
        return;
    }
    let tasks_to_mark = lookup_tasks(list, &cmd.keys);
    let tasks_to_unmark = lookup_tasks(list, &cmd.unmark);
    (mark_tasks(list, &tasks_to_mark, true)
        | mark_tasks(list, &tasks_to_unmark, false))
    .include_done(list, cmd.include_done)
    .iter_sorted(list)
    .for_each(|id| {
        let task = format_task(list, id);
        printer.print_task(&task.action(
            if tasks_to_mark.contains(id) || tasks_to_unmark.contains(id) {
                Action::Select
            } else {
                Action::None
            },
        ));
    });
}
