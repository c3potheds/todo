use {
    super::util::{format_task, format_task_brief, lookup_tasks},
    todo_cli::Unsnooze,
    todo_model::{TaskSet, TodoList, UnsnoozeWarning},
    todo_printing::{
        Action, PrintableAppSuccess, PrintableResult, PrintableWarning,
    },
};

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Unsnooze,
) -> PrintableResult<'list> {
    let (tasks_to_print, warnings) =
        lookup_tasks(list, &cmd.keys).iter_sorted(list).fold(
            (TaskSet::default(), Vec::new()),
            |(mut tasks_to_print, mut warnings), id| {
                match list.unsnooze(id) {
                    Ok(()) => {
                        tasks_to_print.push(id);
                    }
                    Err(w) => {
                        warnings
                            .extend(w.into_iter().map(|warning| (id, warning)));
                    }
                }
                (tasks_to_print, warnings)
            },
        );
    let formatted_tasks_to_print = tasks_to_print
        .iter_sorted(list)
        .map(|id| format_task(list, id).action(Action::Unsnooze))
        .collect();
    let formatted_warnings = warnings
        .into_iter()
        .map(|(id, warning)| {
            use self::UnsnoozeWarning::*;
            match warning {
                TaskIsComplete => {
                    PrintableWarning::CannotUnsnoozeBecauseComplete(
                        format_task_brief(list, id),
                    )
                }
                NotSnoozed => {
                    PrintableWarning::CannotUnsnoozeBecauseNotSnoozed(
                        format_task_brief(list, id),
                    )
                }
            }
        })
        .collect();
    let mutated = !tasks_to_print.is_empty();
    Ok(PrintableAppSuccess {
        tasks: formatted_tasks_to_print,
        warnings: formatted_warnings,
        mutated,
        ..Default::default()
    })
}
