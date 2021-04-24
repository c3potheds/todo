pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Restore {
    /// Tasks to restore, marking as incomplete.
    pub keys: Vec<Key>,
    /// If passed, all complete antidependencies will also be restored.
    ///
    /// If this is not passed, it is an error to 'restore' (mark as incomplete)
    /// a task that blocks other complete tasks. When you "force"-restore a
    /// task, it will be guaranteed to be incomplete at the end of the
    /// operation.
    #[structopt(long)]
    pub force: bool,
}
