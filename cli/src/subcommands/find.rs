use clap::Parser;

/// Search for tasks with descriptions that contain the given search terms.
///
/// The tasks, as always, will be ordered by their canonical numbering.
/// The results include incomplete tasks and blocked tasks, and can include
/// complete tasks if you pass the --include-done flag.
#[derive(Debug, Default, PartialEq, Eq, Parser)]
#[command(verbatim_doc_comment)]
pub struct Find {
    /// Search terms, which can be a substring of any task description.
    #[arg(required = true, num_args = 1..)]
    pub terms: Vec<String>,
    /// Show completed tasks in search results.
    #[arg(long, short = 'd')]
    pub include_done: bool,
}
