use {
    serde::{Deserialize, Serialize},
    std::io::Read,
    thiserror::Error,
};

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
struct ConfigModel {
    paginator_cmd: Option<Vec<String>>,
    text_editor_cmd: Option<String>,
}

pub struct Config {
    pub paginator_cmd: Vec<String>,
    pub text_editor_cmd: String,
}

fn default_paginator_cmd() -> Vec<String> {
    vec!["less".to_string(), "-rFX".to_string()]
}

fn default_text_editor_cmd() -> String {
    "vim".to_string()
}

impl Config {
    fn new(model: ConfigModel) -> Self {
        Self {
            paginator_cmd: model
                .paginator_cmd
                .unwrap_or_else(&default_paginator_cmd),
            text_editor_cmd: model
                .text_editor_cmd
                .unwrap_or_else(&default_text_editor_cmd),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paginator_cmd: default_paginator_cmd(),
            text_editor_cmd: default_text_editor_cmd(),
        }
    }
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Load error")]
    DeserializeError(#[from] serde_json::Error),
}

pub fn load<R>(reader: R) -> Result<Config, LoadError>
where
    R: Read,
{
    Ok(Config::new(serde_json::from_reader(reader)?))
}
