use std::io::Read;

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
struct ConfigModel {
    paginator_cmd: Option<Vec<String>>,
}

pub struct Config {
    pub paginator_cmd: Vec<String>,
}

fn default_paginator_cmd() -> Vec<String> {
    vec!["less".to_string(), "-rX".to_string()]
}

impl Config {
    fn new(model: ConfigModel) -> Self {
        Self {
            paginator_cmd: model
                .paginator_cmd
                .unwrap_or_else(&default_paginator_cmd),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paginator_cmd: default_paginator_cmd(),
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    IoError(std::io::Error),
    DeserializeError(serde_json::Error),
}

impl From<std::io::Error> for LoadError {
    fn from(src: std::io::Error) -> Self {
        Self::IoError(src)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(src: serde_json::Error) -> Self {
        Self::DeserializeError(src)
    }
}
pub fn load<R>(reader: R) -> Result<Config, LoadError>
where
    R: Read,
{
    Ok(Config::new(serde_json::from_reader(reader)?))
}