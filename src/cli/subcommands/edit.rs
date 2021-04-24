pub use super::Key;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct Edit {
    /// Tasks to edit.
    pub keys: Vec<Key>,
    /// The new description. If not set, a text editor is used.
    #[structopt(long)]
    pub desc: Option<String>,
}
