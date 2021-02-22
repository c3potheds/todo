extern crate app_dirs;
extern crate structopt;
extern crate todo;

use app_dirs::AppDataType;
use app_dirs::AppInfo;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use structopt::StructOpt;
use todo::app::todo;
use todo::cli::Options;
use todo::model::TodoList;
use todo::printing::SimpleTodoPrinter;

#[derive(Debug)]
enum TodoError {
    NoDataDirectoryError(app_dirs::AppDirsError),
    IoError(std::io::Error),
    CommandLineParsingError(structopt::clap::Error),
    SaveError(SaveError),
}

impl From<std::io::Error> for TodoError {
    fn from(src: std::io::Error) -> Self {
        Self::IoError(src)
    }
}

impl From<structopt::clap::Error> for TodoError {
    fn from(src: structopt::clap::Error) -> Self {
        Self::CommandLineParsingError(src)
    }
}

impl From<app_dirs::AppDirsError> for TodoError {
    fn from(src: app_dirs::AppDirsError) -> Self {
        Self::NoDataDirectoryError(src)
    }
}

impl From<SaveError> for TodoError {
    fn from(src: SaveError) -> Self {
        Self::SaveError(src)
    }
}

enum LoadError {
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

fn load_model<P: AsRef<Path>>(path: P) -> Result<TodoList, LoadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

#[derive(Debug)]
enum SaveError {
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

fn save_model<P: AsRef<Path>>(
    path: P,
    model: &TodoList,
) -> Result<(), SaveError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    Ok(serde_json::to_writer(writer, model)?)
}

type TodoResult = Result<(), TodoError>;

fn main() -> TodoResult {
    let options = Options::from_args();
    let app_info = AppInfo {
        name: "todo",
        author: "Simeon Anfinrud",
    };
    let mut path = app_dirs::app_root(AppDataType::UserData, &app_info)?;
    path.push("data.json");
    let mut model = load_model(&path).unwrap_or_else(|_| TodoList::new());
    todo(&mut model, &mut SimpleTodoPrinter {}, &options);
    save_model(&path, &model)?;
    Ok(())
}
