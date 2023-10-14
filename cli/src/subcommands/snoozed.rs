use clap::Parser;

/// Shows snoozed tasks.
#[derive(Debug, Default, PartialEq, Eq, Parser)]
#[command(verbatim_doc_comment)]
pub struct Snoozed {
    /// Only show tasks that will unsnooze before the given time.
    ///
    /// This is a human-readable description of a date or time, like "5pm" or
    /// "tomorrow".
    #[arg(long, num_args = 1..)]
    pub until: Option<Vec<String>>,
}
