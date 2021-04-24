pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Find {
    /// Search terms, which can be a substring of any task description.
    pub terms: Vec<String>,
    /// Show completed tasks in search results.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
