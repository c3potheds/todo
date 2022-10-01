use {clap::Parser, lookup_key::Key};

/// Removes tasks from the list permanently.
///
/// This is not the same as the 'check' command, which marks tasks as
/// complete, but does not remove all trace that the task had ever existed.
///
/// When you remove a task that blocks some adeps and is blocked by some
/// deps, the adeps will be blocked directly on the deps to preserve
/// structure. For example, if you have the chain:
///
///   a <- b <- c
///
/// ... and you run:
///
///   todo rm b
///
/// ... then you will get the chain:
///
///   a <- c
///
/// Removal of tasks cannot be undone! You must manually re-create the task
/// if you want to undo it.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Rm {
    /// Tasks to remove.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
}
