use app::util::format_task;
use app::util::lookup_tasks;
use app::util::should_include_done;
use cli::Get;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

pub fn run(list: &TodoList, printer: &mut impl TodoPrinter, cmd: &Get) {
    let requested_tasks = lookup_tasks(list, &cmd.keys);
    let include_done = should_include_done(
        cmd.include_done,
        list,
        requested_tasks.iter_unsorted(),
    );
    requested_tasks
        .iter_sorted(list)
        .fold(requested_tasks.clone(), |so_far, id| {
            so_far | list.transitive_deps(id) | list.transitive_adeps(id)
        })
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
