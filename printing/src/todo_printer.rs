use super::*;

pub trait TodoPrinter {
    fn print_task(&mut self, task: &PrintableTask);
    fn print_warning(&mut self, warning: &PrintableWarning);
    fn print_error(&mut self, error: &PrintableError);
}
