use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq)]
pub enum Key {
    ByNumber(i32),
    ByName(String),
}

impl FromStr for Key {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(n) => Ok(Key::ByNumber(n)),
            Err(_) => Ok(Key::ByName(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct New {
    pub desc: Vec<String>,
    #[structopt(
        long,
        short = "p",
        value_name = "keys",
        help = "Block new tasks on these tasks."
    )]
    pub blocked_by: Vec<Key>,
    #[structopt(
        long,
        short = "b",
        value_name = "keys",
        help = "Block these tasks on new tasks."
    )]
    pub blocking: Vec<Key>,
    #[structopt(long, help = "Put the new tasks in a blocking sequence.")]
    pub chain: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct Check {
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Restore {
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Block {
    pub keys: Vec<Key>,
    #[structopt(long)]
    pub on: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Unblock {
    pub keys: Vec<Key>,
    #[structopt(long)]
    pub from: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Get {
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Punt {
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Edit {
    pub keys: Vec<Key>,
    #[structopt(long, help = "The new description.")]
    pub desc: String,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommand {
    /// Marks tasks as complete.
    Check(Check),
    /// Creates new tasks in the to-do list.
    New(New),
    /// Shows completed tasks.
    Log,
    /// Restore completed tasks.
    Restore(Restore),
    /// Block tasks on other tasks.
    Block(Block),
    /// Unblock tasks from other tasks.
    Unblock(Unblock),
    /// Shows tasks related to given tasks.
    Get(Get),
    /// Punts tasks to the end of the list.
    Punt(Punt),
    /// Edit the description of tasks.
    Edit(Edit),
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
    #[structopt(long, short = "b", help = "Show blocked tasks in the status.")]
    pub include_blocked: bool,
    #[structopt(
        long,
        short = "d",
        help = "Show complete tasks in the status."
    )]
    pub include_done: bool,
    #[structopt(long, short = "a", help = "Show all tasks in the status.")]
    pub include_all: bool,
}
