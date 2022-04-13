use model::TodoList;
use std::io::Write;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Deserialize error")]
    DeserializeError(#[from] serde_json::Error),
}

pub fn load(s: &str) -> Result<TodoList<'_>, LoadError> {
    Ok(serde_json::from_str(s)?)
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Serialize error")]
    SerializeError(#[from] serde_json::Error),
}

pub fn save<W>(writer: W, list: &TodoList) -> Result<(), SaveError>
where
    W: Write,
{
    Ok(serde_json::to_writer(writer, list)?)
}
