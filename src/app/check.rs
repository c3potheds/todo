use app::util::format_task;
use app::util::lookup_tasks;
use cli::Check;
use model::TaskId;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::PrintingContext;
use printing::TodoPrinter;
use std::collections::HashSet;

fn print_check_error(
    printer: &mut impl TodoPrinter,
    model: &TodoList,
    id: TaskId,
) {
    model.position(id).map(|cannot_check| {
        let blocked_by = model
            .deps(id)
            .into_iter()
            .filter(|&dep| model.status(dep) != Some(TaskStatus::Complete))
            .flat_map(|dep| model.position(dep).into_iter())
            .collect();
        printer.print_error(&PrintableError::CannotCheckBecauseBlocked {
            cannot_check: cannot_check,
            blocked_by: blocked_by,
        });
    });
}

pub fn run(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Check,
) {
    let tasks_to_check = lookup_tasks(model, &cmd.keys);
    let tasks_to_print = tasks_to_check
        .iter()
        .copied()
        .flat_map(|id| match model.check(id) {
            Ok(mut unlocked) => {
                unlocked.insert(id);
                unlocked.into_iter()
            }
            Err(_) => {
                print_check_error(printer, model, id);
                HashSet::new().into_iter()
            }
        })
        .collect::<HashSet<_>>();
    model
        .all_tasks()
        .filter(|id| tasks_to_print.contains(id))
        .for_each(|id| {
            printer.print_task(&format_task(
                printing_context,
                model,
                id,
                if tasks_to_check.contains(&id) {
                    Action::Check
                } else {
                    Action::Unlock
                },
            ))
        });
}
