use app::util::format_task;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter) {
    model.complete_tasks().for_each(|id| {
        printer.print_task(&format_task(model, id, Action::None))
    });
}
