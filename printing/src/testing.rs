use {
    crate::{
        Action, LogDate, Plicit, PrintableError, PrintableTask,
        PrintableWarning, Status, TodoPrinter,
    },
    chrono::{DateTime, Duration, Local, Utc},
};

#[derive(Debug)]
struct PrintedTaskInfo {
    desc: String,
    number: i32,
    status: Status,
    action: Action,
    log_date: Option<LogDate>,
    priority: Option<Plicit<i32>>,
    due_date: Option<Plicit<DateTime<Utc>>>,
    start_date: Option<DateTime<Utc>>,
    budget: Option<Duration>,
}

#[derive(Debug)]
enum PrintedItem {
    Task(PrintedTaskInfo),
    Warning(PrintableWarning),
    Error(PrintableError),
}

#[derive(Default)]
pub struct FakePrinter {
    record: Vec<PrintedItem>,
}

#[derive(Debug)]
enum Expect<'a> {
    Desc(&'a str),
    Number(i32),
    Status(Status),
    Action(Action),
    LogDate(LogDate),
    Priority(Option<Plicit<i32>>),
    DueDate(Plicit<DateTime<Utc>>),
    StartDate(DateTime<Utc>),
    Budget(Duration),
}

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
                let actual = &info.priority;
                if actual != expected {
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
                            actual.clone().map(|d| d.with_timezone(&Local)),
                            expected.clone().map(|d| d.with_timezone(&Local))
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
            Expect::Budget(expected) => {
                let actual = info.budget;
                if actual != Some(*expected) {
                    panic!(
                        "Unexpected budget: {:?} (Expected {:?})",
                        actual, expected
                    );
                }
            }
        }
    }
}

pub struct Validation<'a> {
    record: &'a mut Vec<PrintedItem>,
}

impl<'a> Validation<'a> {
    fn pop(&mut self, expected: &impl std::fmt::Debug) -> PrintedItem {
        if self.record.is_empty() {
            panic!("Missing item: {:#?}", expected);
        }
        self.record.drain(0..1).next().unwrap()
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
        expectations.push(Expect::Priority(task.priority.clone()));
        if let Some(due_date) = task.due_date.clone() {
            expectations.push(Expect::DueDate(due_date));
        }
        if let Some(start_date) = task.start_date {
            expectations.push(Expect::StartDate(start_date));
        }
        if let Some(budget) = task.budget {
            expectations.push(Expect::Budget(budget));
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
        if !self.record.is_empty() {
            panic!("Extra tasks were recorded: {:#?}", self.record);
        }
    }
}

impl FakePrinter {
    pub fn validate(&mut self) -> Validation<'_> {
        Validation {
            record: &mut self.record,
        }
    }
}

impl TodoPrinter for FakePrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        self.record.push(PrintedItem::Task(PrintedTaskInfo {
            desc: task.desc.to_string(),
            number: task.number,
            status: task.status,
            action: task.action,
            log_date: task.log_date.clone(),
            priority: task.priority.clone(),
            due_date: task.due_date.clone(),
            start_date: task.start_date,
            budget: task.budget,
        }));
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        self.record.push(PrintedItem::Warning(warning.clone()));
    }

    fn print_error(&mut self, error: &PrintableError) {
        self.record.push(PrintedItem::Error(error.clone()));
    }
}
