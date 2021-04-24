pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Merge {
    /// Tasks to merge.
    #[structopt(required = true, min_values = 2)]
    pub keys: Vec<Key>,
    /// Description of new task to merge into.
    #[structopt(long)]
    pub into: String,
}
