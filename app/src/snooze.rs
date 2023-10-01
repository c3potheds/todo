use {
    super::util::{
        format_task, format_task_brief, lookup_tasks, parse_snooze_date,
    },
    chrono::{DateTime, Utc},
    todo_cli::Snooze,
    todo_model::{SnoozeWarning, TaskId, TaskSet, TodoList},
    todo_printing::{
        Action, PrintableAppSuccess, PrintableError, PrintableResult,
        PrintableWarning,
    },
};

fn format_snooze_warning(
    list: &TodoList,
    id: TaskId,
    warning: SnoozeWarning,
) -> PrintableWarning {
    match warning {
        SnoozeWarning::TaskIsComplete => {
            PrintableWarning::CannotSnoozeBecauseComplete {
                cannot_snooze: format_task_brief(list, id),
            }
        }
        SnoozeWarning::SnoozedUntilAfterDueDate {
            snoozed_until,
            due_date,
        } => PrintableWarning::SnoozedAfterDueDate {
            snoozed_task: format_task_brief(list, id),
            due_date,
            snooze_date: snoozed_until,
        },
    }
}

pub fn run<'list>(
    list: &'list mut TodoList,
    now: DateTime<Utc>,
    cmd: &Snooze,
) -> PrintableResult<'list> {
    let snooze_date = parse_snooze_date(now, &cmd.until)
        .and_then(|date| match date {
            Some(date) => Ok(date),
            None => Err(PrintableError::EmptyDate {
                flag: Some("--until".to_string()),
            }),
        })
        .map_err(|e| vec![e])?;
    // If the snooze date has already passed, we don't need to do anything. Just
    // print the tasks and a warning indicating that the snooze date has already
    // passed.
    if snooze_date <= now {
        let warnings = lookup_tasks(list, &cmd.keys)
            .iter_sorted(list)
            .map(|id| PrintableWarning::SnoozedUntilPast {
                snoozed_task: format_task_brief(list, id),
                snooze_date,
            })
            .collect();
        return Ok(PrintableAppSuccess {
            warnings,
            tasks: lookup_tasks(list, &cmd.keys)
                .iter_sorted(list)
                .map(|id| format_task(list, id))
                .collect(),
            ..Default::default()
        });
    }
    let (snoozed, warnings, mutated) =
        lookup_tasks(list, &cmd.keys).iter_sorted(list).fold(
            (TaskSet::default(), Vec::new(), false),
            |(mut snoozed, mut warnings, mut mutated), id| {
                match list.snooze(id, snooze_date) {
                    Ok(()) => {
                        mutated = true;
                        snoozed.push(id);
                    }
                    Err(new_warnings) => {
                        warnings.extend(
                            new_warnings
                                .into_iter()
                                .inspect(|w| {
                                    if let
                                    SnoozeWarning::SnoozedUntilAfterDueDate{
                                        snoozed_until: _, due_date: _
                                    } = w {
                                        mutated = true;
                                        snoozed.push(id);
                                    }
                                })
                                .map(|w| format_snooze_warning(list, id, w)),
                        );
                    }
                };
                (snoozed, warnings, mutated)
            },
        );
    let tasks_to_print = snoozed
        .iter_sorted(list)
        .map(|id| format_task(list, id).action(Action::Snooze))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        warnings,
        mutated,
        ..Default::default()
    })
}
