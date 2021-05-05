pub use super::subcommands::*;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommand {
    Block(Block),
    Budget(Budget),
    Chain(Chain),
    Check(Check),
    Due(Due),
    Edit(Edit),
    Find(Find),
    Get(Get),

    /// Shows completed tasks.
    ///
    /// Completed tasks are displayed in the reverse order that they were
    /// completed, i.e. most-recently completed tasks first. Completed tasks
    /// are associated with non-positive integers (with the most-recently
    /// completed task having number 0, and others having negative numbers) that
    /// can be used as task key arguments in commands.
    #[structopt(verbatim_doc_comment)]
    Log,

    Merge(Merge),
    New(New),
    Path(Path),
    Prefix(Prefix),
    Priority(Priority),
    Punt(Punt),
    Put(Put),
    Restore(Restore),
    Rm(Rm),
    Split(Split),
    Top(Top),
    Unblock(Unblock),
}
