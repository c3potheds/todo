use clap::Parser;
use todo_lookup_key::Key;

/// Assigns or queries due dates.
///
/// Tasks inherit an implicit due date from their antidependencies. If an
/// antidependency has an earlier due date than the task's explicit due
/// date, then the task will be treated as if it has that antidependency's
/// due date.
///
/// This command takes two optional lists: task keys (positional) and a due
/// date description (after the --due, --in, or --on flag). The latter is a
/// human-readable description of a date, like "2 days", "1 month", or
/// "april 1". If the description cannot be interpreted, an error will be
/// printed.
///
/// If neither task keys nor a due date is provided, this will print all
/// tasks that are due today, implicitly or explicitly. I.e.
///
///   todo due
///
/// If only task keys are provided, then those tasks will be printed, in
/// addition to the antidependencies that cause them to have that due date.
/// E.g.
///
///   todo due a b c
///
/// If only a due date description is provided, then all tasks with a due
/// date occurring before the given due date are printed. E.g.
///
///   todo due --in 3 days
///
/// If both task keys and a due date are provided, tasks matching those keys
/// will be assigned the given due date, and all affected tasks (including
/// dependencies whose due dates changed) will be printed.
///
///   todo due "buy christmas presents" --on dec 24
#[derive(Debug, PartialEq, Eq, Parser, Default)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Due {
    /// Tasks to query or assign the due date.
    pub keys: Vec<Key>,
    /// The due date to assign or query.
    ///
    /// This is a human-readable description of a date or time, like "1 day" or
    /// "5pm".
    #[arg(long, alias = "in", alias = "on", num_args = 1..)]
    pub due: Option<Vec<String>>,
    /// Remove the explicit due date. If the implicit due date is inherited from
    /// an antidependency, it is retained.
    #[arg(long)]
    pub none: bool,
    /// Show completed tasks in queries.
    ///
    /// This is not used when assigning or unassigning due dates. It only
    /// affects the printed results when querying the source of a task's due
    /// date, querying all tasks with due dates, or querying all tasks due
    /// earlier than a given date.
    #[arg(long, short = 'd')]
    pub include_done: bool,
    /// Show blocked tasks in queries.
    ///
    /// By default, when querying for due tasks, only incomplete, unblocked
    /// tasks are shown. With this flag, blocked tasks are also shown.
    ///
    /// This is not used when assigning or unassigning due dates. It only
    /// affects the printed results when querying the source of a task's due
    /// date, querying all tasks with due dates, or querying all tasks due
    /// earlier than a given date.
    #[arg(long, short = 'b')]
    pub include_blocked: bool,
}
