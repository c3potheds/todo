use {clap::Parser, lookup_key::Key};

/// Unsnoozes snoozed tasks.
///
/// Unsnoozed tasks with no incomplete deps will become visible, moved to the
/// end of the list of unblocked incomplete tasks.
#[derive(Debug, PartialEq, Eq, Parser)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Unsnooze {
    /// Tasks to unsnooze.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,
}
