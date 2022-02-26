use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use cli::Punt;
use model::PuntError;
use model::TodoList;
use printing::Action;
use printing::PrintableWarning;
use printing::TodoPrinter;

pub fn run(list: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Punt) {
    lookup_tasks(list, &cmd.keys)
        .iter_sorted(list)
        .filter(|&id| match list.punt(id) {
            Err(PuntError::TaskIsComplete) => {
                printer.print_warning(
                    &PrintableWarning::CannotPuntBecauseComplete {
                        cannot_punt: format_task_brief(list, id),
                    },
                );
                false
            }
            _ => true,
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(list, id).action(Action::Punt))
        });
}
