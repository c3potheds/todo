use std::collections::HashMap;

use todo_cli::Chain;
use todo_model::BlockError;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableError;
use todo_printing::PrintableResult;

use super::util::format_task;
use super::util::format_task_brief;
use super::util::lookup_task;
use super::util::should_include_done;

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Chain,
) -> PrintableResult<'list> {
    let tasks_to_chain = cmd
        .keys
        .iter()
        .flat_map(|key| lookup_task(list, key).iter_sorted(list))
        .collect::<Vec<_>>();
    let include_done = should_include_done(
        cmd.include_done,
        list,
        tasks_to_chain.iter().copied(),
    );
    let mut actions = HashMap::new();
    let mut mutated = false;
    use itertools::Itertools;
    let tasks_to_print = tasks_to_chain
        .into_iter()
        .tuple_windows()
        .try_fold(TaskSet::default(), |so_far, (a, b)| {
            match list.block(b).on(a) {
                Ok(affected) => {
                    mutated = true;
                    actions.insert(b, Action::Lock);
                    Ok(so_far | affected)
                }
                Err(BlockError::WouldCycle(_)) => {
                    Err(PrintableError::CannotBlockBecauseWouldCauseCycle {
                        cannot_block: format_task_brief(list, b),
                        requested_dependency: format_task_brief(list, a),
                    })
                }
                Err(BlockError::WouldBlockOnSelf) => Ok(so_far),
            }
        })
        .map_err(|e| vec![e])?
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id)
                .action(*actions.get(&id).unwrap_or(&Action::None))
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}
