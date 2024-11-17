use chrono::DateTime;
use chrono::Utc;
use todo_cli::Snooze;
use todo_model::SnoozeWarning;
use todo_model::TaskId;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableError;
use todo_printing::PrintableResult;
use todo_printing::PrintableWarning;

use super::util::format_task;
use super::util::format_task_brief;
use super::util::lookup_tasks;
use super::util::parse_snooze_date;

fn format_snooze_warning(
    list: &TodoList,
    id: TaskId,
    warning: SnoozeWarning,
) -> PrintableWarning {
    match warning {
        SnoozeWarning::TaskNotFound { id } => {
            // This should never happen, as we've already looked up the task.
            unreachable!("Task not found: {id:?}");
        }
        SnoozeWarning::TaskIsComplete => {
            PrintableWarning::CannotSnoozeBecauseComplete {
                cannot_snooze: format_task_brief(list, id),
            }
        }
        SnoozeWarning::TaskIsAlreadySnoozed {
            current_snooze,
            requested_snooze,
        } => PrintableWarning::AlreadySnoozedAfterRequestedTime {
            snoozed_task: format_task_brief(list, id),
            requested_snooze_date: requested_snooze,
            snooze_date: current_snooze,
        },
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
                                    if let SnoozeWarning::TaskIsAlreadySnoozed {
                                        current_snooze: _,
                                        requested_snooze: _,
                                    } = w {
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
