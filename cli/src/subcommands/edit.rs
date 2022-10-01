use {clap::Parser, lookup_key::Key};

/// Edit the description of tasks.
///
/// This allows you to fix typos in task descriptions, or add new
/// information if needed. If no --desc is provided, then a text editor is
/// opened, which lets you edit the task descriptions interactively.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Edit {
    /// Tasks to edit.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
    /// The new description. If not set, a text editor is used.
    #[arg(long)]
    pub desc: Option<String>,
}
