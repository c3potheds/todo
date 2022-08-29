use {clap::Parser, lookup_key::Key};

/// Makes tasks temporarily "snoozed" until the given amount of time passes.
///
/// Snoozed tasks are considered "blocked" and will not show up in the
/// incomplete task list, but can be "checked" off without unsnoozing them if
/// you know their position in the list (which you can determine using e.g. the
/// 'todo -b', 'todo get', or 'todo find' commands). The first time you run
/// 'todo' (with no subcommand) after the given amount of time elapses, the
/// snoozed tasks will be "unsnoozed" and appear back in the incomplete task
/// list.
///
/// Snoozed tasks only become unsnoozed through the 'unsnooze' command or when
/// the 'todo' command, with no subcommand, is run after the given amount of
/// time has passed. This is to prevent the positions of tasks from shuffling
/// around invisibly between commands.
///
/// The 'until' argument is a human-readable description of a duration, date, or
/// time, e.g. "2 days", "9pm", or "saturday". If the 'until' argument evaluates
/// to a day-level precision, the unsnooze time will snap to the beginning of
/// that day. If the 'until' argument is the name of a month, the task will
/// unsnooze at the beginning of that month.
#[derive(Debug, PartialEq, Eq, Parser)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Snooze {
    /// Tasks to snooze.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,

    /// Description of how long to snooze.
    #[clap(long, min_values = 1)]
    pub until: Vec<String>,
}
