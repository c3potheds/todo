use ansi_term::Color;
use cli::Key;
use model::TaskStatus;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Write;

pub struct PrintingContext {
    /// The number of digits that task numbers may have, including a minus sign.
    pub max_index_digits: usize,
    /// The number of columns to render task descriptions in.
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
    Punt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogDate {
    Invisible,
    YearMonthDay(u16, u8, u8),
}

#[derive(Debug)]
pub enum InvalidDate {
    YearOutOfRange(u16),
    MonthOutOfRange(u8),
    DayOutOfRange(u8),
}

impl LogDate {
    pub fn ymd(y: u16, m: u8, d: u8) -> Result<LogDate, InvalidDate> {
        if y < 1000 || y > 9999 {
            return Err(InvalidDate::YearOutOfRange(y));
        }
        if m == 0 || m > 12 {
            return Err(InvalidDate::MonthOutOfRange(m));
        }
        if d == 0 || d > 31 {
            return Err(InvalidDate::DayOutOfRange(d));
        }
        Ok(LogDate::YearMonthDay(y, m, d))
    }
}

impl Display for LogDate {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            LogDate::Invisible => write!(f, "          "),
            LogDate::YearMonthDay(ref y, ref m, ref d) => {
                write!(f, "{:04}-{:02}-{:02}", y, m, d)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PrintableTask<'a> {
    pub desc: &'a str,
    pub number: i32,
    pub status: TaskStatus,
    pub action: Action,
    pub log_date: Option<LogDate>,
}

struct PrintableTaskWithContext<'a> {
    context: &'a PrintingContext,
    task: &'a PrintableTask<'a>,
}

#[derive(Debug, Clone, PartialEq)]
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
    CannotPuntBecauseComplete {
        cannot_punt: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
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
        // TODO(printing.show-cycle): print the path between
        // requested_dependency and cannot_block.
        // cycles: Vec<Vec<i32>>,
    },
    CannotEditBecauseUnexpectedNumber {
        requested: i32,
    },
    CannotEditBecauseNoTaskWithNumber {
        requested: i32,
    },
    CannotEditBecauseInvalidLine {
        malformed_line: String,
    },
    FailedToUseTextEditor,
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
            Action::Punt => write!(f, " ⏎ "),
        }
    }
}

fn format_key(key: &Key) -> String {
    match key {
        &Key::ByNumber(n) => format!("\"{}\"", n),
        &Key::ByName(ref name) => format!("\"{}\"", name),
        &Key::ByRange(start, end) => format!("range({}..{})", start, end),
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

impl<'a> Display for PrintableTaskWithContext<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let start = if let Some(log_date) = &self.task.log_date {
            format!(
                "{} {} {:>width$} ",
                log_date,
                self.task.action,
                format_number(self.task.number, self.task.status),
                width = self.context.max_index_digits + ANSI_OFFSET
            )
        } else {
            format!(
                "{} {:>width$} ",
                self.task.action,
                format_number(self.task.number, self.task.status),
                width = self.context.max_index_digits + ANSI_OFFSET
            )
        };
        write!(
            f,
            "{}",
            textwrap::fill(
                self.task.desc,
                textwrap::Options::new(self.context.width)
                    .initial_indent(&start)
                    .break_words(false)
                    .subsequent_indent(
                        &" ".repeat(self.context.max_index_digits + 6),
                    ),
            )
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
                PrintableWarning::CannotPuntBecauseComplete { cannot_punt } =>
                    format!(
                        "Cannot punt complete task {}",
                        format_number(*cannot_punt, TaskStatus::Complete)
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
                PrintableError::CannotEditBecauseUnexpectedNumber {
                    requested,
                } => format!(
                    "Number {}) doesn't correspond to any of requested tasks",
                    requested,
                ),
                PrintableError::CannotEditBecauseNoTaskWithNumber {
                    requested,
                } => format!("No task with number {})", requested),
                PrintableError::CannotEditBecauseInvalidLine{
                    malformed_line,
                } => format!("Could not parse line: \"{}\"", malformed_line),
                PrintableError::FailedToUseTextEditor => {
                    format!("Failed to open text editor")
                }
            }
        )
    }
}

pub trait TodoPrinter {
    fn print_task(&mut self, task: &PrintableTask);
    fn print_warning(&mut self, warning: &PrintableWarning);
    fn print_error(&mut self, error: &PrintableError);
}

pub struct SimpleTodoPrinter<'a, Out: Write> {
    pub out: Out,
    pub context: &'a PrintingContext,
}

impl<Out: Write> TodoPrinter for SimpleTodoPrinter<'_, Out> {
    fn print_task(&mut self, task: &PrintableTask) {
        writeln!(
            self.out,
            "{}",
            PrintableTaskWithContext {
                context: self.context,
                task: task,
            }
        )
        .unwrap();
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
    log_date: Option<LogDate>,
}

#[derive(Debug)]
#[cfg(test)]
enum PrintedItem {
    Task(PrintedTaskInfo),
    Warning(PrintableWarning),
    Error(PrintableError),
}

#[cfg(test)]
pub struct FakePrinter {
    record: Vec<PrintedItem>,
}

#[derive(Debug)]
#[cfg(test)]
pub enum Expect<'a> {
    Desc(&'a str),
    Number(i32),
    Status(TaskStatus),
    Action(Action),
    LogDate(LogDate),
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
            Expect::LogDate(log_date) => match &info.log_date {
                Some(actual) => {
                    if *log_date != *actual {
                        panic!(
                            "Unexpected log date: {:?} (Expected {:?}",
                            actual, log_date
                        );
                    }
                }
                None => {
                    panic!("Missing required log date: {:?}", log_date);
                }
            },
        }
    }
}

#[cfg(test)]
pub struct Validation<'a> {
    record: &'a mut Vec<PrintedItem>,
}

#[cfg(test)]
impl<'a> Validation<'a> {
    fn pop(&mut self, expected: &impl std::fmt::Debug) -> PrintedItem {
        if self.record.len() == 0 {
            panic!("Missing item: {:#?}", expected);
        }
        self.record.drain(0..1).nth(0).unwrap()
    }

    pub fn printed_task(mut self, es: &[Expect<'a>]) -> Validation<'a> {
        let item = self.pop(&es);
        match &item {
            PrintedItem::Task(ref info) => {
                es.iter().for_each(|e| e.validate(info))
            }
            _ => {
                panic!("Expected\n{:#?}\n... but got\n{:#?}", es, item);
            }
        };
        self
    }

    pub fn printed_warning(
        mut self,
        expected: &'a PrintableWarning,
    ) -> Validation<'a> {
        let item = self.pop(expected);
        match &item {
            PrintedItem::Warning(ref actual) => {
                assert_eq!(actual, expected, "Unexpected warning")
            }
            _ => panic!("Expected\n{:#?}\n... but got\n{:#?}", expected, item),
        };
        self
    }

    pub fn printed_error(
        mut self,
        expected: &'a PrintableError,
    ) -> Validation<'a> {
        let item = self.pop(expected);
        match &item {
            PrintedItem::Error(ref actual) => {
                assert_eq!(actual, expected, "Unexpected error")
            }
            _ => panic!("Expected\n{:#?}\n... but got\n{:#?}", expected, item),
        };
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
        self.record.push(PrintedItem::Task(PrintedTaskInfo {
            desc: task.desc.to_string(),
            number: task.number,
            status: task.status,
            action: task.action,
            log_date: task.log_date.clone(),
        }));
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        self.record.push(PrintedItem::Warning(warning.clone()));
    }

    fn print_error(&mut self, error: &PrintableError) {
        self.record.push(PrintedItem::Error(error.clone()));
    }
}
