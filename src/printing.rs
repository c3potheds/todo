use ansi_term::Color;
use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Utc;
use cli::Key;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Write;

pub struct PrintingContext {
    /// The number of digits that task numbers may have, including a minus sign.
    pub max_index_digits: usize,
    /// The number of columns to render task descriptions in.
    pub width: usize,
    /// The current time.
    pub now: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Complete,
    Incomplete,
    Blocked,
    Removed,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action {
    None,
    New,
    Delete,
    Check,
    Uncheck,
    Lock,
    Unlock,
    Select,
    Punt,
    Snooze,
    Unsnooze,
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
    desc: &'a str,
    number: i32,
    status: Status,
    action: Action,
    log_date: Option<LogDate>,
    priority: i32,
    due_date: Option<DateTime<Utc>>,
    budget: Option<Duration>,
    start_date: Option<DateTime<Utc>>,
}

impl<'a> PrintableTask<'a> {
    pub fn new(desc: &'a str, number: i32, status: Status) -> Self {
        Self {
            desc: desc,
            number: number,
            status: status,
            action: Action::None,
            log_date: None,
            priority: 0,
            due_date: None,
            budget: None,
            start_date: None,
        }
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    pub fn log_date(mut self, log_date: LogDate) -> Self {
        self.log_date = Some(log_date);
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn budget(mut self, budget: Duration) -> Self {
        self.budget = Some(budget);
        self
    }

    pub fn start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }
}

struct PrintableTaskWithContext<'a> {
    context: &'a PrintingContext,
    task: &'a PrintableTask<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BriefPrintableTask {
    number: i32,
    status: Status,
}

impl BriefPrintableTask {
    pub fn new(number: i32, status: Status) -> Self {
        BriefPrintableTask {
            number: number,
            status: status,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrintableWarning {
    NoMatchFoundForKey {
        requested_key: Key,
    },
    CannotCheckBecauseAlreadyComplete {
        cannot_check: BriefPrintableTask,
    },
    CannotRestoreBecauseAlreadyIncomplete {
        cannot_restore: BriefPrintableTask,
    },
    CannotUnblockBecauseTaskIsNotBlocked {
        cannot_unblock: BriefPrintableTask,
        requested_unblock_from: BriefPrintableTask,
    },
    CannotPuntBecauseComplete {
        cannot_punt: BriefPrintableTask,
    },
    CannotSnoozeBecauseComplete {
        cannot_snooze: BriefPrintableTask,
    },
    AmbiguousKey {
        key: Key,
        matches: Vec<BriefPrintableTask>,
    },
    NoPathFoundBetween(BriefPrintableTask, BriefPrintableTask),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrintableError {
    CannotCheckBecauseBlocked {
        cannot_check: BriefPrintableTask,
        blocked_by: Vec<BriefPrintableTask>,
    },
    CannotRestoreBecauseAntidependencyIsComplete {
        cannot_restore: BriefPrintableTask,
        complete_antidependencies: Vec<BriefPrintableTask>,
    },
    CannotBlockBecauseWouldCauseCycle {
        cannot_block: BriefPrintableTask,
        requested_dependency: BriefPrintableTask,
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
    NoMatchForKeys {
        keys: Vec<Key>,
    },
    CannotParseDueDate {
        cannot_parse: String,
    },
    CannotParseDuration {
        cannot_parse: String,
    },
    DurationIsTooLong {
        duration: u64,
        string_repr: String,
    },
    ConflictingArgs((String, String)),
    CannotMerge {
        cycle_through: Vec<BriefPrintableTask>,
        adeps_of: Vec<BriefPrintableTask>,
        deps_of: Vec<BriefPrintableTask>,
    },
}

const ANSI_OFFSET: usize = 10;
const SELECTOR_OFFSET: usize = 6;
const LOG_DATE_OFFSET: usize = 11;

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Action::None => write!(f, "{}", "   "),
            Action::New => write!(f, "{}", Color::Green.paint("NEW")),
            Action::Delete => write!(f, "{}", Color::Red.paint("DEL")),
            Action::Check => write!(f, "{}", Color::Green.paint("[✓]")),
            Action::Uncheck => write!(f, "{}", Color::Yellow.paint("[ ]")),
            Action::Lock => write!(f, " {}", Color::Red.paint("🔒")),
            Action::Unlock => write!(f, " {}", Color::Green.paint("🔓")),
            Action::Select => write!(f, " * "),
            Action::Punt => write!(f, " ⏎ "),
            Action::Snooze => write!(f, "{}", Color::Blue.paint("ZZZ")),
            Action::Unsnooze => write!(f, " {}", Color::Purple.paint("⏰")),
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

fn format_number(number: i32, status: Status) -> String {
    let style = match &status {
        Status::Complete => Color::Green.normal(),
        Status::Incomplete => Color::Yellow.normal(),
        Status::Blocked => Color::Red.normal(),
        Status::Removed => Color::White.normal(),
    };
    let mut indexing = number.to_string();
    indexing.push_str(")");
    format!("{}", style.paint(&indexing))
}

fn format_numbers<'a, I: IntoIterator<Item = &'a BriefPrintableTask>>(
    numbers: I,
) -> String {
    numbers
        .into_iter()
        .map(|t| format_number(t.number, t.status))
        .collect::<Vec<_>>()
        .join(", ")
}

enum Urgency {
    Meh,
    Moderate,
    Urgent,
}

fn calculate_urgency(now: DateTime<Utc>, then: DateTime<Utc>) -> Urgency {
    if then - now < Duration::zero() {
        Urgency::Urgent
    } else if then - now < Duration::days(1) {
        Urgency::Moderate
    } else {
        Urgency::Meh
    }
}

fn calculate_progress(
    now: DateTime<Utc>,
    due: DateTime<Utc>,
    budget: Duration,
) -> i32 {
    let start = due - budget;
    let elapsed = now - start;
    let budget_spent: f64 =
        elapsed.num_seconds() as f64 / budget.num_seconds() as f64;
    let percentage = (budget_spent * 100.0) as i32;
    percentage
}

#[cfg(test)]
#[test]
fn calculate_progress_test() {
    use app::testing::ymdhms;
    assert_eq!(
        0,
        calculate_progress(
            ymdhms(2021, 04, 30, 10, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        50,
        calculate_progress(
            ymdhms(2021, 04, 30, 11, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        100,
        calculate_progress(
            ymdhms(2021, 04, 30, 12, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        -100,
        calculate_progress(
            ymdhms(2021, 04, 30, 08, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        200,
        calculate_progress(
            ymdhms(2021, 04, 30, 14, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
}

impl<'a> Display for PrintableTaskWithContext<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut start = if let Some(log_date) = &self.task.log_date {
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
        if let Some(start_date) = self.task.start_date {
            let snooze_duration = start_date - self.context.now;
            if snooze_duration > chrono::Duration::zero() {
                start.push_str(
                    &Color::Purple
                        .bold()
                        .paint(format!(
                            "Snoozed for {}",
                            ::time_format::format_duration_laconic(
                                snooze_duration
                            )
                        ))
                        .to_string(),
                );
                start.push_str(" ");
            }
        }
        if self.task.priority != 0 {
            let color = match self.task.priority.abs() {
                6..=i32::MAX => Color::Red,
                5 => Color::Yellow,
                4 => Color::Green,
                3 => Color::Cyan,
                2 => Color::Blue,
                1 => Color::Purple,
                _ => Color::Black,
            };
            let style = if self.task.priority >= 0 {
                color.bold()
            } else {
                color.bold().dimmed()
            };
            start.push_str(
                &style.paint(format!("P{}", self.task.priority)).to_string(),
            );
            start.push_str(" ");
        }
        if let Some(due_date) = self.task.due_date {
            let style = match calculate_urgency(self.context.now, due_date) {
                Urgency::Urgent => Color::Red.bold(),
                Urgency::Moderate => Color::Yellow.bold(),
                Urgency::Meh => Color::White.bold().dimmed(),
            };
            let desc = ::time_format::display_relative_time(
                self.context.now.with_timezone(&Local),
                due_date.with_timezone(&Local),
            );
            start.push_str(&style.paint(format!("Due {}", desc)).to_string());
            start.push_str(" ");
            if let Some(budget) = self.task.budget {
                let target_progress =
                    calculate_progress(self.context.now, due_date, budget);
                if target_progress >= 0 && target_progress <= 100 {
                    start.push_str(
                        &Color::White
                            .bold()
                            .paint("Target progress")
                            .to_string(),
                    );
                    start.push_str(" ");
                    let style = if target_progress < 50 {
                        Color::White.bold().dimmed()
                    } else if target_progress < 80 {
                        Color::Yellow.bold()
                    } else {
                        Color::Red.bold()
                    };
                    start.push_str(
                        &style
                            .paint(format!("{}%", target_progress))
                            .to_string(),
                    );
                    start.push_str(" ");
                }
            }
        }
        write!(
            f,
            "{}",
            textwrap::fill(
                self.task.desc,
                textwrap::Options::new(self.context.width)
                    .initial_indent(&start)
                    .break_words(false)
                    .subsequent_indent(&" ".repeat(
                        self.context.max_index_digits
                            + SELECTOR_OFFSET
                            + if self.task.log_date.is_some() {
                                LOG_DATE_OFFSET
                            } else {
                                0
                            }
                    )),
            )
        )
    }
}

impl Display for BriefPrintableTask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", format_number(self.number, self.status))
    }
}

impl Display for PrintableWarning {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            format!("{}", Color::Yellow.bold().paint("warning")),
            match self {
                PrintableWarning::NoMatchFoundForKey { requested_key } =>
                    format!("No match found for {}", format_key(requested_key)),
                PrintableWarning::CannotCheckBecauseAlreadyComplete {
                    cannot_check,
                } => format!("Task {} is already complete", cannot_check),
                PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                    cannot_restore,
                } => format!("Task {} is already incomplete", cannot_restore),
                PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                    cannot_unblock,
                    requested_unblock_from,
                } => format!(
                    "Task {} is not blocked by {}",
                    cannot_unblock, requested_unblock_from
                ),
                PrintableWarning::CannotPuntBecauseComplete { cannot_punt } =>
                    format!("Cannot punt complete task {}", cannot_punt),
                PrintableWarning::CannotSnoozeBecauseComplete {
                    cannot_snooze,
                } => format!("Cannot snooze complete task {}", cannot_snooze),
                PrintableWarning::AmbiguousKey { key, matches } => {
                    format!(
                        "Ambiguous key {} matches multiple tasks: {}",
                        format_key(key),
                        format_numbers(matches.iter())
                    )
                }
                PrintableWarning::NoPathFoundBetween(a, b) => {
                    format!("No path found between {} and {}", a, b)
                }
            }
        )
    }
}

impl Display for PrintableError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            format!("{}", Color::Red.bold().paint("error")),
            match self {
                PrintableError::CannotCheckBecauseBlocked {
                    cannot_check,
                    blocked_by,
                } => format!(
                    "Cannot complete {} because it is blocked by {}",
                    cannot_check,
                    format_numbers(blocked_by.into_iter()),
                ),
                PrintableError::CannotRestoreBecauseAntidependencyIsComplete{
                    cannot_restore,
                    complete_antidependencies,
                } => format!(
                    "Cannot restore {} because it blocks complete tasks {}",
                    cannot_restore,
                    format_numbers(complete_antidependencies.into_iter())
                ),
                PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block,
                    requested_dependency,
                } => format!(
                    "Cannot block {} on {} because it would create a cycle",
                    cannot_block,
                    requested_dependency,
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
                PrintableError::NoMatchForKeys{ keys } => {
                    format!(
                        "No match for keys {}",
                        keys.iter()
                            .map(|key| format_key(key))
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                }
                PrintableError::CannotParseDueDate { cannot_parse } => {
                    format!(
                        "Cannot parse due date: {}",
                        Color::White.bold().paint(cannot_parse),
                    )
                }
                PrintableError::CannotParseDuration { cannot_parse } => {
                    format!(
                        "Cannot parse duration: {}",
                        Color::White.bold().paint(cannot_parse),
                    )
                }
                PrintableError::DurationIsTooLong { duration, string_repr } => {
                    format!(
                        "Time budget is too long: {} (from {}).\n{}: {}",
                        Color::White.bold().paint(format!("{} secs", duration)),
                        Color::White.bold().paint(string_repr),
                        Color::White.bold().dimmed().paint("note"),
                        "Must be less than ~136 years, or 2^32 seconds."
                    )
                }
                PrintableError::ConflictingArgs((a, b)) => {
                    format!(
                        "Cannot pass {} and {} at the same time",
                        Color::White.bold().paint(a),
                        Color::White.bold().paint(b),
                    )
                }
                PrintableError::CannotMerge {
                    cycle_through,
                    adeps_of,
                    deps_of
                } => {
                    format!(
                        "Cannot merge: tasks {} are adeps of {} but deps of {}",
                        format_numbers(cycle_through.iter()),
                        format_numbers(adeps_of.iter()),
                        format_numbers(deps_of.iter())
                    )
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

pub struct SimpleTodoPrinter<Out: Write> {
    pub out: Out,
    pub context: PrintingContext,
}

impl<Out: Write> TodoPrinter for SimpleTodoPrinter<Out> {
    fn print_task(&mut self, task: &PrintableTask) {
        writeln!(
            self.out,
            "{}",
            PrintableTaskWithContext {
                context: &self.context,
                task: task,
            }
        )
        .unwrap();
    }
    fn print_warning(&mut self, warning: &PrintableWarning) {
        writeln!(self.out, "{}", warning).unwrap();
    }
    fn print_error(&mut self, error: &PrintableError) {
        writeln!(self.out, "{}", error).unwrap();
    }
}

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

#[derive(Debug)]
#[cfg(test)]
struct PrintedTaskInfo {
    desc: String,
    number: i32,
    status: Status,
    action: Action,
    log_date: Option<LogDate>,
    priority: i32,
    due_date: Option<DateTime<Utc>>,
    start_date: Option<DateTime<Utc>>,
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
enum Expect<'a> {
    Desc(&'a str),
    Number(i32),
    Status(Status),
    Action(Action),
    LogDate(LogDate),
    Priority(i32),
    DueDate(DateTime<Utc>),
    StartDate(DateTime<Utc>),
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
            Expect::Priority(expected) => {
                let actual = info.priority;
                if actual != *expected {
                    panic!(
                        "Unexpected priority: {:?} (Expected {:?})",
                        actual, expected
                    );
                }
            }
            Expect::DueDate(expected) => match &info.due_date {
                Some(actual) => {
                    if *actual != *expected {
                        panic!(
                            "Unexpected due date: {:?} (Expected {:?})",
                            // Display timestamps in local timezone to avoid
                            // confusion in tests, which use local time.
                            actual.with_timezone(&Local),
                            expected.with_timezone(&Local)
                        );
                    }
                }
                None => {
                    panic!("Missing required due date: {:?}", expected);
                }
            },
            Expect::StartDate(expected) => match &info.start_date {
                Some(actual) => {
                    if *actual != *expected {
                        panic!(
                            "Unexpected start date: {:?} (Expected {:?})",
                            actual.with_timezone(&Local),
                            expected.with_timezone(&Local)
                        );
                    }
                }
                None => {
                    panic!("Missing required start date: {:?}", expected);
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

    pub fn printed_task(self, task: &PrintableTask<'a>) -> Validation<'a> {
        let mut expectations = vec![
            Expect::Desc(task.desc),
            Expect::Number(task.number),
            Expect::Status(task.status),
            Expect::Action(task.action),
        ];
        if let Some(log_date) = &task.log_date {
            expectations.push(Expect::LogDate(log_date.clone()));
        }
        expectations.push(Expect::Priority(task.priority));
        if let Some(due_date) = task.due_date {
            expectations.push(Expect::DueDate(due_date));
        }
        if let Some(start_date) = task.start_date {
            expectations.push(Expect::StartDate(start_date));
        }
        self.printed_task_impl(&expectations)
    }

    fn printed_task_impl(mut self, es: &[Expect<'a>]) -> Validation<'a> {
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
            priority: task.priority,
            due_date: task.due_date,
            start_date: task.start_date,
        }));
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        self.record.push(PrintedItem::Warning(warning.clone()));
    }

    fn print_error(&mut self, error: &PrintableError) {
        self.record.push(PrintedItem::Error(error.clone()));
    }
}
