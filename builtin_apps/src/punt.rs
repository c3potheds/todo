use todo_cli::Punt;
use todo_model::PuntError;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;
use todo_printing::PrintableWarning;

use super::util::format_task;
use super::util::format_task_brief;
use super::util::lookup_tasks;

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Punt,
) -> PrintableResult<'list> {
    let (punted_tasks, warnings, mutated) =
        lookup_tasks(list, &cmd.keys).iter_sorted(list).fold(
            (TaskSet::default(), Vec::new(), false),
            |(mut punted_tasks, mut warnings, mut mutated), id| {
                match list.punt(id) {
                    Err(PuntError::TaskIsComplete) => {
                        warnings.push(
                            PrintableWarning::CannotPuntBecauseComplete {
                                cannot_punt: format_task_brief(list, id),
                            },
                        );
                    }
                    _ => {
                        mutated = true;
                        punted_tasks.push(id);
                    }
                }
                (punted_tasks, warnings, mutated)
            },
        );
    let tasks_to_print = punted_tasks
        .iter_sorted(list)
        .map(|id| format_task(list, id).action(Action::Punt))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        warnings,
        mutated,
        ..Default::default()
    })
}
