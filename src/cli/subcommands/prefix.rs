pub use super::Key;
use structopt::StructOpt;

/// Add a prefix to the given task descriptions.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Prefix {
    /// Tasks to add a prefix to.
    #[structopt(required = true, min_values = 1)]
    pub keys: Vec<Key>,

    /// Prefix to add to the descriptions.
    #[structopt(
        long,
        short = "P",
        alias = "add",
        required = true,
        min_values = 1
    )]
    pub prefix: Vec<String>,
}
