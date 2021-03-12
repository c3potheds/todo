use app::util::format_task;
use app::util::lookup_tasks;
use cli::Punt;
use model::PuntError;
use model::TodoList;
use printing::Action;
use printing::PrintableWarning;
use printing::PrintingContext;
use printing::TodoPrinter;

pub fn run(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Punt,
) {
    lookup_tasks(&model, &cmd.keys)
        .into_iter()
        .filter(|&id| match model.punt(id) {
            Err(PuntError::TaskIsComplete) => {
                model.position(id).map(|n| {
                    printer.print_warning(
                        &PrintableWarning::CannotPuntBecauseComplete {
                            cannot_punt: n,
                        },
                    )
                });
                false
            }
            _ => true,
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(
                printing_context,
                &model,
                id,
                Action::Punt,
            ))
        });
}
