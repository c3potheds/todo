extern crate app_dirs;
extern crate structopt;
extern crate term_size;
extern crate todo;

use app_dirs::AppDataType;
use app_dirs::AppInfo;
use std::fs::File;
use std::io::BufWriter;
use structopt::StructOpt;
use todo::app::todo;
use todo::cli::Options;
use todo::model::load;
use todo::model::save;
use todo::model::LoadError;
use todo::model::SaveError;
use todo::model::TodoList;
use todo::printing::PrintingContext;
use todo::printing::SimpleTodoPrinter;

#[derive(Debug)]
enum TodoError {
    NoDataDirectoryError(app_dirs::AppDirsError),
    IoError(std::io::Error),
    CommandLineParsingError(structopt::clap::Error),
    LoadError(LoadError),
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

impl From<LoadError> for TodoError {
    fn from(src: LoadError) -> Self {
        Self::LoadError(src)
    }
}

impl From<SaveError> for TodoError {
    fn from(src: SaveError) -> Self {
        Self::SaveError(src)
    }
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
    let mut model = File::open(&path)
        .map_or_else(|_| Ok(TodoList::new()), |file| load(file))?;
    let (term_width, _term_height) =
        term_size::dimensions_stdout().unwrap_or((80, 20));
    let printing_context = PrintingContext {
        // TODO: Get the number of tasks from the list.
        max_index_digits: 3,
        width: term_width,
    };
    todo(
        &mut model,
        &printing_context,
        &mut SimpleTodoPrinter {},
        &options,
    );
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    save(writer, &model)?;
    Ok(())
}
