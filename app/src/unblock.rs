use {
    super::util::{
        format_task, format_task_brief, lookup_tasks, should_include_done,
    },
    cli::Unblock,
    model::{TaskId, TaskSet, TodoList},
    printing::{Action, PrintableError, PrintableWarning, TodoPrinter},
};

fn print_unblock_warning(
    printer: &mut dyn TodoPrinter,
    list: &TodoList,
    blocking: TaskId,
    blocked: TaskId,
) {
    printer.print_warning(
        &PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
            cannot_unblock: format_task_brief(list, blocked),
            requested_unblock_from: format_task_brief(list, blocking),
        },
    );
}

fn unblock_from_given(
    list: &mut TodoList,
    printer: &mut dyn TodoPrinter,
    tasks_to_unblock: &TaskSet,
    tasks_to_unblock_from: &TaskSet,
) -> TaskSet {
    tasks_to_unblock.product(tasks_to_unblock_from, list).fold(
        TaskSet::default(),
        |so_far, (blocked, blocking)| match list.unblock(blocked).from(blocking)
        {
            Ok(affected) => so_far | affected,
            Err(_) => {
                print_unblock_warning(printer, list, blocking, blocked);
                so_far
            }
        },
    )
}

fn unblock_from_all(
    list: &mut TodoList,
    tasks_to_unblock: &TaskSet,
) -> TaskSet {
    tasks_to_unblock
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            list.deps(id)
                .iter_unsorted()
                .fold(TaskSet::default(), |so_far, dep| {
                    list.unblock(id).from(dep).unwrap() | so_far
                })
                | so_far
        })
}

pub fn run(list: &mut TodoList, printer: &mut dyn TodoPrinter, cmd: &Unblock) {
    let tasks_to_unblock = lookup_tasks(list, &cmd.keys);
    let tasks_to_unblock_from = lookup_tasks(list, &cmd.from);
    let include_done = should_include_done(
        cmd.include_done,
        list,
        (tasks_to_unblock.clone() | tasks_to_unblock_from.clone())
            .iter_unsorted(),
    );
    if !cmd.from.is_empty() && tasks_to_unblock_from.is_empty() {
        printer.print_error(&PrintableError::NoMatchForKeys {
            keys: cmd.from.clone(),
        });
        return;
    }
    let tasks_to_print = if tasks_to_unblock_from.is_empty() {
        unblock_from_all(list, &tasks_to_unblock)
    } else {
        unblock_from_given(
            list,
            printer,
            &tasks_to_unblock,
            &tasks_to_unblock_from,
        )
    };
    tasks_to_print
        .include_done(list, include_done)
        .iter_sorted(list)
        .for_each(|id| {
            printer.print_task(&format_task(list, id).action(
                if tasks_to_unblock.contains(id) {
                    Action::Unlock
                } else {
                    Action::None
                },
            ));
        });
}
