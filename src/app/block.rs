use app::util::any_tasks_are_complete;
use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use cli::Block;
use itertools::Itertools;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;

fn print_block_error(
    printer: &mut impl TodoPrinter,
    model: &TodoList,
    blocked: TaskId,
    blocking: TaskId,
) {
    printer.print_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
        cannot_block: format_task_brief(model, blocked),
        requested_dependency: format_task_brief(model, blocking),
    });
}

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Block) {
    let tasks_to_block = lookup_tasks(&model, &cmd.keys);
    let tasks_to_block_on = lookup_tasks(&model, &cmd.on);
    let include_done = cmd.include_done
        || any_tasks_are_complete(
            model,
            tasks_to_block
                .iter()
                .chain(tasks_to_block_on.iter())
                .copied(),
        );
    tasks_to_block
        .iter()
        .copied()
        .cartesian_product(tasks_to_block_on.iter().copied())
        .flat_map(|(blocked, blocking)| {
            match model.block(blocked).on(blocking) {
                Ok(affected) => affected.into_iter_unsorted(),
                Err(_) => {
                    print_block_error(printer, model, blocked, blocking);
                    TaskSet::new().into_iter_unsorted()
                }
            }
        })
        .collect::<TaskSet>()
        .include_done(model, include_done)
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id).action(
                if tasks_to_block.contains(&id) {
                    Action::Lock
                } else {
                    Action::None
                },
            ))
        });
}
