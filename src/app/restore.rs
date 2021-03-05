use app::util::format_task;
use app::util::lookup_tasks;
use cli::Restore;
use model::TodoList;
use printing::PrintingContext;
use printing::TodoPrinter;

pub fn run(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Restore,
) {
    lookup_tasks(&model, &cmd.keys)
        .into_iter()
        .filter(|&id| model.restore(id).is_ok())
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(printing_context, model, id))
        });
}
