use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_task;
use app::util::pairwise;
use app::util::should_include_done;
use cli::Chain;
use model::BlockError;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::collections::HashMap;

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Chain) {
    let tasks = cmd
        .keys
        .iter()
        .flat_map(|key| lookup_task(model, key).iter_sorted(model))
        .collect::<Vec<_>>();
    let include_done =
        should_include_done(cmd.include_done, model, tasks.iter().copied());
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
                            cannot_block: format_task_brief(model, b),
                            requested_dependency: format_task_brief(model, a),
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
