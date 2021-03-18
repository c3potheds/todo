use app::util::format_task;
use app::util::lookup_tasks;
use cli::Restore;
use model::RestoreError;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::PrintableWarning;
use printing::TodoPrinter;
use std::collections::HashSet;

fn print_cannot_restore_because_adeps_are_complete_error(
    model: &TodoList,
    printer: &mut impl TodoPrinter,
    id: TaskId,
    would_restore: Vec<TaskId>,
) {
    printer.print_error(
        &PrintableError::CannotRestoreBecauseAntidependencyIsComplete {
            cannot_restore: model.position(id).unwrap(),
            complete_antidependencies: would_restore
                .into_iter()
                .map(|adep| model.position(adep).unwrap())
                .collect(),
        },
    );
}

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Restore,
) {
    let tasks_to_restore = lookup_tasks(&model, &cmd.keys);
    let tasks_to_print = tasks_to_restore
        .iter()
        .fold(HashSet::new(), |mut so_far, &id| match model.restore(id) {
            Ok(blocked) => {
                so_far.insert(id);
                blocked.iter_unsorted().for_each(|id| {
                    so_far.insert(id);
                });
                so_far
            }
            Err(RestoreError::TaskIsAlreadyIncomplete) => {
                printer.print_warning(
                    &PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                        cannot_restore: model.position(id).unwrap(),
                    },
                );
                so_far
            }
            Err(RestoreError::WouldRestore(would_restore)) => {
                print_cannot_restore_because_adeps_are_complete_error(
                    model,
                    printer,
                    id,
                    would_restore,
                );
                so_far
            }
        })
        .into_iter()
        .collect::<TaskSet>();
    tasks_to_print.iter_sorted(model).for_each(|id| {
        printer.print_task(&format_task(
            model,
            id,
            if tasks_to_restore.contains(&id) {
                Action::Uncheck
            } else {
                Action::Lock
            },
        ))
    });
}
