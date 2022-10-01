use cli::Bottom;
use model::{TaskSet, TaskStatus, TodoList};
use printing::{PrintableWarning, TodoPrinter};

use crate::util::{format_task, lookup_task, should_include_done};

pub fn run(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Bottom,
) -> bool {
    if cmd.keys.is_empty() {
        list.all_tasks()
            .filter(|&id| {
                cmd.include_done
                    || list.status(id) != Some(TaskStatus::Complete)
            })
            .filter(|&id| list.deps(id).is_empty())
            .for_each(|id| {
                printer.print_task(&format_task(list, id));
            });
        return false;
    }
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
        // For each matching task, find the bottom tasks that they directly block.
        // a.k.a their direct antidependencies.
        .fold(TaskSet::default(), |so_far, id| {
            list.adeps(id)
                .include_done(list, include_done)
                .iter_unsorted()
                .filter(|&adep| {
                    !list
                        .deps(adep)
                        .iter_unsorted()
                        .any(|dep| list.transitive_deps(dep).contains(id))
                })
                .collect::<TaskSet>()
                | so_far
        })
        .iter_sorted(list)
        .for_each(|id| {
            printer.print_task(&format_task(list, id));
        });
    false
}
