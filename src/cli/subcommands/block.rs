use super::Key;
use structopt::StructOpt;

/// Block tasks on other tasks.
///
/// If a task is blocked on any incomplete tasks, it won't show up in the
/// default list shown by the 'todo' command. It will reappear in the list
/// once all the tasks it is blocked on are marked as complete. This allows
/// you to create a "dependency" relationship between tasks, putting off
/// some tasks until you complete others first.
///
/// For example, I may wish to have two tasks, like "make dinner" and
/// "wash the dishes", where I can't do the second until I finish the first.
/// To put off the "wash the dishes" task until I complete the "make dinner"
/// task, I can do:
///
///   todo block "wash the dishes" --on "make dinner"
///
/// The "make dinner" task will show up in the 'todo' status, but the
/// "wash the dishes" task will not show up until the "make dinner" task is
/// complete.
///
/// You can undo this command with the 'unblock' command, run
/// 'todo help unblock' for more info.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Block {
    /// Tasks to block.
    #[structopt(required = true, min_values = 1)]
    pub keys: Vec<Key>,

    /// Tasks to block on.
    #[structopt(long, required = true, min_values = 1)]
    pub on: Vec<Key>,

    /// Include complete affected deps in result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
