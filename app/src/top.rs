use {
    super::util::{format_task, lookup_task, should_include_done},
    cli::Top,
    model::{TaskSet, TaskStatus, TodoList},
    printing::{PrintableAppSuccess, PrintableResult, PrintableWarning},
};

pub fn run<'list>(list: &'list TodoList, cmd: &Top) -> PrintableResult<'list> {
    // Handle the case where no tasks are specified. In this case, we want to
    // print all tasks that do not have any antidependencies (including complete
    // tasks iff '--include_done' is passed).)
    if cmd.keys.is_empty() {
        let tasks_to_print = list
            .all_tasks()
            .filter(|&id| {
                cmd.include_done
                    || list.status(id) != Some(TaskStatus::Complete)
            })
            .filter(|&id| list.adeps(id).is_empty())
            .map(|id| format_task(list, id))
            .collect();
        return Ok(PrintableAppSuccess {
            tasks: tasks_to_print,
            ..Default::default()
        });
    }

    // Handle the case where tasks are specified. If no matches are found, print
    // warnings. The top tasks for the specified tasks are deps that directly
    // block the specified tasks, and do not have any antidependencies that
    // themselves directly or indirectly block the specified tasks. In other
    // words, the top tasks for a given task are the ones that, if completed,
    // would unblock the given task.
    let mut warnings = Vec::new();
    let tasks = cmd.keys.iter().fold(TaskSet::default(), |so_far, key| {
        let tasks = lookup_task(list, key);
        if tasks.is_empty() {
            warnings.push(PrintableWarning::NoMatchFoundForKey {
                requested_key: key.clone(),
            });
        }
        so_far | tasks
    });
    let include_done =
        should_include_done(cmd.include_done, list, tasks.iter_unsorted());
    let tasks_to_print = tasks
        .iter_unsorted()
        // For each matching task, find the top tasks that directly block it.
        .fold(TaskSet::default(), |so_far, id| {
            list.deps(id)
                .include_done(list, include_done)
                .iter_unsorted()
                .filter(|&dep| {
                    !list
                        .adeps(dep)
                        .iter_unsorted()
                        .any(|adep| list.transitive_adeps(adep).contains(id))
                })
                .collect::<TaskSet>()
                | so_far
        })
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        warnings,
        ..Default::default()
    })
}
