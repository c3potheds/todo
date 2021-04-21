use app::util::any_tasks_are_complete;
use app::util::format_task;
use app::util::lookup_tasks;
use app::util::pairwise;
use cli::Chain;
use model::BlockError;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::collections::HashMap;

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Chain) {
    let tasks = lookup_tasks(model, &cmd.keys);
    let include_done = cmd.include_done
        || any_tasks_are_complete(model, tasks.iter().copied());
    let mut actions = HashMap::new();
    pairwise(tasks.iter().copied())
        .fold(TaskSet::new(), |so_far, (a, b)| {
            match model.block(b).on(a) {
                Ok(affected) => {
                    actions.insert(b, Action::Lock);
                    so_far | affected
                }
                Err(BlockError::WouldCycle(_)) => {
                    printer.print_error(
                        &PrintableError::CannotBlockBecauseWouldCauseCycle {
                            cannot_block: model.position(b).unwrap(),
                            requested_dependency: model.position(a).unwrap(),
                        },
                    );
                    so_far
                }
                Err(BlockError::WouldBlockOnSelf) => so_far,
            }
        })
        .include_done(model, include_done)
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(
                &format_task(model, id)
                    .action(*actions.get(&id).unwrap_or(&Action::None)),
            );
        });
}
