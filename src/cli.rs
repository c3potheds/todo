use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub struct New {
    pub desc: Vec<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct Check {
    pub keys: Vec<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommand {
    /// Marks tasks as complete.
    Check(Check),
    /// Creates new tasks in the to-do list.
    New(New),
}

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
}
