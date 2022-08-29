use {clap::Parser, lookup_key::Key};

/// Add a prefix to the given task descriptions.
#[derive(Debug, PartialEq, Parser)]
#[clap(allow_negative_numbers(true), verbatim_doc_comment)]
pub struct Prefix {
    /// Tasks to add a prefix to.
    #[clap(required = true, min_values = 1)]
    pub keys: Vec<Key>,

    /// Prefix to add to the descriptions.
    #[clap(long, short = 'P', alias = "add", required = true, min_values = 1)]
    pub prefix: Vec<String>,
}
