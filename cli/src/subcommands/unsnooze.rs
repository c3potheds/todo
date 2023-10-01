use {clap::Parser, todo_lookup_key::Key};

/// Unsnoozes snoozed tasks.
///
/// Unsnoozed tasks with no incomplete deps will become visible, moved to the
/// end of the list of unblocked incomplete tasks.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Unsnooze {
    /// Tasks to unsnooze.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
}
