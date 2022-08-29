use clap::Parser;

/// Shows snoozed tasks.
#[derive(Debug, PartialEq, Eq, Parser)]
#[clap(verbatim_doc_comment)]
pub struct Snoozed {}
