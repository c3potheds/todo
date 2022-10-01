use {clap::Parser, lookup_key::Key};

/// Shows bottom-level tasks, i.e. tasks with no dependencies.
///
/// This is most useful for showing the *direct* anti-dependencies of given
/// tasks. For example, if you have a task "b" that is blocked on "a", then
/// running `todo bottom a` will show "b" as a bottom-level task.
#[derive(Debug, PartialEq, Eq, Parser, Default)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Bottom {
    /// Tasks to find the bottom level above. If none are specified, shows the
    /// bottom-level tasks, i.e. tasks with no dependencies. These may serve as
    /// "starting points" for high-level projects.
    pub keys: Vec<Key>,

    /// If passed, shows bottom-level complete tasks too.
    #[clap(long, short = 'd')]
    pub include_done: bool,
}
