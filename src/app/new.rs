use app::util::format_task;
use app::util::lookup_tasks;
use cli::New;
use itertools::Itertools;
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
    let new_tasks: Vec<_> = cmd
        .desc
        .iter()
        .map(|desc| model.add(Task::new(desc)))
        .collect();
    let deps = lookup_tasks(&model, &cmd.blocked_by);
    let adeps = lookup_tasks(&model, &cmd.blocking);
    deps.iter()
        .copied()
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(dep, new)| {
            // TODO: print a warning, but continue in the error case.
            model.block(new).on(dep).expect("Cannot block");
        });
    adeps
        .iter()
        .copied()
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(adep, new)| {
            // TODO: print a warning, but continue in the error case.
            model.block(adep).on(new).expect("Cannot block");
        });
    deps.into_iter().for_each(|id| {
        printer.print_task(&format_task(
            printing_context,
            model,
            id,
            Action::None,
        ))
    });
    new_tasks.into_iter().for_each(|id| {
        printer.print_task(&format_task(
            printing_context,
            model,
            id,
            Action::New,
        ))
    });
    adeps.into_iter().for_each(|id| {
        printer.print_task(&format_task(
            printing_context,
            model,
            id,
            Action::None,
        ))
    });
}
