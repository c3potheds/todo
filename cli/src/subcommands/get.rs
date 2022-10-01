use clap::ArgGroup;

use {clap::Parser, lookup_key::Key};

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
#[derive(Debug, Default, PartialEq, Eq, Parser)]
#[command(
    allow_negative_numbers(true),
    verbatim_doc_comment,
    group(
        ArgGroup::new("context")
            .args(&["no_context", "blocked_by", "blocking"])
    )
)]
pub struct Get {
    /// Tasks to explore.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
    /// Show completed deps if no given task is complete.
    ///
    /// Completed deps and adeps will be shown even without this flag if any of
    /// the tasks that match the given keys are themselves complete.
    #[arg(long, short = 'd')]
    pub include_done: bool,
    /// Do not show context (transitive deps and adeps of given tasks).
    #[arg(long, short = 'n')]
    pub no_context: bool,
    /// Only show transitive deps of given tasks.
    #[arg(long, short = 'p')]
    pub blocked_by: bool,
    /// Only show transitive adeps of given tasks.
    #[arg(long, short = 'b')]
    pub blocking: bool,
}
