use app::util::format_task;
use app::util::lookup_tasks;
use cli::Get;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintingContext;
use printing::TodoPrinter;

pub fn run(
    model: &TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Get,
) {
    let requested_tasks = lookup_tasks(model, &cmd.keys);
    requested_tasks
        .iter()
        .copied()
        .flat_map(|id| {
            (model.transitive_deps(id) | model.transitive_adeps(id))
                .iter_unsorted()
                .chain(std::iter::once(id))
        })
        .collect::<TaskSet>()
        .iter_sorted(&model)
        .for_each(|id| {
            printer.print_task(&format_task(
                printing_context,
                model,
                id,
                if requested_tasks.contains(&id) {
                    Action::Select
                } else {
                    Action::None
                },
            ))
        });
}
