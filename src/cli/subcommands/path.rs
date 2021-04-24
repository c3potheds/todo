pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Path {
    /// Tasks to find paths between. Should match at least two tasks.
    #[structopt(required = true, min_values = 1)]
    pub keys: Vec<Key>,
}
