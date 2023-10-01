use {clap::Parser, todo_lookup_key::Key};

/// Shows, marks, or unmarks tags.
///
/// A tag is a task whose description is inserted into the description of its
/// transitive dependencies and colored (on compatible terminals). Normally,
/// tags are created by using the --tag flag on the 'new' command, but this
/// subcommand can be used to turn existing tasks into tags, as well as turn
/// tags back into tasks with the --unmark flag.
///
/// If no keys are passed, and the --unmark flag is not passed, then this
/// subcommand will print all tags.
#[derive(Debug, Default, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Tag {
    /// Tasks to mark as tags. If none are given and --unmark is not passed,
    /// all existing tags are printed.
    #[arg(required = false)]
    pub keys: Vec<Key>,
    /// Tasks to unmark as tags.
    #[arg(long, short = 'u', num_args = 1.., required = false)]
    pub unmark: Vec<Key>,
    /// If passed, print affected completed tasks.
    #[arg(long, short = 'd')]
    pub include_done: bool,
}
