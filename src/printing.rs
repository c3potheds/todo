use ansi_term::Color;
use cli::Key;
use model::TaskStatus;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct PrintingContext {
    /// The number of digits that task numbers may have, including a minus sign.
    pub max_index_digits: usize,
    /// The number of columns to render task descriptions in (not used yet).
    pub width: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action {
    None,
    New,
    Check,
    Uncheck,
    Lock,
    Unlock,
    Select,
}

pub struct PrintableTask<'a> {
    pub context: &'a PrintingContext,
    pub desc: &'a str,
    pub number: i32,
    pub status: TaskStatus,
    pub action: Action,
}

#[derive(Debug)]
pub enum PrintableWarning {
    NoMatchFoundForKey {
        requested_key: Key,
    },
    CannotCheckBecauseAlreadyComplete {
        cannot_check: i32,
    },
    CannotRestoreBecauseAlreadyIncomplete {
        cannot_restore: i32,
    },
    CannotUnblockBecauseTaskIsNotBlocked {
        cannot_unblock: i32,
        requested_unblock_from: i32,
    },
}

#[derive(Debug)]
pub enum PrintableError {
    CannotCheckBecauseBlocked {
        cannot_check: i32,
        blocked_by: Vec<i32>,
    },
    CannotRestoreBecauseAntidependencyIsComplete {
        cannot_restore: i32,
        complete_antidependencies: Vec<i32>,
    },
    CannotBlockBecauseWouldCauseCycle {
        cannot_block: i32,
        requested_dependency: i32,
        // TODO: print the path between requested_dependency and cannot_block.
        // cycles: Vec<Vec<i32>>,
    },
}

const ANSI_OFFSET: usize = 10;

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Action::None => write!(f, "{}", "   "),
            Action::New => write!(f, "{}", Color::Green.paint("NEW")),
            Action::Check => write!(f, "{}", Color::Green.paint("[✓]")),
            Action::Uncheck => write!(f, "{}", Color::Yellow.paint("[ ]")),
            Action::Lock => write!(f, " {}", Color::Red.paint("🔒")),
            Action::Unlock => write!(f, " {}", Color::Green.paint("🔓")),
            Action::Select => write!(f, " * "),
        }
    }
}

fn format_key(key: &Key) -> String {
    match key {
        &Key::ByNumber(n) => format!("\"{}\"", n),
    }
}

fn format_number(number: i32, status: TaskStatus) -> String {
    let style = match &status {
        TaskStatus::Complete => Color::Green,
        TaskStatus::Incomplete => Color::Yellow,
        TaskStatus::Blocked => Color::Red,
    };
    let mut indexing = number.to_string();
    indexing.push_str(")");
    format!("{}", style.paint(&indexing))
}

fn format_numbers<I: IntoIterator<Item = i32>>(
    numbers: I,
    status: TaskStatus,
) -> String {
    numbers
        .into_iter()
        .map(|n| format_number(n, status))
        .collect::<Vec<_>>()
        .join(", ")
}

impl<'a> Display for PrintableTask<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {:>width$} {}",
            self.action,
            format_number(self.number, self.status),
            self.desc,
            width = self.context.max_index_digits + ANSI_OFFSET,
        )
    }
}

impl Display for PrintableWarning {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            format!("{}", Color::Yellow.paint("warning")),
            match self {
                PrintableWarning::NoMatchFoundForKey { requested_key } =>
                    format!("No match found for {}", format_key(requested_key)),
                PrintableWarning::CannotCheckBecauseAlreadyComplete {
                    cannot_check,
                } => format!(
                    "Task {} is already complete",
                    format_number(*cannot_check, TaskStatus::Complete)
                ),
                PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                    cannot_restore,
                } => format!(
                    "Task {} is already incomplete",
                    format_number(*cannot_restore, TaskStatus::Incomplete)
                ),
                PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                    cannot_unblock,
                    requested_unblock_from,
                } => format!(
                    "Task {} is not blocked by {}",
                    format_number(*cannot_unblock, TaskStatus::Incomplete),
                    format_number(
                        *requested_unblock_from,
                        TaskStatus::Incomplete
                    )
                ),
            }
        )
    }
}

impl Display for PrintableError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            format!("{}", Color::Red.paint("error")),
            match self {
                PrintableError::CannotCheckBecauseBlocked {
                    cannot_check,
                    blocked_by,
                } => format!(
                    "Cannot complete {} because it is blocked by {}",
                    format_number(*cannot_check, TaskStatus::Blocked),
                    format_numbers(
                        blocked_by.into_iter().copied(),
                        TaskStatus::Incomplete
                    ),
                ),
                PrintableError::CannotRestoreBecauseAntidependencyIsComplete{
                    cannot_restore,
                    complete_antidependencies,
                } => format!(
                    "Cannot restore {} because it blocks complete tasks {}",
                    format_number(*cannot_restore, TaskStatus::Complete),
                    format_numbers(
                        complete_antidependencies.into_iter().copied(),
                        TaskStatus::Complete)
                ),
                PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block,
                    requested_dependency,
                } => format!(
                    "Cannot block {} on {} because it would create a cycle",
                    format_number(*cannot_block, TaskStatus::Incomplete),
                    format_number(*requested_dependency, TaskStatus::Blocked),
                ),
            }
        )
    }
}

pub trait TodoPrinter {
    fn print_task(&mut self, task: &PrintableTask);
    fn print_warning(&mut self, warning: &PrintableWarning);
    fn print_error(&mut self, error: &PrintableError);
}

pub struct SimpleTodoPrinter {}

impl TodoPrinter for SimpleTodoPrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        println!("{}", task);
    }
    fn print_warning(&mut self, warning: &PrintableWarning) {
        println!("{}", warning);
    }
    fn print_error(&mut self, error: &PrintableError) {
        println!("{}", error);
    }
}

#[derive(Debug)]
#[cfg(test)]
struct PrintedTaskInfo {
    desc: String,
    number: i32,
    status: TaskStatus,
    action: Action,
}

#[cfg(test)]
pub struct FakePrinter {
    record: Vec<PrintedTaskInfo>,
}

#[derive(Debug)]
#[cfg(test)]
pub enum Expect<'a> {
    Desc(&'a str),
    Number(i32),
    Status(TaskStatus),
    Action(Action),
}

#[cfg(test)]
impl<'a> Expect<'a> {
    fn validate(&self, info: &PrintedTaskInfo) {
        match self {
            Expect::Desc(desc) => {
                if desc != &info.desc {
                    panic!(
                        "Unexpected description: {:?}. (Expected {:?})",
                        &info.desc, desc
                    );
                }
            }
            Expect::Number(number) => {
                if *number != info.number {
                    panic!(
                        "Unexpected number: {} (Expected {})",
                        info.number, number
                    );
                }
            }
            Expect::Status(status) => {
                if *status != info.status {
                    panic!(
                        "Unexpected status: {:?} (Expected {:?})",
                        info.status, status
                    );
                }
            }
            Expect::Action(action) => {
                if *action != info.action {
                    panic!(
                        "Unexpected action: {:?} (Expected {:?})",
                        info.action, action
                    );
                }
            }
        }
    }
}

#[cfg(test)]
pub struct Validation<'a> {
    record: &'a mut Vec<PrintedTaskInfo>,
}

#[cfg(test)]
impl<'a> Validation<'a> {
    pub fn printed_task(self, es: &[Expect<'a>]) -> Validation<'a> {
        if self.record.len() == 0 {
            panic!("Missing task: {:#?}", es);
        }
        let info = self.record.drain(0..1).nth(0).unwrap();
        es.iter().for_each(|e| e.validate(&info));
        self
    }

    pub fn end(self) {
        if self.record.len() > 0 {
            panic!("Extra tasks were recorded: {:#?}", self.record);
        }
    }
}

#[cfg(test)]
impl FakePrinter {
    pub fn new() -> Self {
        Self { record: Vec::new() }
    }

    pub fn validate<'a>(&'a mut self) -> Validation<'a> {
        Validation {
            record: &mut self.record,
        }
    }
}

#[cfg(test)]
impl TodoPrinter for FakePrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        self.record.push(PrintedTaskInfo {
            desc: task.desc.to_string(),
            number: task.number,
            status: task.status,
            action: task.action,
        });
    }

    fn print_warning(&mut self, _warning: &PrintableWarning) {
        unimplemented!()
    }

    fn print_error(&mut self, _error: &PrintableError) {
        unimplemented!()
    }
}
