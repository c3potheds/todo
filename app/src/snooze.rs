use {
    super::util::{
        format_task, format_task_brief, lookup_tasks,
        parse_snooze_date_or_print_error,
    },
    chrono::{DateTime, Utc},
    cli::Snooze,
    model::{SnoozeWarning, TaskSet, TodoList},
    printing::{Action, PrintableError, PrintableWarning, TodoPrinter},
};

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Snooze,
) -> bool {
    let snooze_date =
        match parse_snooze_date_or_print_error(now, &cmd.until, printer) {
            Ok(Some(snooze_date)) => snooze_date,
            Ok(None) => {
                printer.print_error(&PrintableError::EmptyDate {
                    flag: Some("--until".to_string()),
                });
                return false;
            }
            Err(()) => {
                return false;
            }
        };
    let mut mutated = false;
    lookup_tasks(list, &cmd.keys)
        .iter_sorted(list)
        .filter(|&id| match list.snooze(id, snooze_date) {
            Ok(()) => {
                mutated = true;
                true
            }
            Err(warnings) => warnings.into_iter().fold(
                true,
                |snoozed, warning| match warning {
                    SnoozeWarning::TaskIsComplete => {
                        printer.print_warning(
                            &PrintableWarning::CannotSnoozeBecauseComplete {
                                cannot_snooze: format_task_brief(list, id),
                            },
                        );
                        false
                    }
                    SnoozeWarning::SnoozedUntilAfterDueDate {
                        snoozed_until,
                        due_date,
                    } => {
                        printer.print_warning(
                            &PrintableWarning::SnoozedAfterDueDate {
                                snoozed_task: format_task_brief(list, id),
                                due_date,
                                snooze_date: snoozed_until,
                            },
                        );
                        snoozed
                    }
                },
            ),
        })
        .collect::<TaskSet>()
        .iter_sorted(list)
        .for_each(|id| {
            printer.print_task(
                &format_task(list, id)
                    .action(Action::Snooze)
                    .start_date(snooze_date),
            )
        });
    mutated
}
