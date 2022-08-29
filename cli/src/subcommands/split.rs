use {clap::Parser, lookup_key::Key};

/// Turns a single task into multiple tasks.
///
/// This can be useful for adding detail to a plan, while preserving
/// dependency structure. For example, if you have a chain:
///
///   a <- b <- c
///
/// ... and you want to elaborate on the steps needed to accomplish 'b'
/// while keeping the dependencies on 'a' and antidependencies on 'c', you
/// can run:
///
///   todo split b --into b1 b2 b3
///
/// ... which will give:
///         b1 <-
///       /      \
///   a <-- b2 <-- c
///       \      /
///         b3 <-
///
/// If these are the only tasks in the list, the output of 'todo -b' will
/// be:
///
///   1) a
///   2) b1
///   3) b2
///   4) b3
///   5) c
///
/// In the above diagram, there are no dependency relationships between
/// 'b1', 'b2', and 'b3', which is why they're in the same column. If you
/// want to arrange the split task into a dependency chain, use --chain:
///
///    todo split b --into b1 b2 b3 --chain
///
/// ... which results in:
///
///  a <- b1 <- b2 <- b3 <- c
#[derive(Debug, PartialEq, Eq, Parser, Default)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Split {
    /// Tasks to split.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,

    /// Descriptions for new tasks.
    #[clap(long, required = true, min_values = 1)]
    pub into: Vec<String>,

    /// If passed, the results of the split will be put in a dependency chain.
    #[clap(long)]
    pub chain: bool,

    /// Add a prefix to the descriptions.
    #[clap(long, short = 'P', min_values = 1)]
    pub prefix: Vec<String>,

    /// If passed, keeps the original task, but blocks it on the new tasks.
    /// The budget of the original task will be transferred to the new tasks.
    #[clap(long, short = 'k')]
    pub keep: bool,

    /// If passed with the value "true", then the new tasks will be marked as
    /// tags. If passed with "false", then the new tasks will not be marked as
    /// tags. If not passed, then the new tasks will be marked as tags if the
    /// original task was marked as a tag and --keep was not passed, otherwise
    /// they will not be marked as tags.
    #[clap(long, short = 't')]
    pub tag: Option<bool>,
}
