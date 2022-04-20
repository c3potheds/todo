use super::Key;
use structopt::StructOpt;

/// Unsnoozes snoozed tasks.
///
/// Unsnoozed tasks with no incomplete deps will become visible, moved to the
/// end of the list of unblocked incomplete tasks.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Unsnooze {
    /// Tasks to unsnooze.
    #[structopt(required = true, min_values = 1)]
    pub keys: Vec<Key>,
}
