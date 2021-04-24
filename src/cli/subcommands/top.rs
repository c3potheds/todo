pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Top {
    /// Tasks to find the top level underneath. If none are specified, shows the
    /// top-level tasks, i.e. tasks with no antidependencies. These may function
    /// as "categories" for high-level projects.
    pub keys: Vec<Key>,

    /// If passed, shows top-level complete tasks too.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
