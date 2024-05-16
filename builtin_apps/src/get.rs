use crate::util::format_task;
use crate::util::lookup_tasks_by_keys;
use crate::util::should_include_done;
use std::collections::HashMap;
use todo_cli::Get;
use todo_lookup_key::Key;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;

fn disambiguate(
    list: &TodoList,
    key_task_map: HashMap<&Key, TaskSet>,
    include_done: bool,
) -> TaskSet {
    if include_done {
        return key_task_map
            .into_values()
            .fold(TaskSet::default(), |so_far, tasks| so_far | tasks);
    }
    key_task_map
        .into_values()
        .fold(TaskSet::default(), |so_far, tasks| {
            let (complete, incomplete) = tasks.partition_done(list);
            if !complete.is_empty() && !incomplete.is_empty() {
                so_far | incomplete
            } else {
                so_far | complete | incomplete
            }
        })
}

pub fn run<'list>(list: &'list TodoList, cmd: &Get) -> PrintableResult<'list> {
    let requested_tasks = disambiguate(
        list,
        lookup_tasks_by_keys(list, &cmd.keys),
        cmd.include_done,
    );
    let include_done = should_include_done(
        cmd.include_done,
        list,
        requested_tasks.iter_unsorted(),
    );
    let (include_deps, include_adeps) =
        match (cmd.blocking, cmd.blocked_by, cmd.no_context) {
            // --blocking alone should show deps and not adeps.
            (true, false, false) => (true, false),
            // --blocked-by alone should show adeps and not deps.
            (false, true, false) => (false, true),
            // --no-context alone should show neither deps nor adeps.
            (false, false, true) => (false, false),
            // By default, show all context.
            (false, false, false) => (true, true),
            // Any other combination is invalid.
            _ => unreachable!(),
        };
    let tasks_to_print = requested_tasks
        .iter_sorted(list)
        .fold(requested_tasks.clone(), |so_far, id| {
            so_far
                | if include_deps {
                    list.transitive_deps(id)
                } else {
                    TaskSet::default()
                }
                | if include_adeps {
                    list.transitive_adeps(id)
                } else {
                    TaskSet::default()
                }
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| {
            let task = format_task(list, id);
            if requested_tasks.contains(id) {
                task.action(Action::Select).truncate_tags_if_needed(false)
            } else {
                task.action(Action::None)
            }
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}
