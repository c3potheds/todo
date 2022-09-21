use {
    super::util::{format_task, lookup_tasks, should_include_done},
    cli::Get,
    model::TodoList,
    printing::{Action, TodoPrinter},
};

pub fn run(list: &TodoList, printer: &mut dyn TodoPrinter, cmd: &Get) {
    let requested_tasks = lookup_tasks(list, &cmd.keys);
    let include_done = should_include_done(
        cmd.include_done,
        list,
        requested_tasks.iter_unsorted(),
    );
    if cmd.no_context {
        requested_tasks.clone()
    } else {
        requested_tasks.iter_sorted(list).fold(
            requested_tasks.clone(),
            |so_far, id| {
                so_far | list.transitive_deps(id) | list.transitive_adeps(id)
            },
        )
    }
    .include_done(list, include_done)
    .iter_sorted(list)
    .for_each(|id| {
        printer.print_task(&format_task(list, id).action(
            if requested_tasks.contains(id) {
                Action::Select
            } else {
                Action::None
            },
        ))
    });
}
