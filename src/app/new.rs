use app::util::format_task;
use cli::New;
use model::Task;
use model::TodoList;
use printing::Action;
use printing::PrintingContext;
use printing::TodoPrinter;

pub fn run(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &New,
) {
    cmd.desc
        .iter()
        .map(|desc| model.add(Task::new(desc)))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(
                printing_context,
                model,
                id,
                Action::New,
            ))
        });
}
