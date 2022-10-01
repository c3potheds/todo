use {clap::Parser, lookup_key::Key};

/// Merges two or more tasks into one.
///
/// The merged task will retain the dependency structure of the tasks it was
/// merged from. Its due date will be the earliest explicit due date of the
/// constituents and its priority will be the lowest explicit priority of
/// the constituents.
///
/// This is the opposite of 'split'.
#[derive(Debug, Default, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Merge {
    /// Tasks to merge.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
    /// Description of new task to merge into.
    #[arg(long)]
    pub into: String,
    /// If passed with a value of "true", the new task will be marked as a tag.
    /// If passed with a value of "false", the new task will not be marked as a
    /// tag. If not passed, the new task will be marked as a tag if all of the
    /// original tasks were marked as tags, otherwise it will not be marked as
    /// a tag.
    #[arg(long, short = 't')]
    pub tag: Option<bool>,
}
