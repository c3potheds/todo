use {
    clap::Parser,
    cli::Options,
    clock::{Clock, SystemClock},
    printing::{PrintingContext, ScriptingTodoPrinter, SimpleTodoPrinter},
    std::{fs::File, io::BufWriter},
    text_editing::{FakeTextEditor, ScrawlTextEditor},
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Deserialize error")]
    DeserializeError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
enum SaveError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Serialize error")]
    SerializeError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
enum TodoError {
    #[error("IO error")]
    NoDataDirectory,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Command line parsing error")]
    CommandLineParsing(#[from] clap::Error),
    #[error("Load error")]
    Load(#[from] LoadError),
    #[error("Save error")]
    Save(#[from] SaveError),
    #[error("Config error")]
    LoadConfig(#[from] config::LoadError),
}

type TodoResult = Result<(), TodoError>;

mod less;

fn main() -> TodoResult {
    let options = Options::parse();
    let project_dirs = directories::ProjectDirs::from("", "", "todo")
        .ok_or(TodoError::NoDataDirectory)?;

    let mut config_path = project_dirs.config_dir().to_path_buf();

    // If the directory does not exist, create it.
    if !config_path.exists() {
        std::fs::create_dir_all(&config_path)?;
    }

    config_path.push("config.json");
    let config = File::open(&config_path)
        .map_or_else(|_| Ok(config::Config::default()), config::load)?;

    let mut data_path = project_dirs.data_dir().to_path_buf();

    // If the directory does not exist, create it.
    if !data_path.exists() {
        std::fs::create_dir_all(&data_path)?;
    }

    data_path.push("data.json");

    let read_file_result = std::fs::read_to_string(&data_path);
    let mut model = match &read_file_result {
        Ok(s) => serde_json::from_str(s).map_err(LoadError::from)?,
        Err(_) => model::TodoList::default(),
    };

    use printing::Printable;
    let mutated = if let Some((term_width, _)) = term_size::dimensions_stdout()
    {
        let max_index_digits = std::cmp::max(
            // Add one for the minus sign for complete tasks.
            model.num_complete_tasks().checked_ilog10().unwrap_or(0) as usize
                + 1,
            model.num_incomplete_tasks().checked_ilog10().unwrap_or(1) as usize,
        );
        let mut printer = SimpleTodoPrinter {
            out: less::Less::new(&config.paginator_cmd)?,
            context: PrintingContext {
                max_index_digits,
                width: term_width,
                now: SystemClock.now(),
            },
        };
        app::todo(
            &mut model,
            &ScrawlTextEditor(&config.text_editor_cmd),
            &SystemClock,
            options,
        )
        .print(&mut printer)
    } else {
        app::todo(
            &mut model,
            &FakeTextEditor::no_user_output(),
            &SystemClock,
            options,
        )
        .print(&mut ScriptingTodoPrinter)
    };
    if mutated {
        let file = File::create(&data_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &model).map_err(SaveError::from)?;
    }
    Ok(())
}
