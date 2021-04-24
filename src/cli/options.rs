use structopt::StructOpt;

use super::SubCommand;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo",
    about = "Maintains and manipulates your to-do list.",
    author = "Simeon Anfinrud",
    version = "0.1"
)]
pub struct Options {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,

    /// Show blocked tasks in the status.
    #[structopt(long, short = "b")]
    pub include_blocked: bool,

    /// Show complete tasks in the status.
    #[structopt(long, short = "d")]
    pub include_done: bool,

    /// Show all tasks in the status.
    #[structopt(long, short = "a")]
    pub include_all: bool,
}
