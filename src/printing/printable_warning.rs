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
        write!(f, "{}: ", Color::Yellow.bold().paint("warning"))?;
        use self::PrintableWarning::*;
        match self {
            NoMatchFoundForKey { requested_key } => {
                write!(f, "No match found for {}", format_key(requested_key))
            }
            CannotCheckBecauseAlreadyComplete { cannot_check } => {
                write!(f, "Task {} is already complete", cannot_check)
            }
            CannotRestoreBecauseAlreadyIncomplete { cannot_restore } => {
                write!(f, "Task {} is already incomplete", cannot_restore)
            }
            CannotUnblockBecauseTaskIsNotBlocked {
                cannot_unblock,
                requested_unblock_from,
            } => write!(
                f,
                "Task {} is not blocked by {}",
                cannot_unblock, requested_unblock_from
            ),
            CannotPuntBecauseComplete { cannot_punt } => {
                write!(f, "Cannot punt {} because it is complete", cannot_punt)
            }
            CannotSnoozeBecauseComplete { cannot_snooze } => {
                write!(
                    f,
                    "Cannot snooze {} because it is complete",
                    cannot_snooze
                )
            }
            SnoozedAfterDueDate {
                snoozed_task,
                due_date,
                snooze_date: _,
            } => write!(
                f,
                "Snoozed {} until after its due date {}",
                snoozed_task, due_date
            ),
            AmbiguousKey { key, matches } => {
                write!(
                    f,
                    "Ambiguous key {} matches {}",
                    format_key(key),
                    format_numbers(matches)
                )
            }
            NoPathFoundBetween(from, to) => {
                write!(f, "No path found between {} and {}", from, to)
            }
        }
    }
}