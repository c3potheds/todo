use app::util::format_task;
use app::util::lookup_tasks;
use cli::Block;
use itertools::Itertools;
use model::TaskId;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::collections::HashSet;

fn print_block_error(
    printer: &mut impl TodoPrinter,
    model: &TodoList,
    blocked: TaskId,
    blocking: TaskId,
) {
    model.position(blocked).zip(model.position(blocking)).map(
        |(cannot_block, requested_dependency)| {
            printer.print_error(
                &PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block: cannot_block,
                    requested_dependency: requested_dependency,
                },
            )
        },
    );
}

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Block) {
    let tasks_to_block = lookup_tasks(&model, &cmd.keys);
    let tasks_to_block_on = lookup_tasks(&model, &cmd.on);
    let tasks_to_print = tasks_to_block
        .iter()
        .copied()
        .cartesian_product(tasks_to_block_on.iter().copied())
        .flat_map(|(blocked, blocking)| {
            match model.block(blocked).on(blocking) {
                Ok(()) => vec![blocked, blocking].into_iter(),
                Err(_) => {
                    print_block_error(printer, model, blocked, blocking);
                    vec![].into_iter()
                }
            }
        })
        .collect::<HashSet<_>>();
    model
        .all_tasks()
        .filter(|id| tasks_to_print.contains(id))
        .for_each(|id| {
            printer.print_task(&format_task(
                model,
                id,
                if tasks_to_block.contains(&id) {
                    Action::Lock
                } else {
                    Action::None
                },
            ));
        });
}
