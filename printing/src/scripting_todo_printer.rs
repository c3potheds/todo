use {
    crate::{
        PrintableError, PrintableInfo, PrintableTask, PrintableWarning,
        TodoPrinter,
    },
    std::io::Write,
};

pub struct ScriptingTodoPrinter;

impl<'a> TodoPrinter<'a> for ScriptingTodoPrinter {
    fn print_task(&mut self, task: &PrintableTask<'a>) {
        writeln!(std::io::stdout(), "{}", task.number).unwrap_or_default();
    }

    fn print_info(&mut self, info: &PrintableInfo) {
        writeln!(std::io::stderr(), "{}", info).unwrap_or_default();
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        writeln!(std::io::stderr(), "{}", warning).unwrap_or_default();
    }

    fn print_error(&mut self, error: &PrintableError) {
        writeln!(std::io::stderr(), "{}", error).unwrap_or_default();
    }
}
