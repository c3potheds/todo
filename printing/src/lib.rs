mod brief_printable_task;
mod format_util;
mod printable_error;
mod printable_info;
mod printable_task;
mod printable_warning;
mod scripting_todo_printer;
mod simple_todo_printer;
mod todo_printer;

pub use self::brief_printable_task::*;
pub use self::printable_error::*;
pub use self::printable_info::*;
pub use self::printable_task::*;
pub use self::printable_warning::*;
pub use self::scripting_todo_printer::*;
pub use self::simple_todo_printer::*;
pub use self::todo_printer::*;

#[cfg(test)]
mod printable_error_test;
#[cfg(test)]
mod printable_info_test;
#[cfg(test)]
mod printable_result_test;
#[cfg(test)]
mod printable_task_test;
#[cfg(test)]
mod printable_warning_test;
#[cfg(test)]
mod simple_todo_printer_test;

#[derive(Default)]
pub struct PrintableAppSuccess<'list> {
    pub warnings: Vec<PrintableWarning>,
    pub infos: Vec<PrintableInfo>,
    pub tasks: Vec<PrintableTask<'list>>,
    pub mutated: bool,
}

pub type PrintableResult<'list> =
    Result<PrintableAppSuccess<'list>, Vec<PrintableError>>;

pub trait Printable<'a> {
    fn max_index_digits(&self) -> usize;
    fn print(&self, printer: &mut impl TodoPrinter<'a>) -> bool;
}

fn count_digits(n: i32) -> usize {
    match n.cmp(&0) {
        std::cmp::Ordering::Less => 1 + count_digits(-n),
        std::cmp::Ordering::Equal => 1,
        std::cmp::Ordering::Greater => 1 + n.ilog10() as usize,
    }
}

impl<'a, 'list> Printable<'a> for PrintableResult<'list>
where
    'list: 'a,
{
    fn max_index_digits(&self) -> usize {
        match self {
            Ok(success) => success
                .tasks
                .iter()
                .map(|t| t.number)
                .map(count_digits)
                .max()
                .unwrap_or(1),
            Err(_) => 1,
        }
    }

    fn print(&self, printer: &mut impl TodoPrinter<'a>) -> bool {
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
