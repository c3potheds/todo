pub use super::Key;
use structopt::StructOpt;

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
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Put {
    /// Selected task keys.
    pub keys: Vec<Key>,
    /// Put the selected tasks before these tasks.
    #[structopt(long, short = "b")]
    pub before: Vec<Key>,
    /// Put the selected tasks after these tasks.
    #[structopt(long, short = "a")]
    pub after: Vec<Key>,
    /// Include affected complete tasks in the result.
    #[structopt(long, short = "d")]
    pub include_done: bool,
}
