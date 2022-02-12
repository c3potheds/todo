use app::util::format_task;
use app::util::lookup_tasks;
use app::util::should_include_done;
use cli::Get;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter, cmd: &Get) {
    let requested_tasks = lookup_tasks(model, &cmd.keys);
    let include_done = should_include_done(
        cmd.include_done,
        model,
        requested_tasks.iter_unsorted(),
    );
    requested_tasks
        .iter_sorted(model)
        .fold(requested_tasks.clone(), |so_far, id| {
            so_far | model.transitive_deps(id) | model.transitive_adeps(id)
        })
        .include_done(model, include_done)
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id).action(
                if requested_tasks.contains(id) {
                    Action::Select
                } else {
                    Action::None
                },
            ))
        });
}
