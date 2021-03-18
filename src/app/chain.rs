use app::util::format_task;
use app::util::lookup_tasks;
use app::util::pairwise;
use cli::Chain;
use itertools::Itertools;
use model::BlockError;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::collections::HashSet;

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Chain) {
    let task_lists: Vec<Vec<TaskId>> = cmd
        .keys
        .iter()
        .map(|key| lookup_tasks(model, std::iter::once(key)))
        .collect();
    let first = task_lists.first();
    pairwise(task_lists.iter())
        .fold(HashSet::new(), |mut so_far, (deps, ids)| {
            deps.iter().cartesian_product(ids.iter()).for_each(
                |(&dep, &id)| match model.block(id).on(dep) {
                    Ok(_) => {
                        so_far.insert(dep);
                        so_far.insert(id);
                    }
                    Err(BlockError::WouldCycle(_)) => printer.print_error(
                        &PrintableError::CannotBlockBecauseWouldCauseCycle {
                            cannot_block: model.position(id).unwrap(),
                            requested_dependency: model.position(dep).unwrap(),
                        },
                    ),
                    Err(BlockError::WouldBlockOnSelf) => (),
                },
            );
            so_far
        })
        .into_iter()
        .collect::<TaskSet>()
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(
                model,
                id,
                first
                    .filter(|fs| fs.iter().any(|&f| f == id))
                    .map(|_| Action::None)
                    .unwrap_or(Action::Lock),
            ));
        });
}
