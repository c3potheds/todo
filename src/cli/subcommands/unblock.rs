pub use super::Key;
use structopt::StructOpt;

/// Unblock tasks from other tasks.
///
/// This is the "undo" operation for the 'block' command. If no --from argument
/// is provided, then the given tasks will be unblocked from all of their direct
/// dependencies.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Unblock {
    /// Tasks to unblock.
    pub keys: Vec<Key>,

    /// Tasks to unblock from.
    #[structopt(long)]
    pub from: Vec<Key>,

    /// Show affected complete tasks in the result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
