use super::Key;
use structopt::StructOpt;

/// Shows top-level tasks, i.e. tasks with no antidependencies.
///
/// One can represent "categories" for tasks by blocking a task representing
/// a category on the tasks that should be in that category. When running
/// this command, you can see all "uncategorized" tasks.
#[derive(Debug, PartialEq, StructOpt, Default)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Top {
    /// Tasks to find the top level underneath. If none are specified, shows the
    /// top-level tasks, i.e. tasks with no antidependencies. These may function
    /// as "categories" for high-level projects.
    pub keys: Vec<Key>,

    /// If passed, shows top-level complete tasks too.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
