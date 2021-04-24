pub use super::Key;
use structopt::StructOpt;

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
