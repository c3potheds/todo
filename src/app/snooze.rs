use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use app::util::parse_snooze_date_or_print_error;
use chrono::DateTime;
use chrono::Utc;
use cli::Snooze;
use model::SnoozeWarning;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableWarning;
use printing::TodoPrinter;

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Snooze,
) {
    let snooze_date =
        match parse_snooze_date_or_print_error(now, &cmd.until, printer) {
            Ok(snooze_date) => snooze_date,
            Err(()) => {
                return;
            }
        };
    lookup_tasks(list, &cmd.keys)
        .iter_sorted(list)
        .filter(|&id| match list.snooze(id, snooze_date) {
            Ok(()) => true,
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
                                due_date: due_date,
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
}
