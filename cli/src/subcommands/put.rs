use clap::Parser;
use todo_lookup_key::Key;

#[derive(Debug, Default, PartialEq, Eq, Parser)]
pub struct Prepositions {
    /// Put the selected tasks before these tasks.
    #[arg(long, short = 'B', num_args = 1..)]
    pub before: Vec<Key>,
    /// Put the selected tasks after these tasks.
    #[arg(long, short = 'A', num_args = 1..)]
    pub after: Vec<Key>,
    /// Put the selected tasks 'by' these tasks.
    ///
    /// The selected tasks will have the same deps and adeps as these tasks.
    #[arg(long, short = 'Y', num_args = 1..)]
    pub by: Vec<Key>,
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
///
/// If you put task 't' by b, the result is:
///
///   a <- (b, t) <- c
///
/// ... where (b, t) represents two tasks, both of which depend on a and are
/// depended on by c.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Put {
    /// Selected task keys.
    #[arg(required = true, num_args = 1..)]
    pub keys: Vec<Key>,
    #[command(flatten)]
    pub preposition: Prepositions,
    /// Include affected complete tasks in the result.
    #[arg(long, short = 'd')]
    pub include_done: bool,
}
