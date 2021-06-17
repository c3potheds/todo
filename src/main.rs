extern crate atty;
extern crate directories;
extern crate structopt;
extern crate term_size;
extern crate todo;

use std::fs::File;
use std::io::BufWriter;
use structopt::StructOpt;
use todo::app;
use todo::cli::Options;
use todo::clock::Clock;
use todo::clock::SystemClock;
use todo::config;
use todo::long_output;
use todo::model;
use todo::model::TodoList;
use todo::printing::PrintingContext;
use todo::printing::ScriptingTodoPrinter;
use todo::printing::SimpleTodoPrinter;
use todo::text_editing::ScrawlTextEditor;

#[derive(Debug)]
enum TodoError {
    NoDataDirectoryError,
    IoError(std::io::Error),
    CommandLineParsingError(structopt::clap::Error),
    LoadError(model::LoadError),
    SaveError(model::SaveError),
    LoadConfigError(config::LoadError),
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

impl From<model::LoadError> for TodoError {
    fn from(src: model::LoadError) -> Self {
        Self::LoadError(src)
    }
}

impl From<model::SaveError> for TodoError {
    fn from(src: model::SaveError) -> Self {
        Self::SaveError(src)
    }
}

impl From<config::LoadError> for TodoError {
    fn from(src: config::LoadError) -> Self {
        Self::LoadConfigError(src)
    }
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
    let project_dirs = match directories::ProjectDirs::from("", "", "todo") {
        Some(project_dirs) => project_dirs,
        None => return Err(TodoError::NoDataDirectoryError),
    };

    let mut config_path = project_dirs.config_dir().to_path_buf();
    config_path.push("config.json");
    let config = File::open(&config_path).map_or_else(
        |_| Ok(config::Config::default()),
        |file| config::load(file),
    )?;

    let mut data_path = project_dirs.data_dir().to_path_buf();
    data_path.push("data.json");
    let mut model = File::open(&data_path)
        .map_or_else(|_| Ok(TodoList::new()), |file| model::load(file))?;

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
            &todo::text_editing::FakeTextEditor::no_user_output(),
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
