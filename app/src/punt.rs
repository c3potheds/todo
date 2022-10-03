use {
    super::util::{format_task, format_task_brief, lookup_tasks},
    cli::Punt,
    model::{PuntError, TaskSet, TodoList},
    printing::{
        Action, PrintableAppSuccess, PrintableResult, PrintableWarning,
    },
};

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
                        punted_tasks = punted_tasks | TaskSet::of(id);
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
    })
}
