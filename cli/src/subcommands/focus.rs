use clap::Parser;
use todo_lookup_key::Key;

/// Sets or removes the focus time for tasks.
///
/// Focus time determines when a task is considered "in focus". This affects
/// whether it appears in filtered views or receives priority notifications
/// (depending on configuration or future features). Tasks inherit focus
/// status from their antidependencies: a task is only in focus if it AND
/// all tasks blocking it are in focus.
///
/// You must provide tasks to modify and either '--on <PREDICATE>' to set
/// a focus time or '--none' to remove it.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(verbatim_doc_comment)]
pub struct Focus {
    /// Tasks to set or remove focus time for.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,

    /// The focus predicate string to apply.
    ///
    /// Examples: "weekdays", "mon", "tue", "mwf", "9am-5pm", "14:00-17:30",
    /// "after 6pm", "before 8:00". See 'todo help focus' for more.
    #[arg(long, conflicts_with = "none", value_name = "PREDICATE")]
    pub on: Option<String>,

    /// Remove the focus time constraint from the tasks.
    #[arg(long, conflicts_with = "on")]
    pub none: bool,
}
