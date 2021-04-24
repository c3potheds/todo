pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Put {
    /// Selected task keys.
    pub keys: Vec<Key>,
    /// Put the selected tasks before these tasks.
    #[structopt(long, short = "b")]
    pub before: Vec<Key>,
    /// Put the selected tasks after these tasks.
    #[structopt(long, short = "a")]
    pub after: Vec<Key>,
    /// Include affected complete tasks in the result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
