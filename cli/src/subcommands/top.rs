use clap::Parser;
use todo_lookup_key::Key;

/// Shows top-level tasks, i.e. tasks with no antidependencies.
///
/// One can represent "categories" for tasks by blocking a task representing
/// a category on the tasks that should be in that category. When running
/// this command, you can see all "uncategorized" tasks.
#[derive(Debug, PartialEq, Eq, Parser, Default)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Top {
    /// Tasks to find the top level underneath. If none are specified, shows the
    /// top-level tasks, i.e. tasks with no antidependencies. These may function
    /// as "categories" for high-level projects.
    pub keys: Vec<Key>,

    /// If passed, shows top-level complete tasks too.
    #[arg(long, short = 'd')]
    pub include_done: bool,
}
