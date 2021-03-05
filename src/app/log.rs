use app::util::format_task;
use model::TodoList;
use printing::Action;
use printing::PrintingContext;
use printing::TodoPrinter;

pub fn run(
    model: &TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
) {
    model.complete_tasks().for_each(|id| {
        printer.print_task(&format_task(
            printing_context,
            model,
            id,
            Action::None,
        ))
    });
}
