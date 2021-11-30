use super::PrintableError;
use super::PrintableTask;
use super::PrintableWarning;
use super::TodoPrinter;

pub struct ScriptingTodoPrinter;

impl TodoPrinter for ScriptingTodoPrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        println!("{}", task.number);
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        eprintln!("{}", warning);
    }

    fn print_error(&mut self, error: &PrintableError) {
        eprintln!("{}", error);
    }
}
