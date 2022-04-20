use structopt::StructOpt;

/// Search for tasks with descriptions that contain the given search terms.
///
/// The tasks, as always, will be ordered by their canonical numbering.
/// The results include incomplete tasks and blocked tasks, and can include
/// complete tasks if you pass the --include-done flag.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(verbatim_doc_comment)]
pub struct Find {
    /// Search terms, which can be a substring of any task description.
    #[structopt(required = true, min_values = 1)]
    pub terms: Vec<String>,
    /// Show completed tasks in search results.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
