pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Split {
    /// Tasks to split.
    pub keys: Vec<Key>,

    /// Descriptions for new tasks.
    #[structopt(long)]
    pub into: Vec<String>,

    /// If passed, the results of the split will be put in a dependency chain.
    #[structopt(long)]
    pub chain: bool,
}
