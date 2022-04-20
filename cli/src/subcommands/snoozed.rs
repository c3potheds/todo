use structopt::StructOpt;

/// Shows snoozed tasks.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(verbatim_doc_comment)]
pub struct Snoozed {}
