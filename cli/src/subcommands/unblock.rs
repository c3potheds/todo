use {clap::Parser, lookup_key::Key};

/// Unblock tasks from other tasks.
///
/// This is the "undo" operation for the 'block' command. If no --from argument
/// is provided, then the given tasks will be unblocked from all of their direct
/// dependencies.
#[derive(Debug, PartialEq, Eq, Parser)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Unblock {
    /// Tasks to unblock.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,

    /// Tasks to unblock from.
    #[clap(long, min_values = 1)]
    pub from: Vec<Key>,

    /// Show affected complete tasks in the result.
    #[clap(long, short = 'd')]
    pub include_done: bool,
}
