use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use todo_lookup_key::Key;
use yansi::Paint;

use crate::format_util::format_keys;
use crate::format_util::format_numbers;
use crate::BriefPrintableTask;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrintableError {
    CannotCheckBecauseBlocked {
        cannot_check: BriefPrintableTask,
        blocked_by: Vec<BriefPrintableTask>,
    },
    CannotRestoreBecauseAntidependencyIsComplete {
        cannot_restore: BriefPrintableTask,
        complete_antidependencies: Vec<BriefPrintableTask>,
    },
    CannotBlockBecauseWouldCauseCycle {
        cannot_block: BriefPrintableTask,
        requested_dependency: BriefPrintableTask,
    },
    CannotEditBecauseUnexpectedNumber {
        requested: i32,
    },
    CannotEditBecauseNoTaskWithNumber {
        requested: i32,
    },
    CannotEditBecauseInvalidLine {
        malformed_line: String,
        explanation: String,
    },
    FailedToUseTextEditor,
    NoMatchForKeys {
        keys: Vec<Key>,
    },
    EmptyDate {
        flag: Option<String>,
    },
    CannotParseDueDate {
        cannot_parse: String,
    },
    CannotParseDuration {
        cannot_parse: String,
    },
    DurationIsTooLong {
        duration: u64,
        string_repr: String,
    },
    ConflictingArgs((String, String)),
    CannotMerge {
        cycle_through: Vec<BriefPrintableTask>,
        adeps_of: Vec<BriefPrintableTask>,
        deps_of: Vec<BriefPrintableTask>,
    },
}

impl Display for PrintableError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            "error".red().bold(),
            match self {
                PrintableError::CannotCheckBecauseBlocked {
                    cannot_check,
                    blocked_by,
                } => format!(
                    "Cannot complete {} because it is blocked by {}",
                    cannot_check,
                    format_numbers(blocked_by.iter()),
                ),
                PrintableError::CannotRestoreBecauseAntidependencyIsComplete{
                    cannot_restore,
                    complete_antidependencies,
                } => format!(
                    "Cannot restore {} because it blocks complete tasks {}",
                    cannot_restore,
                    format_numbers(complete_antidependencies.iter())
                ),
                PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block,
                    requested_dependency,
                } => format!(
                    "Cannot block {} on {} because it would create a cycle",
                    cannot_block,
                    requested_dependency,
                ),
                PrintableError::CannotEditBecauseUnexpectedNumber {
                    requested,
                } => format!(
                    "Number {}) doesn't correspond to any of requested tasks",
                    requested,
                ),
                PrintableError::CannotEditBecauseNoTaskWithNumber {
                    requested,
                } => format!("No task with number {})", requested),
                PrintableError::CannotEditBecauseInvalidLine{
                    malformed_line,
                    explanation,
                } => format!(
                    "Could not parse line: \"{}\"; {}",
                    malformed_line,
                    explanation,
                ),
                PrintableError::FailedToUseTextEditor => {
                    "Failed to open text editor".to_string()
                }
                PrintableError::NoMatchForKeys{ keys } => {
                    format!(
                        "No match for keys {}",
                        format_keys(keys),
                    )
                }
                PrintableError::EmptyDate{ flag } => {
                    match flag {
                        Some(flag) => format!(
                            "Empty date for flag {}",
                            flag.white().bold(),
                        ),
                        None => "Empty date".to_string(),
                    }
                }
                PrintableError::CannotParseDueDate { cannot_parse } => {
                    format!(
                        "Cannot parse due date: {}",
                        cannot_parse.white().bold(),
                    )
                }
                PrintableError::CannotParseDuration { cannot_parse } => {
                    format!(
                        "Cannot parse duration: {}",
                        cannot_parse.white().bold(),
                    )
                }
                PrintableError::DurationIsTooLong { duration, string_repr } => {
                    format!(
                        "Time budget is too long: {} (from {}).\n{}: {}",
                        format!("{duration} secs").white().bold(),
                        string_repr.white().bold(),
                        "note".white().bold().dim(),
                        "Must be less than ~136 years, or 2^32 seconds."
                    )
                }
                PrintableError::ConflictingArgs((a, b)) => {
                    format!(
                        "Cannot pass {} and {} at the same time",
                        a.white().bold(),
                        b.white().bold(),
                    )
                }
                PrintableError::CannotMerge {
                    cycle_through,
                    adeps_of,
                    deps_of
                } => {
                    format!(
                        "Cannot merge: tasks {} are adeps of {} but deps of {}",
                        format_numbers(cycle_through.iter()),
                        format_numbers(adeps_of.iter()),
                        format_numbers(deps_of.iter())
                    )
                }
            }
        )
    }
}
