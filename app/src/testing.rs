#![allow(clippy::zero_prefixed_literal)]

use {
    chrono::{DateTime, Duration, TimeZone, Utc},
    clap::Parser,
    cli::Options,
    clock::FakeClock,
    model::TodoList,
    printing::{
        Action, LogDate, Plicit, PrintableError, PrintableInfo, PrintableTask,
        PrintableWarning, Status, TodoPrinter,
    },
    text_editing::FakeTextEditor,
    pretty_assertions::assert_eq,
};

#[derive(Debug, PartialEq, Eq)]
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
    is_explicit_tag: bool,
    implicit_tags: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum PrintedItem {
    Task(PrintedTaskInfo),
    Info(PrintableInfo),
    Warning(PrintableWarning),
    Error(PrintableError),
}

pub struct Validation<'a> {
    actual: &'a Vec<PrintedItem>,
    expected: Vec<PrintedItem>,
}

fn record_task_info(task: &PrintableTask) -> PrintedItem {
    PrintedItem::Task(PrintedTaskInfo {
        desc: task.desc.to_string(),
        number: task.number,
        status: task.status,
        action: task.action,
        log_date: task.log_date.clone(),
        priority: task.priority.clone(),
        due_date: task.due_date.clone(),
        start_date: task.start_date,
        budget: task.budget,
        is_explicit_tag: task.is_explicit_tag,
        implicit_tags: task
            .implicit_tags
            .iter()
            .map(|tag| tag.to_string())
            .collect(),
    })
}

impl<'a> Validation<'a> {
    pub fn printed_task(
        mut self,
        task: &'a PrintableTask<'a>,
    ) -> Validation<'a> {
        self.expected.push(record_task_info(task));
        self
    }

    pub fn printed_info(mut self, info: &'a PrintableInfo) -> Validation<'a> {
        self.expected.push(PrintedItem::Info(info.clone()));
        self
    }

    pub fn printed_warning(
        mut self,
        expected: &'a PrintableWarning,
    ) -> Validation<'a> {
        self.expected.push(PrintedItem::Warning(expected.clone()));
        self
    }

    pub fn printed_error(
        mut self,
        expected: &'a PrintableError,
    ) -> Validation<'a> {
        self.expected.push(PrintedItem::Error(expected.clone()));
        self
    }

    pub fn end(self) {
        assert_eq!(self.actual, &self.expected);
    }
}

#[derive(Default)]
struct FakePrinter {
    record: Vec<PrintedItem>,
}

impl TodoPrinter for FakePrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        self.record.push(record_task_info(task));
    }

    fn print_info(&mut self, info: &PrintableInfo) {
        self.record.push(PrintedItem::Info(info.clone()));
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        self.record.push(PrintedItem::Warning(warning.clone()));
    }

    fn print_error(&mut self, error: &PrintableError) {
        self.record.push(PrintedItem::Error(error.clone()));
    }
}

pub struct Fixture<'a> {
    pub list: TodoList<'a>,
    pub clock: FakeClock,
    pub text_editor: FakeTextEditor<'a>,
}

impl<'a> Default for Fixture<'a> {
    fn default() -> Self {
        Fixture {
            list: TodoList::default(),
            clock: FakeClock::new(Utc.ymd(2000, 01, 01).and_hms(00, 00, 00)),
            text_editor: FakeTextEditor::no_user_output(),
        }
    }
}

pub struct Validator {
    record: Vec<PrintedItem>,
    mutated: bool,
    cmd: String,
}

impl Validator {
    pub fn modified(self, expected: bool) -> Self {
        assert_eq!(
            self.mutated, expected,
            "Incorrect mutation from '{}'; expected {}, got {}",
            self.cmd, expected, self.mutated
        );
        self
    }

    pub fn validate(&mut self) -> Validation<'_> {
        Validation {
            actual: &mut self.record,
            expected: Vec::new(),
        }
    }
}

impl<'a> Fixture<'a> {
    pub fn test(&mut self, s: &str) -> Validator {
        let mut printer = FakePrinter::default();
        let args = shlex::split(s).expect("Could not split args");
        let options =
            Options::try_parse_from(args).expect("Could not parse args");
        use printing::Printable;
        let mutated = crate::todo(
            &mut self.list,
            &self.text_editor,
            &self.clock,
            options,
        )
        .print(&mut printer);
        Validator {
            record: printer.record,
            mutated,
            cmd: s.to_string(),
        }
    }
}
