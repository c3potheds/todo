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
enum TodoError {
    #[error("IO error")]
    NoDataDirectory,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Command line parsing error")]
    CommandLineParsing(#[from] clap::Error),
    #[error("Load error")]
    Load(#[from] model::LoadError),
    #[error("Save error")]
    Save(#[from] model::SaveError),
    #[error("Config error")]
    LoadConfig(#[from] config::LoadError),
}

type TodoResult = Result<(), TodoError>;

fn log10(n: usize) -> usize {
    if n < 10 {
        1
    } else {
        1 + log10(n / 10)
    }
}

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
        Ok(s) => model::load(s)?,
        Err(_) => model::TodoList::default(),
    };

    use printing::Printable;
    let mutated = if let Some((term_width, _)) = term_size::dimensions_stdout()
    {
        let mut printer = SimpleTodoPrinter {
            out: less::Less::new(&config.paginator_cmd)?,
            context: PrintingContext {
                max_index_digits: std::cmp::max(
                    // Add one for the minus sign for complete tasks.
                    log10(model.num_complete_tasks()) + 1,
                    log10(model.num_incomplete_tasks()),
                ),
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
        model::save(writer, &model)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn log10_examples() {
        assert_eq!(log10(0), 1);
        assert_eq!(log10(5), 1);
        assert_eq!(log10(10), 2);
        assert_eq!(log10(99), 2);
        assert_eq!(log10(100), 3);
        assert_eq!(log10(999), 3);
        assert_eq!(log10(1000), 4);
        assert_eq!(log10(123456789), 9);
    }
}
