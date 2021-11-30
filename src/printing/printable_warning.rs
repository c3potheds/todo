use super::format_util::format_key;
use super::format_util::format_numbers;
use super::BriefPrintableTask;
use ansi_term::Color;
use chrono::DateTime;
use chrono::Utc;
use cli::Key;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum PrintableWarning {
    NoMatchFoundForKey {
        requested_key: Key,
    },
    CannotCheckBecauseAlreadyComplete {
        cannot_check: BriefPrintableTask,
    },
    CannotRestoreBecauseAlreadyIncomplete {
        cannot_restore: BriefPrintableTask,
    },
    CannotUnblockBecauseTaskIsNotBlocked {
        cannot_unblock: BriefPrintableTask,
        requested_unblock_from: BriefPrintableTask,
    },
    CannotPuntBecauseComplete {
        cannot_punt: BriefPrintableTask,
    },
    CannotSnoozeBecauseComplete {
        cannot_snooze: BriefPrintableTask,
    },
    SnoozedAfterDueDate {
        snoozed_task: BriefPrintableTask,
        due_date: DateTime<Utc>,
        snooze_date: DateTime<Utc>,
    },
    AmbiguousKey {
        key: Key,
        matches: Vec<BriefPrintableTask>,
    },
    NoPathFoundBetween(BriefPrintableTask, BriefPrintableTask),
}

impl Display for PrintableWarning {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            format!("{}", Color::Yellow.bold().paint("warning")),
            match self {
                PrintableWarning::NoMatchFoundForKey { requested_key } =>
                    format!("No match found for {}", format_key(requested_key)),
                PrintableWarning::CannotCheckBecauseAlreadyComplete {
                    cannot_check,
                } => format!("Task {} is already complete", cannot_check),
                PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                    cannot_restore,
                } => format!("Task {} is already incomplete", cannot_restore),
                PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                    cannot_unblock,
                    requested_unblock_from,
                } => format!(
                    "Task {} is not blocked by {}",
                    cannot_unblock, requested_unblock_from
                ),
                PrintableWarning::CannotPuntBecauseComplete { cannot_punt } =>
                    format!("Cannot punt complete task {}", cannot_punt),
                PrintableWarning::CannotSnoozeBecauseComplete {
                    cannot_snooze,
                } => format!("Cannot snooze complete task {}", cannot_snooze),
                PrintableWarning::SnoozedAfterDueDate {
                    snoozed_task,
                    due_date: _due_date,
                    snooze_date: _snooze_date,
                } => {
                    format!("Snoozed {} until after its due date", snoozed_task)
                }
                PrintableWarning::AmbiguousKey { key, matches } => {
                    format!(
                        "Ambiguous key {} matches multiple tasks: {}",
                        format_key(key),
                        format_numbers(matches.iter())
                    )
                }
                PrintableWarning::NoPathFoundBetween(a, b) => {
                    format!("No path found between {} and {}", a, b)
                }
            }
        )
    }
}
