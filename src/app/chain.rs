use app::util::any_tasks_are_complete;
use app::util::format_task;
use app::util::lookup_tasks;
use app::util::pairwise;
use chrono::DateTime;
use chrono::Utc;
use cli::Chain;
use itertools::Itertools;
use model::BlockError;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::collections::HashMap;

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Chain,
) {
    let task_lists: Vec<Vec<TaskId>> = cmd
        .keys
        .iter()
        .map(|key| lookup_tasks(model, std::iter::once(key)))
        .collect();
    let include_done = cmd.include_done
        || any_tasks_are_complete(
            model,
            task_lists.iter().flat_map(|inner| inner.iter()).copied(),
        );
    let mut actions = HashMap::new();
    pairwise(task_lists.iter())
        .fold(TaskSet::new(), |so_far, (deps, ids)| {
            deps.iter().cartesian_product(ids.iter()).fold(
                so_far,
                |so_far, (&dep, &id)| match model.block(id).on(dep) {
                    Ok(affected) => {
                        actions.insert(id, Action::Lock);
                        so_far | affected
                    }
                    Err(BlockError::WouldCycle(_)) => {
                        printer.print_error(
                        &PrintableError::CannotBlockBecauseWouldCauseCycle {
                            cannot_block: model.position(id).unwrap(),
                            requested_dependency: model.position(dep).unwrap(),
                        });
                        so_far
                    }
                    Err(BlockError::WouldBlockOnSelf) => so_far,
                },
            )
        })
        .include_done(model, include_done)
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(
                &format_task(model, id, now)
                    .action(*actions.get(&id).unwrap_or(&Action::None)),
            );
        });
}
