use {clap::Parser, lookup_key::Key};

/// Assign a time budget to the given tasks.
///
/// If a task with a due date has a time budget, then its dependencies' implicit
/// due dates will be forced to be earlier than or at its due date minus its
/// time budget. For example, if my term paper is due in 1 week, I budget 3 days
/// for it after initial research, then the "initial research" task will have a
/// due date at latest 4 days from now.
///
/// This is most useful for planning out time management for long chains of
/// tasks that must be completed in order.
///
/// The --is argument is a human-readable description of a duration, like
/// "2 days" or "1 month" or "15 min". An error will be printed if the
/// description cannot be parsed. Unlike due dates, budgets cannot be absolute
/// dates or times or shorthands like "tomorrow", and must be a duration.
///
/// A description of "0" effectively removes the time budget, allowing
/// dependencies to have due dates up to the given task's due date.
#[derive(Debug, PartialEq, Eq, Parser)]
pub struct Budget {
    /// The tasks to assign a budget to.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,
    /// The description of the budgeted duration.
    #[clap(long, alias = "is", required = true, min_values = 1)]
    pub budget: Vec<String>,
    /// Show completed affected tasks.
    #[clap(long, short = 'd')]
    pub include_done: bool,
}
