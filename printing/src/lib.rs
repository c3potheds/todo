pub mod brief_printable_task;
mod format_util;
pub mod printable_error;
pub mod printable_info;
pub mod printable_task;
pub mod printable_warning;
pub mod scripting_todo_printer;
pub mod simple_todo_printer;
pub mod testing;
pub mod todo_printer;

pub use self::brief_printable_task::*;
pub use self::printable_error::*;
pub use self::printable_info::*;
pub use self::printable_task::*;
pub use self::printable_warning::*;
pub use self::scripting_todo_printer::*;
pub use self::simple_todo_printer::*;
pub use self::testing::*;
pub use self::todo_printer::*;

#[cfg(test)]
mod printable_error_test;
#[cfg(test)]
mod printable_info_test;
#[cfg(test)]
mod printable_task_test;
#[cfg(test)]
mod printable_warning_test;
#[cfg(test)]
mod simple_todo_printer_test;
#[cfg(test)]
mod testing_test;

#[derive(Default)]
pub struct PrintableAppSuccess<'list> {
    pub warnings: Vec<PrintableWarning>,
    pub infos: Vec<PrintableInfo>,
    pub tasks: Vec<PrintableTask<'list>>,
    pub mutated: bool,
}

pub type PrintableResult<'list> =
    Result<PrintableAppSuccess<'list>, Vec<PrintableError>>;

pub trait Printable {
    fn print(&self, printer: &mut impl TodoPrinter) -> bool;
}

impl Printable for PrintableResult<'_> {
    fn print(&self, printer: &mut impl TodoPrinter) -> bool {
        match self {
            Self::Ok(PrintableAppSuccess {
                warnings,
                infos,
                tasks,
                mutated,
            }) => {
                for warning in warnings {
                    printer.print_warning(warning);
                }
                for info in infos {
                    printer.print_info(info);
                }
                for task in tasks {
                    printer.print_task(task);
                }
                *mutated
            }
            Self::Err(errors) => {
                for error in errors {
                    printer.print_error(error);
                }
                false
            }
        }
    }
}
