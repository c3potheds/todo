use clap::Parser;

/// Search for tasks with descriptions that contain the given search terms.
///
/// The tasks, as always, will be ordered by their canonical numbering.
/// The results include incomplete tasks and blocked tasks, and can include
/// complete tasks if you pass the --include-done flag.
#[derive(Debug, Default, PartialEq, Parser)]
#[clap(verbatim_doc_comment)]
pub struct Find {
    /// Search terms, which can be a substring of any task description.
    #[clap(required = true, min_values = 1)]
    pub terms: Vec<String>,
    /// Show completed tasks in search results.
    #[clap(long, short = 'd')]
    pub include_done: bool,
    /// Search for tasks with the given tags.
    #[clap(long, short = 't')]
    pub tag: bool,
}
