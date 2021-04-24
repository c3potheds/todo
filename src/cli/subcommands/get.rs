pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Get {
    /// Tasks to explore.
    pub keys: Vec<Key>,
    /// Show completed deps if no given task is complete.
    ///
    /// Completed deps and adeps will be shown even without this flag if any of
    /// the tasks that match the given keys are themselves complete.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
