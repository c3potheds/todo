use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use app::util::should_include_done;
use cli::Put;
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
    printer.print_error(&PrintableError::CannotBlockBecauseWouldCauseCycle {
        cannot_block: format_task_brief(model, blocked),
        requested_dependency: format_task_brief(model, blocking),
    });
}

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Put) {
    let tasks_to_put = lookup_tasks(model, &cmd.keys);
    let before = lookup_tasks(model, &cmd.preposition.before);
    let after = lookup_tasks(model, &cmd.preposition.after);
    let include_done = should_include_done(
        cmd.include_done,
        model,
        (tasks_to_put.clone() | before.clone() | after.clone()).iter_unsorted(),
    );
    let before_deps: TaskSet = before
        .iter_unsorted()
        .flat_map(|id| model.deps(id).into_iter_unsorted())
        .collect();
    let after_adeps: TaskSet = after
        .iter_unsorted()
        .flat_map(|id| model.adeps(id).into_iter_unsorted())
        .collect();
    let tasks_to_block_on = after | before_deps;
    let tasks_to_block = before | after_adeps;
    let pairs_to_block: Vec<(TaskId, TaskId)> = tasks_to_put
        .product(&tasks_to_block_on, model)
        .chain(
            tasks_to_put
                .product(&tasks_to_block, model)
                .map(|(a, b)| (b, a)),
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
