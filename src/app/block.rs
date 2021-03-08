use app::util::format_task;
use app::util::lookup_tasks;
use cli::Block;
use itertools::Itertools;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::PrintingContext;
use printing::TodoPrinter;
use std::collections::HashSet;

pub fn run(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Block,
) {
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
                    model
                        .get_number(blocked)
                        .zip(model.get_number(blocking))
                        .map(|(cannot_block, requested_dependency)| {
                            printer.print_error(
                            &PrintableError::CannotBlockBecauseWouldCauseCycle {
                                cannot_block: cannot_block,
                                requested_dependency: requested_dependency,
                            },
                        )
                        });
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
                printing_context,
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
