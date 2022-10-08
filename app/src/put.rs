use {
    super::util::{
        format_task, format_task_brief, lookup_tasks, should_include_done,
    },
    cli::Put,
    model::{TaskId, TaskSet, TodoList},
    printing::{Action, PrintableAppSuccess, PrintableError, PrintableResult},
    std::collections::HashSet,
};

fn print_block_error(
    list: &TodoList,
    blocked: TaskId,
    blocking: TaskId,
) -> PrintableError {
    PrintableError::CannotBlockBecauseWouldCauseCycle {
        cannot_block: format_task_brief(list, blocked),
        requested_dependency: format_task_brief(list, blocking),
    }
}

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Put,
) -> PrintableResult<'list> {
    let tasks_to_put = lookup_tasks(list, &cmd.keys);
    let before = lookup_tasks(list, &cmd.preposition.before);
    let after = lookup_tasks(list, &cmd.preposition.after);
    let include_done = should_include_done(
        cmd.include_done,
        list,
        (tasks_to_put.clone() | before.clone() | after.clone()).iter_unsorted(),
    );
    let before_deps: TaskSet = before
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.deps(id));
    let after_adeps: TaskSet = after
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.adeps(id));
    let tasks_to_block_on = after | before_deps;
    let tasks_to_block = before | after_adeps;
    let mut mutated = false;

    let mut blocked_tasks = HashSet::new();
    let tasks_to_print = tasks_to_put
        .product(&tasks_to_block_on, list)
        .chain(
            tasks_to_put
                .product(&tasks_to_block, list)
                .map(|(a, b)| (b, a)),
        )
        .try_fold(TaskSet::default(), |so_far, (blocked, blocking)| match list
            .block(blocked)
            .on(blocking)
        {
            Ok(affected) => {
                mutated = true;
                blocked_tasks.insert(blocked);
                Ok(so_far | affected)
            }
            Err(_) => Err(vec![print_block_error(list, blocked, blocking)]),
        })?
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id).action(if blocked_tasks.contains(&id) {
                Action::Lock
            } else {
                Action::None
            })
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}
