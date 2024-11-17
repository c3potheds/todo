use clap::Parser;

pub use crate::subcommands::*;

#[derive(Debug, PartialEq, Eq, Parser)]
pub enum SubCommand {
    Block(Block),
    Bottom(Bottom),
    Budget(Budget),
    Chain(Chain),
    Check(Check),
    Clean(Clean),
    Config(Config),
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
    #[command(verbatim_doc_comment)]
    Log,

    Merge(Merge),
    New(New),
    Path(Path),
    Priority(Priority),
    Punt(Punt),
    Put(Put),
    Restore(Restore),
    Rm(Rm),
    Snooze(Snooze),
    Snoozed(Snoozed),
    Split(Split),
    Tag(Tag),
    Top(Top),
    Unblock(Unblock),
    Unsnooze(Unsnooze),
}
