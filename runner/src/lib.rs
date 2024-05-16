use std::io::IsTerminal;

use {
    std::{fs::File, io::BufWriter},
    thiserror::Error,
    todo_app::Application,
    todo_app::Mutated,
    todo_clock::{Clock, SystemClock},
    todo_printing::{PrintingContext, ScriptingTodoPrinter, SimpleTodoPrinter},
    todo_text_editing::{FakeTextEditor, ScrawlTextEditor},
};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Deserialize error")]
    DeserializeError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Serialize error")]
    SerializeError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("IO error")]
    NoDataDirectory,
    #[error("Could not create config directory")]
    CouldNotCreateConfigDirectory(std::io::Error),
    #[error("Could not create data directory")]
    CouldNotCreateDataDirectory(std::io::Error),
    #[error("Load error")]
    Load(#[from] LoadError),
    #[error("Save error")]
    Save(#[from] SaveError),
    #[error("Config error")]
    LoadConfig(#[from] todo_config::LoadError),
}

mod less;

pub type TodoResult = Result<(), TodoError>;

pub fn run(app: impl Application) -> TodoResult {
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

    let mutated = if std::io::stdout().is_terminal() {
        use either::{Left, Right};
        let out = match less::Less::new(&config.paginator_cmd) {
            Ok(paginator) => Left(paginator),
            Err(e) => {
                eprintln!("Could not spawn paginator: {e:?}");
                Right(std::io::stdout())
            }
        };
        app.run(
            &mut model,
            &ScrawlTextEditor(&config.text_editor_cmd),
            &SystemClock,
            |max_index_digits| SimpleTodoPrinter {
                // TODO: fall back on stdout if paginator cannot spawn.
                out,
                context: PrintingContext {
                    max_index_digits,
                    width: terminal_size::terminal_size()
                        .map(|(terminal_size::Width(w), _)| w)
                        .unwrap_or(80) as usize,
                    now: SystemClock.now(),
                },
            },
        )
    } else {
        app.run(
            &mut model,
            &FakeTextEditor::no_user_output(),
            &SystemClock,
            |_| ScriptingTodoPrinter,
        )
    };
    if mutated == Mutated::Yes {
        let file = File::create(&data_path).map_err(SaveError::from)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &model).map_err(SaveError::from)?;
    }
    Ok(())
}
