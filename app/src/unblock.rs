use {
    super::util::{
        format_task, format_task_brief, lookup_tasks, should_include_done,
    },
    cli::Unblock,
    model::{TaskSet, TodoList},
    printing::{
        Action, PrintableAppSuccess, PrintableError, PrintableResult,
        PrintableWarning,
    },
};

fn unblock_from_given(
    list: &mut TodoList,
    tasks_to_unblock: &TaskSet,
    tasks_to_unblock_from: &TaskSet,
) -> (TaskSet, Vec<PrintableWarning>) {
    tasks_to_unblock.product(tasks_to_unblock_from, list).fold(
        (TaskSet::default(), Vec::new()),
        |(affected_so_far, mut warnings_so_far), (blocked, blocking)| match list
            .unblock(blocked)
            .from(blocking)
        {
            Ok(affected) => (affected_so_far | affected, warnings_so_far),
            Err(_) => {
                warnings_so_far.push(
                    PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                        cannot_unblock: format_task_brief(list, blocked),
                        requested_unblock_from: format_task_brief(
                            list, blocking,
                        ),
                    },
                );
                (affected_so_far, warnings_so_far)
            }
        },
    )
}

fn unblock_from_all(
    list: &mut TodoList,
    tasks_to_unblock: &TaskSet,
) -> TaskSet {
    tasks_to_unblock
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            so_far
                | list
                    .deps(id)
                    .iter_unsorted()
                    .fold(TaskSet::default(), |so_far, dep| {
                        list.unblock(id).from(dep).unwrap() | so_far
                    })
        })
}

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Unblock,
) -> PrintableResult<'list> {
    let tasks_to_unblock = lookup_tasks(list, &cmd.keys);
    let tasks_to_unblock_from = lookup_tasks(list, &cmd.from);
    let include_done = should_include_done(
        cmd.include_done,
        list,
        (tasks_to_unblock.clone() | tasks_to_unblock_from.clone())
            .iter_unsorted(),
    );
    if !cmd.from.is_empty() && tasks_to_unblock_from.is_empty() {
        return Err(vec![PrintableError::NoMatchForKeys {
            keys: cmd.from.clone(),
        }]);
    }
    let mut mutated = false;
    let (affected_tasks, warnings) = if tasks_to_unblock_from.is_empty() {
        (unblock_from_all(list, &tasks_to_unblock), Vec::new())
    } else {
        unblock_from_given(list, &tasks_to_unblock, &tasks_to_unblock_from)
    };
    let tasks_to_print = affected_tasks
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id).action(if tasks_to_unblock.contains(id) {
                mutated = true;
                Action::Unlock
            } else {
                Action::None
            })
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        warnings,
        mutated,
    })
}
