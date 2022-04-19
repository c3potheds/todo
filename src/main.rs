extern crate atty;
extern crate directories;
extern crate structopt;
extern crate term_size;
extern crate thiserror;

extern crate clock;
extern crate long_output;
extern crate model;
extern crate text_editing;
extern crate todo;

use clock::Clock;
use clock::SystemClock;
use std::fs::File;
use std::io::BufWriter;
use structopt::StructOpt;
use text_editing::ScrawlTextEditor;
use text_editing::FakeTextEditor;
use thiserror::Error;
use todo::app;
use todo::cli::Options;
use todo::config;
use todo::printing::PrintingContext;
use todo::printing::ScriptingTodoPrinter;
use todo::printing::SimpleTodoPrinter;

#[derive(Debug, Error)]
enum TodoError {
    #[error("IO error")]
    NoDataDirectory,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Command line parsing error")]
    CommandLineParsing(#[from] structopt::clap::Error),
    #[error("Load error")]
    Load(#[from] model::LoadError),
    #[error("Save error")]
    Save(#[from] model::SaveError),
    #[error("Config error")]
    LoadConfig(#[from] config::LoadError),
}

type TodoResult = Result<(), TodoError>;

fn log10(n: usize) -> usize {
    let mut log = 1;
    let mut base = 1;
    loop {
        if n / base < 10 {
            return log;
        }
        log += 1;
        base *= 10;
    }
}

fn main() -> TodoResult {
    let options = Options::from_args();
    let project_dirs = directories::ProjectDirs::from("", "", "todo")
        .ok_or(TodoError::NoDataDirectory)?;

    let mut config_path = project_dirs.config_dir().to_path_buf();
    config_path.push("config.json");
    let config = File::open(&config_path)
        .map_or_else(|_| Ok(config::Config::default()), config::load)?;

    let mut data_path = project_dirs.data_dir().to_path_buf();
    data_path.push("data.json");

    let read_file_result = std::fs::read_to_string(&data_path);
    let mut model = match &read_file_result {
        Ok(s) => model::load(s)?,
        Err(_) => model::TodoList::default(),
    };

    if atty::is(atty::Stream::Stdout) {
        let (term_width, term_height) =
            term_size::dimensions_stdout().unwrap_or((80, 20));

        let mut printer = SimpleTodoPrinter {
            // Subtract 1 from the term height to leave room for the input prompt
            // after the program finishes.
            out: long_output::max_lines(term_height - 1)
                .primary(std::io::stdout())
                .alternate(|| {
                    long_output::Less::new(
                        &config.paginator_cmd[0],
                        &config.paginator_cmd[1..],
                    )
                    .unwrap()
                }),
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
            &mut printer,
            &ScrawlTextEditor(&config.text_editor_cmd),
            &SystemClock,
            options,
        );
    } else {
        app::todo(
            &mut model,
            &mut ScriptingTodoPrinter,
            &FakeTextEditor::no_user_output(),
            &SystemClock,
            options,
        );
    }
    let file = File::create(&data_path)?;
    let writer = BufWriter::new(file);
    model::save(writer, &model)?;
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
