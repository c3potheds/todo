pub use super::Key;
use structopt::StructOpt;

/// Creates new tasks in the to-do list.
///
/// Each positional string will become the description of a new task in the
/// to-do list. By default, all new tasks are in the incomplete state, showing
/// up in the default list that you see when you invoke a raw 'todo' command.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct New {
    /// Descriptions for the new tasks, as raw strings.
    ///
    /// The description will be printed next to the task number when showing
    /// the task. You can use the description as a 'key' argument in commands
    /// that select existing tasks.
    #[structopt(verbatim_doc_comment, required = true, min_values = 1)]
    pub desc: Vec<String>,

    /// Block new tasks on these tasks.
    #[structopt(long, short = "p", value_name = "keys", min_values = 1)]
    pub blocked_by: Vec<Key>,

    /// Block these tasks on new tasks.
    #[structopt(long, short = "b", value_name = "keys", min_values = 1)]
    pub blocking: Vec<Key>,

    /// Put the new tasks before these tasks.
    ///
    /// This blocks the given "before" tasks on the new tasks, and block the new
    /// tasks on the deps of the "before" tasks.
    #[structopt(long, value_name = "keys", min_values = 1)]
    pub before: Vec<Key>,

    /// Put the new tasks after these tasks.
    ///
    /// This blocks the new tasks on the given "after" tasks, and block the
    /// adeps of the "after" tasks on the new tasks.
    #[structopt(long, value_name = "keys", min_values = 1)]
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
    /// A priority may be any decimal integer that's representable with 32 bits.
    ///
    /// When a task has a priority, it will show up before all other tasks with
    /// lower priorities. Tasks with no priority have an implicit priority of 0.
    /// Tasks may have negative priorities, in which case they show up after all
    /// unprioritized tasks.
    ///
    /// A task inherits an implicit priority from its antidependencies. The
    /// implicit priority of a task is the maximum implicit or explicit priority
    /// of all its antidependencies. This means tasks in --blocked-by may be
    /// reordered if you assign a priority! Dependencies whose implicit
    /// priority, and therefore ordering, are updated by assigning the new tasks
    /// this priority will be printed in the console output.
    #[structopt(long)]
    pub priority: Option<i32>,

    /// Assign a due date to the new tasks.
    ///
    /// The due date is expressed as a human-readable string, e.g. "2 days",
    /// "wednesday", "april 1", "10:30 pm", etc. The app will try to interpret
    /// the string in relation to the current system time in the local timezone.
    /// If the string cannot be interpreted, an error will be printed.
    ///
    /// When a task has a due date, it will show up before all other tasks with
    /// later due dates or no due dates (unless those tasks have higher
    /// priorirites). "No due date" is considered "later" than all explicit
    /// dates for the purposes of comparison.
    ///
    /// A task inherits an implicit due date from its antidependencies. The
    /// implicit due date of a task is the earliest implicit or explicit due
    /// date of all its antidependencies. This means tasks in --blocked-by may
    /// be reordered if you assign a due date! Dependencies whose implicit due
    /// date, and therefore ordering, are updated by assigning the new tasks
    /// this due date will be printed in the console output.
    #[structopt(long, min_values = 1)]
    pub due: Vec<String>,

    /// Allocate a time budget for the new tasks.
    ///
    /// The due dates of any dependencies of the new tasks will be bounded by
    /// the due date of the new tasks minus the budget.
    ///
    /// The budget is a human-readable description of a duration, like "2 days"
    /// or "12 hours". A time budget must be a duration, not an absolute time,
    /// like "12 pm" or "tomorrow", like the --due argument accepts.
    ///
    /// A budget has no effect unless the task also has an implicit or explicit
    /// due date.
    #[structopt(long, min_values = 1)]
    pub budget: Vec<String>,
}
