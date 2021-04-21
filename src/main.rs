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
use todo::long_output;
use todo::model::load;
use todo::model::save;
use todo::model::LoadError;
use todo::model::SaveError;
use todo::model::TodoList;
use todo::printing::PrintingContext;
use todo::printing::SimpleTodoPrinter;
use todo::text_editing::ScrawlTextEditor;

#[derive(Debug)]
enum TodoError {
    NoDataDirectoryError,
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
    let mut path = project_dirs.data_dir().to_path_buf();
    path.push("data.json");

    let mut model = File::open(&path)
        .map_or_else(|_| Ok(TodoList::new()), |file| load(file))?;
    let (term_width, term_height) =
        term_size::dimensions_stdout().unwrap_or((80, 20));
    let printing_context = PrintingContext {
        max_index_digits: std::cmp::max(
            // Add one for the minus sign for complete tasks.
            log10(model.num_complete_tasks()) + 1,
            log10(model.num_incomplete_tasks()),
        ),
        width: term_width,
        now: SystemClock.now(),
    };
    // Subtract 1 from the term height to leave room for the input prompt after
    // the program finishes.
    let mut out = long_output::max_lines(term_height - 1)
        .primary(std::io::stdout())
        .alternate(|| long_output::Less::new().unwrap());
    let mut printer = SimpleTodoPrinter {
        out: &mut out,
        context: &printing_context,
    };
    let text_editor = ScrawlTextEditor;

    // Hack.
    model
        .all_tasks()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            model.update_implicits(id);
        });

    app::todo(
        &mut model,
        &mut printer,
        &text_editor,
        &SystemClock,
        options,
    );
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    save(writer, &model)?;
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
