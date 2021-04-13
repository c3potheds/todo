use app::util::format_task;
use app::util::lookup_tasks;
use chrono::DateTime;
use chrono::Utc;
use cli::Punt;
use model::PuntError;
use model::TodoList;
use printing::Action;
use printing::PrintableWarning;
use printing::TodoPrinter;

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
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
            printer
                .print_task(&format_task(&model, id, now).action(Action::Punt))
        });
}
