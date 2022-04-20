use {lookup_key::Key, structopt::StructOpt};

/// Merges two or more tasks into one.
///
/// The merged task will retain the dependency structure of the tasks it was
/// merged from. Its due date will be the earliest explicit due date of the
/// constituents and its priority will be the lowest explicit priority of
/// the constituents.
///
/// This is the opposite of 'split'.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Merge {
    /// Tasks to merge.
    #[structopt(required = true, min_values = 2)]
    pub keys: Vec<Key>,
    /// Description of new task to merge into.
    #[structopt(long)]
    pub into: String,
}
