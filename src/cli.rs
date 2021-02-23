use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq)]
pub enum Key {
    ByNumber(i32),
}

impl FromStr for Key {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Key::ByNumber(s.parse::<i32>()?))
    }
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct New {
    pub desc: Vec<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct Check {
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct Restore {
    #[structopt(set = structopt::clap::ArgSettings::AllowLeadingHyphen)]
    pub keys: Vec<Key>,
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
