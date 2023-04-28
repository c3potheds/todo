use {
    ansi_term::Color,
    chrono::{DateTime, Duration, Utc},
    std::{
        fmt,
        fmt::{Display, Formatter},
    },
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    #[default]
    Incomplete,
    Complete,
    Blocked,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    #[default]
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

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Plicit<T> {
    Implicit(T),
    Explicit(T),
}

impl<T> Plicit<T> {
    pub fn map<R, F: FnOnce(T) -> R>(self, f: F) -> Plicit<R> {
        match self {
            Plicit::Implicit(t) => Plicit::Implicit(f(t)),
            Plicit::Explicit(t) => Plicit::Explicit(f(t)),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct PrintableTask<'a> {
    pub desc: &'a str,
    pub number: i32,
    pub status: Status,
    pub action: Action,
    pub log_date: Option<LogDate>,
    pub priority: Option<Plicit<i32>>,
    pub due_date: Option<Plicit<DateTime<Utc>>>,
    pub punctuality: Option<Duration>,
    pub budget: Option<Duration>,
    pub start_date: Option<DateTime<Utc>>,
    pub deps_stats: (usize, usize),
    pub adeps_stats: (usize, usize),
    pub is_explicit_tag: bool,
    pub implicit_tags: Vec<&'a str>,
}

impl<'a> PrintableTask<'a> {
    pub fn new(desc: &'a str, number: i32, status: Status) -> Self {
        Self {
            desc,
            number,
            status,
            ..Default::default()
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

    pub fn priority(mut self, priority: Plicit<i32>) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn due_date(mut self, due_date: Plicit<DateTime<Utc>>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn punctuality(mut self, punctuality: Duration) -> Self {
        self.punctuality = Some(punctuality);
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

    pub fn deps_stats(mut self, immediate: usize, total: usize) -> Self {
        self.deps_stats = (immediate, total);
        self
    }

    pub fn adeps_stats(mut self, immediate: usize, total: usize) -> Self {
        self.adeps_stats = (immediate, total);
        self
    }

    pub fn as_tag(mut self) -> Self {
        self.is_explicit_tag = true;
        self
    }

    pub fn tag(mut self, tag: &'a str) -> Self {
        self.implicit_tags.push(tag);
        self
    }
}
