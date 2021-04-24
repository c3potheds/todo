pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Priority {
    /// Tasks to assign a priority to.
    pub keys: Vec<Key>,
    /// The priority level for the tasks.
    #[structopt(long = "is", short = "P")]
    pub priority: Option<i32>,
    /// Show complete tasks in the result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
