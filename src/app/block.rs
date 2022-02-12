use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use app::util::should_include_done;
use cli::Block;
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
    let tasks_to_block = lookup_tasks(model, &cmd.keys);
    let tasks_to_block_on = lookup_tasks(model, &cmd.on);
    let include_done = should_include_done(
        cmd.include_done,
        model,
        (tasks_to_block.clone() | tasks_to_block_on.clone()).iter_sorted(model),
    );
    tasks_to_block
        .product(&tasks_to_block_on, model)
        .fold(TaskSet::default(), |so_far, (blocked, blocking)| {
            match model.block(blocked).on(blocking) {
                Ok(affected) => so_far | affected,
                Err(_) => {
                    print_block_error(printer, model, blocked, blocking);
                    so_far
                }
            }
        })
        .include_done(model, include_done)
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id).action(
                if tasks_to_block.contains(id) {
                    Action::Lock
                } else {
                    Action::None
                },
            ))
        });
}
