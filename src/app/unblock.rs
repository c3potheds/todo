use app::util::format_task;
use app::util::lookup_tasks;
use cli::Unblock;
use itertools::Itertools;
use model::TaskId;
use model::TodoList;
use printing::Action;
use printing::PrintableWarning;
use printing::PrintingContext;
use printing::TodoPrinter;
use std::collections::HashSet;

fn print_unblock_warning(
    printer: &mut impl TodoPrinter,
    model: &TodoList,
    blocking: TaskId,
    blocked: TaskId,
) {
    model
        .get_number(blocked)
        .zip(model.get_number(blocking))
        .map(|(cannot_unblock, requested_unblock_from)| {
            printer.print_warning(
                &PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                    cannot_unblock: cannot_unblock,
                    requested_unblock_from: requested_unblock_from,
                },
            )
        });
}

pub fn run(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Unblock,
) {
    let tasks_to_unblock = lookup_tasks(&model, &cmd.keys);
    let tasks_to_unblock_from = lookup_tasks(&model, &cmd.from);
    let tasks_to_print = tasks_to_unblock
        .iter()
        .copied()
        .cartesian_product(tasks_to_unblock_from.iter().copied())
        .flat_map(|(blocked, blocking)| {
            match model.unblock(blocked).from(blocking) {
                Ok(()) => vec![blocking, blocked].into_iter(),
                Err(_) => {
                    print_unblock_warning(printer, model, blocking, blocked);
                    vec![].into_iter()
                }
            }
        })
        .collect::<HashSet<_>>();

    model
        .all_tasks()
        .filter(|id| tasks_to_print.contains(&id))
        .for_each(|id| {
            printer.print_task(&format_task(
                printing_context,
                model,
                id,
                if tasks_to_unblock.contains(&id) {
                    Action::Unlock
                } else {
                    Action::None
                },
            ));
        });
}
