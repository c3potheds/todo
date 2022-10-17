use super::*;

pub trait TodoPrinter<'a> {
    fn print_task(&mut self, task: &PrintableTask<'a>);
    fn print_info(&mut self, info: &PrintableInfo);
    fn print_warning(&mut self, warning: &PrintableWarning);
    fn print_error(&mut self, error: &PrintableError);
}
