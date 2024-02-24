use yansi::Paint;
use {
    crate::{
        format_util::{format_key, format_numbers},
        BriefPrintableTask,
    },
    chrono::{DateTime, Utc},
    std::{
        fmt,
        fmt::{Display, Formatter},
    },
    todo_lookup_key::Key,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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
    SnoozedUntilPast {
        snoozed_task: BriefPrintableTask,
        snooze_date: DateTime<Utc>,
    },
    CannotUnsnoozeBecauseComplete(BriefPrintableTask),
    CannotUnsnoozeBecauseBlocked {
        cannot_unsnooze: BriefPrintableTask,
        blocked_by: Vec<BriefPrintableTask>,
    },
    CannotUnsnoozeBecauseNotSnoozed(BriefPrintableTask),
    AmbiguousKey {
        key: Key,
        matches: Vec<BriefPrintableTask>,
    },
    NoPathFoundBetween(BriefPrintableTask, BriefPrintableTask),
}

impl Display for PrintableWarning {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: ", "warning".yellow().bold())?;
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
            SnoozedUntilPast {
                snoozed_task,
                snooze_date,
            } => write!(
                f,
                "Snoozed {} until {} which is in the past",
                snoozed_task, snooze_date
            ),
            CannotUnsnoozeBecauseComplete(task) => {
                write!(f, "Cannot unsnooze {} because it is complete", task)
            }
            CannotUnsnoozeBecauseBlocked {
                cannot_unsnooze,
                blocked_by,
            } => write!(
                f,
                "Cannot unsnooze {} because it is blocked by {}",
                cannot_unsnooze,
                format_numbers(blocked_by)
            ),
            CannotUnsnoozeBecauseNotSnoozed(task) => {
                write!(f, "Cannot unsnooze {} because it is not snoozed", task)
            }
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
