use clap::Parser;

/// Shows snoozed tasks.
#[derive(Debug, PartialEq, Eq, Parser)]
#[command(verbatim_doc_comment)]
pub struct Snoozed {}
