pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Chain {
    /// Tasks to arrange in a blocking sequence.
    pub keys: Vec<Key>,
    /// Show complete affected tasks.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
