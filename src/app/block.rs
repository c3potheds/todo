use app::util::format_task;
use app::util::lookup_tasks;
use cli::Block;
use itertools::Itertools;
use model::TodoList;
use printing::Action;
use printing::PrintingContext;
use printing::TodoPrinter;

pub fn run(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Block,
) {
    lookup_tasks(&model, &cmd.keys)
        .into_iter()
        .cartesian_product(lookup_tasks(&model, &cmd.on).into_iter())
        .filter(|&(blocked, blocking)| {
            model.block(blocked).on(blocking).is_ok()
        })
        .map(|(blocked, _)| blocked)
        .unique()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(
                printing_context,
                model,
                id,
                Action::None,
            ));
        });
}