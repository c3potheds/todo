pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Block {
    /// Tasks to block.
    pub keys: Vec<Key>,

    /// Tasks to block on.
    #[structopt(long)]
    pub on: Vec<Key>,

    /// Include complete affected deps in result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
