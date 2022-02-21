extern crate bisection;

use std::io::Read;
use std::io::Write;

mod task_id;
pub use self::task_id::*;

mod task_status;
pub use self::task_status::*;

mod duration;
pub use self::duration::*;

mod task;
pub use self::task::*;

mod layering;
use self::layering::*;

mod task_set;
pub use self::task_set::*;

mod todo_list;
pub use self::todo_list::*;

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

pub fn load<R>(reader: R) -> Result<TodoList, LoadError>
where
    R: Read,
{
    Ok(serde_json::from_reader(reader)?)
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
