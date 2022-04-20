use {
    super::util::{format_task, lookup_task, should_include_done},
    crate::cli::Top,
    model::{TaskSet, TaskStatus, TodoList},
    printing::{PrintableWarning, TodoPrinter},
};

pub fn run(list: &TodoList, printer: &mut impl TodoPrinter, cmd: &Top) {
    // Handle the case where no tasks are specified. In this case, we want to
    // print all tasks that do not have any antidependencies (including complete
    // tasks iff '--include_done' is passed).)
    if cmd.keys.is_empty() {
        list.all_tasks()
            .filter(|&id| {
                cmd.include_done
                    || list.status(id) != Some(TaskStatus::Complete)
            })
            .filter(|&id| list.adeps(id).is_empty())
            .for_each(|id| {
                printer.print_task(&format_task(list, id));
            });
        return;
    }

    // Handle the case where tasks are specified. If no matches are found, print
    // warnings. The top tasks for the specified tasks are deps that directly
    // block the specified tasks, and do not have any antidependencies that
    // themselves directly or indirectly block the specified tasks. In other
    // words, the top tasks for a given task are the ones that, if completed,
    // would unblock the given task.
    let tasks = cmd.keys.iter().fold(TaskSet::default(), |so_far, key| {
        let tasks = lookup_task(list, key);
        if tasks.is_empty() {
            printer.print_warning(&PrintableWarning::NoMatchFoundForKey {
                requested_key: key.clone(),
            });
        }
        so_far | tasks
    });
    let include_done =
        should_include_done(cmd.include_done, list, tasks.iter_unsorted());
    tasks
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
        .for_each(|id| {
            printer.print_task(&format_task(list, id));
        });
}
