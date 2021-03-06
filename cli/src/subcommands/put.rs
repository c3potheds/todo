use {clap::Parser, lookup_key::Key};

#[derive(Debug, PartialEq, Parser)]
pub struct Prepositions {
    /// Put the selected tasks before these tasks.
    #[clap(long, short = 'B', min_values = 1)]
    pub before: Vec<Key>,
    /// Put the selected tasks after these tasks.
    #[clap(long, short = 'A', min_values = 1)]
    pub after: Vec<Key>,
}

/// Puts tasks before or after other tasks.
///
/// Putting a task before another task is like blocking the latter on the
/// former, but also blocks the former on all the dependencies of the
/// latter. Likewise, putting a task after another task blocks the former on
/// the latter and blocks everything that was blocked on the latter on the
/// former.
///
/// For example, in a dependency chain:
///
///   a <- b <- c
///
/// If you put task "t" before b, the result is:
///
///   a <- t <- b <- c
///
/// If you put task "t" after b, the result is:
///
///   a <- b <- t <- c
#[derive(Debug, PartialEq, Parser)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Put {
    /// Selected task keys.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,
    #[clap(flatten)]
    pub preposition: Prepositions,
    /// Include affected complete tasks in the result.
    #[clap(long, short = 'd')]
    pub include_done: bool,
}
