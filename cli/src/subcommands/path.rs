use {clap::Parser, todo_lookup_key::Key};

/// Show the dependency paths between two tasks.
///
/// The given two tasks will be displayed at the start and end of the list,
/// with any tasks that are antidependencies of the first and dependencies
/// of the second printed in between, in order.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Path {
    /// Tasks to find paths between. Should match at least two tasks.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
}
