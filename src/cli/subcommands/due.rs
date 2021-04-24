pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Due {
    /// Tasks to query or assign the due date.
    pub keys: Vec<Key>,
    /// The due date to assign or query.
    ///
    /// This is a human-readable description of a date or time, like "1 day" or
    /// "5pm".
    #[structopt(long, alias = "in", alias = "on")]
    pub due: Vec<String>,
    /// Remove the explicit due date. If the implicit due date is inherited from
    /// an antidependency, it is retained.
    #[structopt(long)]
    pub none: bool,
    /// Show completed tasks in queries.
    ///
    /// This is not used when assigning or unassigning due dates. It only
    /// affects the printed results when querying the source of a task's due
    /// date, querying all tasks with due dates, or querying all tasks due
    /// earlier than a given date.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
