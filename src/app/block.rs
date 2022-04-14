use crate::{
    app::util::{
        format_task, format_task_brief, lookup_tasks, should_include_done,
    },
    cli::Block,
    model::{TaskId, TaskSet, TodoList},
    printing::{Action, PrintableError, TodoPrinter},
};

fn print_block_error(
    printer: &mut impl TodoPrinter,
    list: &TodoList,
    blocked: TaskId,
    blocking: TaskId,
) {
    printer.print_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
        cannot_block: format_task_brief(list, blocked),
        requested_dependency: format_task_brief(list, blocking),
    });
}

pub fn run(list: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Block) {
    let tasks_to_block = lookup_tasks(list, &cmd.keys);
    let tasks_to_block_on = lookup_tasks(list, &cmd.on);
    let include_done = should_include_done(
        cmd.include_done,
        list,
        (tasks_to_block.clone() | tasks_to_block_on.clone()).iter_sorted(list),
    );
    tasks_to_block
        .product(&tasks_to_block_on, list)
        .fold(TaskSet::default(), |so_far, (blocked, blocking)| match list
            .block(blocked)
            .on(blocking)
        {
            Ok(affected) => so_far | affected,
            Err(_) => {
                print_block_error(printer, list, blocked, blocking);
                so_far
            }
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .for_each(|id| {
            printer.print_task(&format_task(list, id).action(
                if tasks_to_block.contains(id) {
                    Action::Lock
                } else {
                    Action::None
                },
            ))
        });
}
