pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Unblock {
    /// Tasks to unblock.
    pub keys: Vec<Key>,

    /// Tasks to unblock from.
    #[structopt(long)]
    pub from: Vec<Key>,

    /// Show affected complete tasks in the result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
