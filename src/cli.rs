use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Clone)]
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
pub struct Block {
    /// Tasks to block.
    pub keys: Vec<Key>,

    /// Tasks to block on.
    #[structopt(long)]
    pub on: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct Check {
    /// Tasks to mark as complete.
    #[structopt(verbatim_doc_comment)]
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Edit {
    /// Tasks to edit.
    pub keys: Vec<Key>,
    /// The new description.
    #[structopt(long)]
    pub desc: String,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Get {
    /// Tasks to explore.
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct New {
    /// Descriptions for the new tasks, as raw strings.
    ///
    /// The description will be printed next to the task number when showing
    /// the task. You can use the description as a 'key' argument in commands
    /// that select existing tasks.
    #[structopt(verbatim_doc_comment)]
    pub desc: Vec<String>,

    /// Block new tasks on these tasks.
    #[structopt(long, short = "p", value_name = "keys")]
    pub blocked_by: Vec<Key>,

    /// Block these tasks on new tasks.
    #[structopt(long, short = "b", value_name = "keys")]
    pub blocking: Vec<Key>,

    /// Put the new tasks in a blocking sequence.
    ///
    /// For example, if you do:
    ///
    ///   todo new a b c --chain
    ///
    /// ... then 'a' will block 'b', and 'b' will block 'c'.
    ///
    /// This allows you to write out step-by-step plans for a project in one
    /// command, but keep only one step in focus in the default 'todo' list at
    /// a time.
    #[structopt(long, verbatim_doc_comment)]
    pub chain: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Punt {
    /// Tasks to punt.
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Restore {
    /// Tasks to restore, marking as incomplete.
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Unblock {
    /// Tasks to unblock.
    pub keys: Vec<Key>,

    /// Tasks to unblock from.
    #[structopt(long)]
    pub from: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommand {
    /// Block tasks on other tasks.
    ///
    /// If a task is blocked on any incomplete tasks, it won't show up in the
    /// default list shown by the 'todo' command. It will reappear in the list
    /// once all the tasks it is blocked on are marked as complete. This allows
    /// you to create a "dependency" relationship between tasks, putting off
    /// some tasks until you complete others first.
    ///
    /// For example, I may wish to have two tasks, like "make dinner" and
    /// "wash the dishes", where I can't do the second until I finish the first.
    /// To put off the "wash the dishes" task until I complete the "make dinner"
    /// task, I can do:
    ///
    ///   todo block "wash the dishes" --on "make dinner"
    ///
    /// The "make dinner" task will show up in the 'todo' status, but the
    /// "wash the dishes" task will not show up until the "make dinner" task is
    /// complete.
    ///
    /// You can undo this command with the 'unblock' command, run
    /// 'todo help unblock' for more info.
    #[structopt(verbatim_doc_comment)]
    Block(Block),

    /// Marks tasks as complete.
    ///
    /// A task can only be "checked," or marked as complete, if it is incomplete
    /// and if it is not blocked by any other incomplete tasks. Marking a task
    /// as complete removes it from the list and puts it in the completed tasks
    /// list.
    ///
    /// When you check a task off the list, any tasks that were blocked on that
    /// task will become unblocked if they have no other incomplete
    /// dependencies.
    ///
    /// You can undo this operation with the 'restore' command, run
    /// 'todo help restore' for more info.
    #[structopt(verbatim_doc_comment)]
    Check(Check),

    /// Edit the description of tasks.
    ///
    /// This allows you to fix typos in task descriptions, or add new
    /// information if needed.
    #[structopt(verbatim_doc_comment)]
    Edit(Edit),

    /// Shows tasks related to given tasks.
    ///
    /// When you have complex dependency relationships between many tasks, it
    /// can be helpful to trace out the paths between them. The 'get' command
    /// will show the given tasks, as well as any tasks they're transitively
    /// blocked on or blocking.
    ///
    /// One way this is useful is to have a succinct, easy-to-remember umbrella
    /// task representing a category, which you block on all sub-tasks related
    /// to that category. You can then show all the tasks in the category by
    /// running the 'get' command, selecting the category task. For example:
    ///
    ///   # Create tasks representing a 'work' category and a 'home' category.
    ///   todo new work home
    ///
    ///   # Create tasks in the 'work' category.
    ///   todo new "9 am meeting" "review project proposal" -b work
    ///
    ///   # Create tasks in the 'home' category.
    ///   todo new "walk the dog" "file taxes" -b home
    ///
    ///   # View all tasks related to the 'home' category.
    ///   todo get home
    ///
    /// This is also useful for seeing what tasks may be unlocked if you
    /// complete a certain task. You can use this to get a "big picture" view
    /// of how a task fits into the larger plan.
    #[structopt(verbatim_doc_comment)]
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

    /// Creates new tasks in the to-do list.
    ///
    /// Each positional string will become the description of a new task in the
    /// to-do list. By default, all new tasks are in incomplete state, showing
    /// in the default list that you see when you invoke a raw 'todo' command.
    ///
    /// You can use the --blocked-by argument to block the new tasks on tasks
    /// matching the given keys. This is a shorthand for using the 'block'
    /// command after the 'new' command. Likewise, you can use --blocking to
    /// block existing tasks on the new tasks.
    ///
    /// You can also use the --chain argument to string the new tasks together,
    /// blocking each new task on the previous one. This lets you create a
    /// sequence of multiple tasks, where one must be completed before the next,
    /// in a single command.
    #[structopt(verbatim_doc_comment)]
    New(New),

    /// Punts tasks to the end of the list.
    ///
    /// Tasks are, by default, sorted by the order of insertion. Punting a task
    /// re-inserts it as if you removed and re-created it, without changing
    /// anything else.
    ///
    /// This can be useful if you have a long list of incomplete tasks and
    /// habitually focus on the first ones in the list, but want to put off a
    /// task for later without blocking it on anything. You can send it to the
    /// end of the list with the 'punt' command.
    #[structopt(verbatim_doc_comment)]
    Punt(Punt),

    /// Restore completed tasks.
    ///
    /// This resets the completion status of the given tasks and puts them back
    /// in the list of incomplete tasks, as if they had never been completed.
    ///
    /// A task cannot be restored if there are complete tasks that are blocked
    /// on it. The complete blocked tasks must be restored first, just as
    /// incomplete blocking tasks must be completed before the task they block
    /// is completed.
    #[structopt(verbatim_doc_comment)]
    Restore(Restore),

    /// Unblock tasks from other tasks.
    ///
    /// This is the "undo" operation for the 'block' command.
    #[structopt(verbatim_doc_comment)]
    Unblock(Unblock),
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
