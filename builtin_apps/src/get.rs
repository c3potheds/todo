use {
    super::util::{format_task, lookup_tasks, should_include_done},
    todo_cli::Get,
    todo_model::{TaskSet, TodoList},
    todo_printing::{Action, PrintableAppSuccess, PrintableResult},
};

pub fn run<'list>(list: &'list TodoList, cmd: &Get) -> PrintableResult<'list> {
    let requested_tasks = lookup_tasks(list, &cmd.keys);
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
            format_task(list, id).action(if requested_tasks.contains(id) {
                Action::Select
            } else {
                Action::None
            })
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}
