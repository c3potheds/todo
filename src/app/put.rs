use app::util::any_tasks_are_complete;
use app::util::format_task;
use app::util::lookup_tasks;
use cli::Put;
use itertools::Itertools;
use model::TaskId;
use model::TaskSet;
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

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Put) {
    let tasks_to_put = lookup_tasks(model, &cmd.keys);
    let before = lookup_tasks(model, &cmd.before);
    let after = lookup_tasks(model, &cmd.after);
    let include_done = cmd.include_done
        || any_tasks_are_complete(
            model,
            tasks_to_put
                .iter()
                .chain(before.iter())
                .chain(after.iter())
                .copied(),
        );
    let before_deps: TaskSet = before
        .iter()
        .copied()
        .flat_map(|id| model.deps(id).into_iter_unsorted())
        .collect();
    let after_adeps: TaskSet = after
        .iter()
        .copied()
        .flat_map(|id| model.adeps(id).into_iter_unsorted())
        .collect();
    let tasks_to_block_on: Vec<_> = after
        .iter()
        .copied()
        .chain(before_deps.iter_sorted(&model))
        .collect::<TaskSet>()
        .iter_sorted(&model)
        .collect();
    let tasks_to_block: Vec<_> = before
        .iter()
        .copied()
        .chain(after_adeps.iter_sorted(&model))
        .collect::<TaskSet>()
        .iter_sorted(&model)
        .collect();
    let pairs_to_block: Vec<(TaskId, TaskId)> = tasks_to_put
        .iter()
        .copied()
        .cartesian_product(tasks_to_block_on.iter().copied())
        .chain(
            tasks_to_block
                .iter()
                .copied()
                .cartesian_product(tasks_to_put.iter().copied()),
        )
        .collect();
    let mut blocked_tasks = HashSet::new();
    pairs_to_block
        .into_iter()
        .flat_map(|(blocked, blocking)| {
            match model.block(blocked).on(blocking) {
                Ok(affected) => {
                    blocked_tasks.insert(blocked);
                    affected.into_iter_unsorted()
                }
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
                if blocked_tasks.contains(&id) {
                    Action::Lock
                } else {
                    Action::None
                },
            ))
        });
}
