use cli::Bottom;
use model::{TaskId, TaskSet, TaskStatus, TodoList};
use printing::{PrintableAppSuccess, PrintableResult, PrintableWarning};

use crate::util::{format_task, lookup_task, should_include_done};

fn bottom_underneath(
    list: &TodoList,
    id: TaskId,
    include_done: bool,
) -> TaskSet {
    list.adeps(id)
        .include_done(list, include_done)
        .iter_unsorted()
        .filter(|&adep| {
            !list
                .deps(adep)
                .iter_unsorted()
                .any(|dep| list.transitive_deps(dep).contains(id))
        })
        .collect()
}

pub fn run<'list>(
    list: &'list TodoList,
    cmd: &Bottom,
) -> PrintableResult<'list> {
    if cmd.keys.is_empty() {
        let tasks = list
            .all_tasks()
            .filter(|&id| {
                cmd.include_done
                    || list.status(id) != Some(TaskStatus::Complete)
            })
            .filter(|&id| list.deps(id).is_empty())
            .map(|id| format_task(list, id))
            .collect();
        return Ok(PrintableAppSuccess {
            tasks,
            ..Default::default()
        });
    }
    let (tasks_to_query, warnings) = cmd.keys.iter().fold(
        (TaskSet::default(), Vec::new()),
        |(so_far, mut warnings), key| {
            let found_tasks = lookup_task(list, key);
            if found_tasks.is_empty() {
                warnings.push(PrintableWarning::NoMatchFoundForKey {
                    requested_key: key.clone(),
                });
            }
            (so_far | found_tasks, warnings)
        },
    );
    let include_done = should_include_done(
        cmd.include_done,
        list,
        tasks_to_query.iter_unsorted(),
    );
    let tasks = tasks_to_query
        .iter_unsorted()
        // For each matching task, find the bottom tasks that they directly
        // block, a.k.a their direct antidependencies.
        .fold(TaskSet::default(), |so_far, id| {
            so_far | bottom_underneath(list, id, include_done)
        })
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        warnings,
        tasks,
        ..Default::default()
    })
}
