use {clap::Parser, lookup_key::Key};

/// Punts tasks to the end of the list.
///
/// Tasks are, by default, sorted by the order of insertion. Punting a task
/// re-inserts it as if you removed and re-created it, without changing
/// anything else.
///
/// This can be useful if you have a long list of incomplete tasks and
/// habitually focus on the first ones in the list, but want to put off a
/// task for later without blocking it on anything. You can send it to the
/// end of the list with the 'punt' command.
#[derive(Debug, PartialEq, Parser)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Punt {
    /// Tasks to punt.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,
}
