use {
    super::util::{
        format_task, format_task_brief, format_tasks_brief, lookup_tasks,
    },
    crate::cli::Unsnooze,
    model::{TaskId, TaskSet, TodoList, UnsnoozeWarning},
    printing::{Action, PrintableWarning, TodoPrinter},
};

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Unsnooze,
) {
    #[derive(Default)]
    struct UnsnoozeResult {
        tasks_to_print: TaskSet,
        warnings: Vec<(TaskId, UnsnoozeWarning)>,
    }
    let UnsnoozeResult {
        tasks_to_print,
        warnings,
    } = lookup_tasks(list, &cmd.keys).iter_sorted(list).fold(
        UnsnoozeResult::default(),
        |mut result, id| {
            match list.unsnooze(id) {
                Ok(()) => {
                    result.tasks_to_print =
                        result.tasks_to_print | TaskSet::of(id);
                }
                Err(warnings) => {
                    result.warnings.extend(
                        warnings.into_iter().map(|warning| (id, warning)),
                    );
                }
            }
            result
        },
    );
    warnings.into_iter().for_each(|(id, warning)| {
        use self::UnsnoozeWarning::*;
        printer.print_warning(&match warning {
            TaskIsComplete => PrintableWarning::CannotUnsnoozeBecauseComplete(
                format_task_brief(list, id),
            ),
            TaskIsBlocked => PrintableWarning::CannotUnsnoozeBecauseBlocked {
                cannot_unsnooze: format_task_brief(list, id),
                blocked_by: format_tasks_brief(
                    list,
                    &list.deps(id).include_done(list, false),
                ),
            },
            NotSnoozed => PrintableWarning::CannotUnsnoozeBecauseNotSnoozed(
                format_task_brief(list, id),
            ),
        })
    });
    tasks_to_print.iter_sorted(list).for_each(|id| {
        printer.print_task(&format_task(list, id).action(Action::Unsnooze));
    });
}
