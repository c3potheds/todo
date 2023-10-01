use std::io::IsTerminal;

use {
    clap::Parser,
    std::{fs::File, io::BufWriter},
    thiserror::Error,
    todo_cli::Options,
    todo_clock::{Clock, SystemClock},
    todo_printing::{PrintingContext, ScriptingTodoPrinter, SimpleTodoPrinter},
    todo_text_editing::{FakeTextEditor, ScrawlTextEditor},
};

#[derive(Debug, Error)]
enum LoadError {
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
    #[error("Could not create config directory")]
    CouldNotCreateConfigDirectory(std::io::Error),
    #[error("Could not create data directory")]
    CouldNotCreateDataDirectory(std::io::Error),
    #[error("Could not spawn paginator")]
    CouldNotSpawnPaginator(less::CouldNotSpawnPaginator),
    #[error("Load error")]
    Load(#[from] LoadError),
    #[error("Save error")]
    Save(#[from] SaveError),
    #[error("Config error")]
    LoadConfig(#[from] todo_config::LoadError),
}

type TodoResult = Result<(), TodoError>;

mod less;

fn main() -> TodoResult {
    let options = Options::parse();
    let project_dirs = directories::ProjectDirs::from("", "", "todo")
        .ok_or(TodoError::NoDataDirectory)?;

    let config = {
        let mut config_path = project_dirs.config_dir().to_path_buf();

        // If the directory does not exist, create it.
        if !config_path.exists() {
            std::fs::create_dir_all(&config_path)
                .map_err(TodoError::CouldNotCreateConfigDirectory)?;
        }

        config_path.push("config.json");
        File::open(&config_path).map_or_else(
            |_| Ok(todo_config::Config::default()),
            todo_config::load,
        )?
    };

    let mut data_path = project_dirs.data_dir().to_path_buf();

    // If the directory does not exist, create it.
    if !data_path.exists() {
        std::fs::create_dir_all(&data_path)
            .map_err(TodoError::CouldNotCreateDataDirectory)?;
    }

    data_path.push("data.json");

    let read_file_result = std::fs::read_to_string(&data_path);
    let mut model = match &read_file_result {
        Ok(s) => serde_json::from_str(s).map_err(LoadError::from)?,
        Err(_) => todo_model::TodoList::default(),
    };

    let app_result = if std::io::stdout().is_terminal() {
        todo_app::todo(
            &mut model,
            &ScrawlTextEditor(&config.text_editor_cmd),
            &SystemClock,
            options,
        )
    } else {
        todo_app::todo(
            &mut model,
            &FakeTextEditor::no_user_output(),
            &SystemClock,
            options,
        )
    };
    use todo_printing::Printable;
    let mutated = if std::io::stdout().is_terminal() {
        let mut printer = SimpleTodoPrinter {
            out: less::Less::new(&config.paginator_cmd)
                .map_err(TodoError::CouldNotSpawnPaginator)?,
            context: PrintingContext {
                max_index_digits: app_result.max_index_digits(),
                width: term_size::dimensions_stdout()
                    .map(|(w, _)| w)
                    .unwrap_or(80),
                now: SystemClock.now(),
            },
        };
        app_result.print(&mut printer)
    } else {
        app_result.print(&mut ScriptingTodoPrinter)
    };
    if mutated {
        let file = File::create(&data_path).map_err(SaveError::from)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &model).map_err(SaveError::from)?;
    }
    Ok(())
}
