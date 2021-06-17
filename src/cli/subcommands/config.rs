pub use super::Key;
use structopt::StructOpt;

/// Query or modify options.
///
/// If 'key' is omitted, then all config values are shown. If 'key' is provided
/// and 'value' is not, then the current value for the given key is printed. If
/// 'key' and 'value' are provided, then the config value is mutated and printed
/// if successful. If '--reset' is passed, the config value is restored to the
/// default.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::AllowNegativeNumbers,
    verbatim_doc_comment,
)]
pub struct Config {
    /// The key to change. Must be a valid config key
    pub key: Option<String>,
    /// If provided, sets the config value for the given key.
    pub value: Vec<String>,
    /// If passed, resets the config value to the default.
    #[structopt(long)]
    pub reset: bool,
}
