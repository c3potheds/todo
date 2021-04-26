pub use super::Key;
use structopt::StructOpt;

/// Marks tasks as complete.
///
/// A task can only be "checked," or marked as complete, if it is incomplete
/// and if it is not blocked by any other incomplete tasks. Marking a task
/// as complete removes it from the list and puts it in the completed tasks
/// list.
///
/// When you check a task off the list, any tasks that were blocked on that
/// task will become unblocked if they have no other incomplete
/// dependencies.
///
/// You can undo this operation with the 'restore' command, run
/// 'todo help restore' for more info.
#[derive(Debug, PartialEq, StructOpt)]
pub struct Check {
    /// Tasks to mark as complete.
    #[structopt(verbatim_doc_comment)]
    pub keys: Vec<Key>,
    /// If passed, all incomplete dependencies will also be completed.
    ///
    /// If this is not passed, it is an error to 'check' (complete) a blocked
    /// task. When you "force"-check a task, it will be guaranteed to be
    /// complete at the end of the operation.
    #[structopt(long)]
    pub force: bool,
}
