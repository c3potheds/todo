use todo_cli::Block;
use todo_model::TaskId;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableError;
use todo_printing::PrintableResult;

use super::util::format_task;
use super::util::format_task_brief;
use super::util::lookup_tasks;
use super::util::should_include_done;

fn to_error(list: &TodoList, a: TaskId, b: TaskId) -> Vec<PrintableError> {
    vec![PrintableError::CannotBlockBecauseWouldCauseCycle {
        cannot_block: format_task_brief(list, a),
        requested_dependency: format_task_brief(list, b),
    }]
}

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Block,
) -> PrintableResult<'list> {
    let tasks_to_block = lookup_tasks(list, &cmd.keys);
    let tasks_to_block_on = lookup_tasks(list, &cmd.on);
    let include_done = should_include_done(
        cmd.include_done,
        list,
        (tasks_to_block.clone() | tasks_to_block_on.clone()).iter_sorted(list),
    );
    let tasks_to_print: Vec<_> = tasks_to_block
        .product(&tasks_to_block_on, list)
        .try_fold(
            TaskSet::default(),
            |so_far, (a, b)| -> Result<_, Vec<PrintableError>> {
                Ok(list.block(a).on(b).map_err(|_| to_error(list, a, b))?
                    | so_far)
            },
        )?
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id).action(if tasks_to_block.contains(id) {
                Action::Lock
            } else {
                Action::None
            })
        })
        .collect();
    let mutated = !tasks_to_print.is_empty();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}
