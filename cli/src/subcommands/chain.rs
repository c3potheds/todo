use {clap::Parser, todo_lookup_key::Key};

/// Chain tasks in a blocking sequence.
///
/// Each task will block its successor in the order given. For example, if
/// you run:
///
///   todo chain a b c
///
/// ... then 'a' will block 'b' and 'b' will block 'c'.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Chain {
    /// Tasks to arrange in a blocking sequence.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
    /// Show complete affected tasks.
    #[arg(long, short = 'd')]
    pub include_done: bool,
}
