use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Clone)]
pub enum Key {
    ByNumber(i32),
    ByName(String),
    ByRange(i32, i32),
}

fn split_once<'a>(s: &'a str, pattern: &'a str) -> Option<(&'a str, &'a str)> {
    let mut iter = s.splitn(2, pattern);
    match iter.next() {
        Some(first) => match iter.next() {
            Some(rest) => Some((first, rest)),
            _ => None,
        },
        _ => None,
    }
}

impl FromStr for Key {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i32>() {
            return Ok(Key::ByNumber(n));
        }
        if let Some((prefix, suffix)) = split_once(s, "..") {
            if let (Ok(start), Ok(end)) =
                (prefix.parse::<i32>(), suffix.parse::<i32>())
            {
                return Ok(Key::ByRange(start, end));
            }
        }
        Ok(Key::ByName(s.to_string()))
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
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Chain {
    /// Tasks to arrange in a blocking sequence.
    pub keys: Vec<Key>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct Check {
    /// Tasks to mark as complete.
    #[structopt(verbatim_doc_comment)]
    pub keys: Vec<Key>,
    /// If passed, all incomplete dependencies will also be completed.
    ///
    /// If this is not passed, it is an error to 'check' (complete) a blocked
    /// task. When you "force"-check a task, it will be guaranteed to be
    /// complete at the end of the operation.
    #[structopt(long)]
    pub force: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Edit {
    /// Tasks to edit.
    pub keys: Vec<Key>,
    /// The new description. If not set, a text editor is used.
    #[structopt(long)]
    pub desc: Option<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct Find {
    /// Search terms, which can be a substring of any task description.
    pub terms: Vec<String>,
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

    /// Put the new tasks before these tasks.
    ///
    /// This blocks the given "before" tasks on the new tasks, and block the new
    /// tasks on the deps of the "before" tasks.
    #[structopt(long, value_name = "keys")]
    pub before: Vec<Key>,

    /// Put the new tasks after these tasks.
    ///
    /// This blocks the new tasks on the given "after" tasks, and block the
    /// adeps of the "after" tasks on the new tasks.
    #[structopt(long, value_name = "keys")]
    pub after: Vec<Key>,

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

    /// Assign a priority to the new tasks.
    ///
    /// When a task has a priority, it will show up before all other tasks with
    /// lower priorities. Tasks with no priority have an implicit priority of 0.
    /// Tasks may have negative priorities, in which case they show up after all
    /// unprioritized tasks.
    ///
    /// A task inherits an implicit priority from its antidependencies. The
    /// implicit priority of a task is the maximum implicit or explicit priority
    /// of all its antidependencies. This means tasks in --blocked-by may be
    /// reordered if you assign a priority!
    #[structopt(long)]
    pub priority: Option<i32>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Path {
    /// The least dependent task.
    pub from: Key,
    /// The most dependent task.
    pub to: Key,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Priority {
    /// Tasks to assign a priority to.
    pub keys: Vec<Key>,
    /// The priority level for the tasks.
    #[structopt(long = "is", short = "P")]
    pub priority: i32,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Punt {
    /// Tasks to punt.
    pub keys: Vec<Key>,
}

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
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Restore {
    /// Tasks to restore, marking as incomplete.
    pub keys: Vec<Key>,
    /// If passed, all complete antidependencies will also be restored.
    ///
    /// If this is not passed, it is an error to 'restore' (mark as incomplete)
    /// a task that blocks other complete tasks. When you "force"-restore a
    /// task, it will be guaranteed to be incomplete at the end of the
    /// operation.
    #[structopt(long)]
    pub force: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Rm {
    /// Tasks to remove.
    pub keys: Vec<Key>,
}

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

    /// Chain tasks in a blocking sequence.
    ///
    /// Each task will block its successor in the order given. For example, if
    /// you run:
    ///
    ///   todo chain a b c
    ///
    /// ... then 'a' will block 'b' and 'b' will block 'c'.
    #[structopt(verbatim_doc_comment)]
    Chain(Chain),

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

    /// Search for tasks with descriptions that contain the given search terms.
    ///
    /// The tasks, as always, will be ordered by their canonical numbering.
    /// The results include complete tasks, incomplete tasks, and blocked tasks.
    #[structopt(verbatim_doc_comment)]
    Find(Find),

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

    /// Show the dependency paths between two tasks.
    ///
    /// The given two tasks will be displayed at the start and end of the list,
    /// with any tasks that are antidependencies of the first and dependencies
    /// of the second printed in between, in order.
    #[structopt(verbatim_doc_comment)]
    Path(Path),

    /// Assign priority levels to given tasks.
    ///
    /// The ordering of tasks is determined first by the dependency structure.
    /// The "depth" of a task is how many dependency edges one needs to travel
    /// to reach a task with no incomplete dependencies. (By this definition,
    /// unblocked tasks have a depth of 0.) Tasks with the same "depth" are then
    /// sorted by priority, with higher-priority tasks appearing before lower-
    /// priority tasks. Tasks by default have a priority of 0. Tasks may have
    /// negative priorities, in which case they'll appear after the default-
    /// priority tasks.
    ///
    /// All dependent tasks will inherit an implicit priority, which overrides
    /// the explicit priority if it's greater. For example, if task "a" blocks
    /// task "b", and "b" has a higher priority, then "a" will be treated as if
    /// it has "b"'s priority. If the dependency relationship is broken, e.g.
    /// through the 'unblock' or 'rm' command, then "a"'s canonical priority
    /// will again be its own explicit priority (unless, of course, "a" has any
    /// other transitive antidependencies with a higher priority).
    #[structopt(verbatim_doc_comment)]
    Priority(Priority),

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

    /// Puts tasks before or after other tasks.
    ///
    /// Putting a task before another task is like blocking the latter on the
    /// former, but also blocks the former on all the dependencies of the
    /// latter. Likewise, putting a task after another task blocks the former on
    /// the latter and blocks everything that was blocked on the latter on the
    /// former.
    ///
    /// For example, in a dependency chain:
    ///
    ///   a <- b <- c
    ///
    /// If you put task "t" before b, the result is:
    ///
    ///   a <- t <- b <- c
    ///
    /// If you put task "t" after b, the result is:
    ///
    ///   a <- b <- t <- c
    #[structopt(verbatim_doc_comment)]
    Put(Put),

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

    /// Removes tasks from the list permanently.
    ///
    /// This is not the same as the 'check' command, which marks tasks as
    /// complete, but does not remove all trace that the task had ever existed.
    ///
    /// When you remove a task that blocks some adeps and is blocked by some
    /// deps, the adeps will be blocked directly on the deps to preserve
    /// structure. For example, if you have the chain:
    ///
    ///   a <- b <- c
    ///
    /// ... and you run:
    ///
    ///   todo rm b
    ///
    /// ... then you will get the chain:
    ///
    ///   a <- c
    ///
    /// Removal of tasks cannot be undone! You must manually re-create the task
    /// if you want to undo it.
    #[structopt(verbatim_doc_comment)]
    Rm(Rm),

    /// Shows top-level tasks, i.e. tasks with no antidependencies.
    ///
    /// One can represent "categories" for tasks by blocking a task representing
    /// a category on the tasks that should be in that category. When running
    /// this command, you can see all "uncategorized" tasks.
    #[structopt(verbatim_doc_comment)]
    Top(Top),

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
