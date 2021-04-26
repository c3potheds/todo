pub use super::Key;
use structopt::StructOpt;

/// Assign or query priority levels.
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
///
/// If you provide neither tasks nor a priority, all tasks with a priority
/// greater than zero will be printed. If you only supply a priority, all
/// tasks with a priority greater than or equal to that level will be
/// printed. If you only supply task keys, the matching tasks will be
/// printed along with their antidependencies that share their priority.
/// And if both task keys and a priority are supplied, matching tasks will
/// have their priority set to the given level, and all affected tasks will
/// be printed.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Priority {
    /// Tasks to assign a priority to.
    pub keys: Vec<Key>,
    /// The priority level for the tasks.
    #[structopt(long = "is", short = "P")]
    pub priority: Option<i32>,
    /// Show complete tasks in the result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
