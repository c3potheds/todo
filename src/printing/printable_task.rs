use ansi_term::Color;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Action::*;
        use self::Color::*;
        match self {
            None => write!(f, "   "),
            New => write!(f, "{}", Green.paint("NEW")),
            Delete => write!(f, "{}", Red.paint("DEL")),
            Check => write!(f, "{}", Green.paint("[✓]")),
            Uncheck => write!(f, "{}", Yellow.paint("[ ]")),
            Lock => write!(f, " {}", Red.paint("🔒")),
            Unlock => write!(f, " {}", Green.paint("🔓")),
            Select => write!(f, " * "),
            Punt => write!(f, " ⏎ "),
            Snooze => write!(f, "{}", Blue.paint("ZZZ")),
            Unsnooze => write!(f, " {}", Purple.paint("⏰")),
        }
    }
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
        if !(1000..=9999).contains(&y) {
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
    pub status: Status,
    pub action: Action,
    pub log_date: Option<LogDate>,
    pub priority: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub budget: Option<Duration>,
    pub start_date: Option<DateTime<Utc>>,
    pub dependent_tasks: (usize, usize),
}

impl<'a> PrintableTask<'a> {
    pub fn new(desc: &'a str, number: i32, status: Status) -> Self {
        Self {
            desc,
            number,
            status,
            action: Action::None,
            log_date: None,
            priority: 0,
            due_date: None,
            budget: None,
            start_date: None,
            dependent_tasks: (0, 0),
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

    pub fn dependent_tasks(mut self, immediate: usize, total: usize) -> Self {
        self.dependent_tasks = (immediate, total);
        self
    }
}
