use super::format_util::format_keys;
use super::format_util::format_numbers;
use super::BriefPrintableTask;
use ansi_term::Color;
use cli::Key;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
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
    },
    FailedToUseTextEditor,
    NoMatchForKeys {
        keys: Vec<Key>,
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
            Color::Red.bold().paint("error"),
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
                } => format!("Could not parse line: \"{}\"", malformed_line),
                PrintableError::FailedToUseTextEditor => {
                    "Failed to open text editor".to_string()
                }
                PrintableError::NoMatchForKeys{ keys } => {
                    format!(
                        "No match for keys {}",
                        format_keys(keys),
                    )
                }
                PrintableError::CannotParseDueDate { cannot_parse } => {
                    format!(
                        "Cannot parse due date: {}",
                        Color::White.bold().paint(cannot_parse),
                    )
                }
                PrintableError::CannotParseDuration { cannot_parse } => {
                    format!(
                        "Cannot parse duration: {}",
                        Color::White.bold().paint(cannot_parse),
                    )
                }
                PrintableError::DurationIsTooLong { duration, string_repr } => {
                    format!(
                        "Time budget is too long: {} (from {}).\n{}: {}",
                        Color::White.bold().paint(format!("{} secs", duration)),
                        Color::White.bold().paint(string_repr),
                        Color::White.bold().dimmed().paint("note"),
                        "Must be less than ~136 years, or 2^32 seconds."
                    )
                }
                PrintableError::ConflictingArgs((a, b)) => {
                    format!(
                        "Cannot pass {} and {} at the same time",
                        Color::White.bold().paint(a),
                        Color::White.bold().paint(b),
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
