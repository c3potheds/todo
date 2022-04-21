use {
    crate::{PrintableError, PrintableTask, PrintableWarning, TodoPrinter},
    std::io::Write,
};

pub struct ScriptingTodoPrinter;

impl TodoPrinter for ScriptingTodoPrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        writeln!(std::io::stdout(), "{}", task.number).unwrap_or_default();
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        writeln!(std::io::stderr(), "{}", warning).unwrap_or_default();
    }

    fn print_error(&mut self, error: &PrintableError) {
        writeln!(std::io::stderr(), "{}", error).unwrap_or_default();
    }
}
