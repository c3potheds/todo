pub use super::Key;
use structopt::StructOpt;

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
    #[structopt(long)]
    pub due: Vec<String>,
}
