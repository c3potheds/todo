use model::TodoList;
use std::io::Write;

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

pub fn load(s: &str) -> Result<TodoList<'_>, LoadError> {
    Ok(serde_json::from_str(s)?)
}

#[derive(Debug)]
pub enum SaveError {
    IoError(std::io::Error),
    SerializeError(serde_json::Error),
}

impl From<std::io::Error> for SaveError {
    fn from(src: std::io::Error) -> Self {
        Self::IoError(src)
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(src: serde_json::Error) -> Self {
        Self::SerializeError(src)
    }
}

pub fn save<W>(writer: W, model: &TodoList) -> Result<(), SaveError>
where
    W: Write,
{
    Ok(serde_json::to_writer(writer, model)?)
}
