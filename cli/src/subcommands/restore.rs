use {clap::Parser, todo_lookup_key::Key};

/// Restore completed tasks.
///
/// This resets the completion status of the given tasks and puts them back
/// in the list of incomplete tasks, as if they had never been completed.
///
/// A task cannot be restored if there are complete tasks that are blocked
/// on it. The complete blocked tasks must be restored first, just as
/// incomplete blocking tasks must be completed before the task they block
/// is completed.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Restore {
    /// Tasks to restore, marking as incomplete.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
    /// If passed, all complete antidependencies will also be restored.
    ///
    /// If this is not passed, it is an error to 'restore' (mark as incomplete)
    /// a task that blocks other complete tasks. When you "force"-restore a
    /// task, it will be guaranteed to be incomplete at the end of the
    /// operation.
    #[arg(long)]
    pub force: bool,
}
