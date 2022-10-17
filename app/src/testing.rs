#![allow(clippy::zero_prefixed_literal)]

use {
    chrono::{TimeZone, Utc},
    clap::Parser,
    cli::Options,
    clock::FakeClock,
    model::TodoList,
    pretty_assertions::assert_eq,
    printing::{
        PrintableError, PrintableInfo, PrintableTask, PrintableWarning,
        TodoPrinter,
    },
    text_editing::FakeTextEditor,
};

#[derive(Debug, PartialEq, Eq)]
enum PrintedItem<'list> {
    Task(PrintableTask<'list>),
    Info(PrintableInfo),
    Warning(PrintableWarning),
    Error(PrintableError),
}

pub struct Validation<'validation, 'test> {
    cmd: &'validation str,
    actual: &'validation Vec<PrintedItem<'test>>,
    expected: Vec<PrintedItem<'test>>,
}

impl<'validation, 'test> Validation<'validation, 'test> {
    pub fn printed_task(mut self, task: &PrintableTask<'test>) -> Self {
        self.expected.push(PrintedItem::Task(task.clone()));
        self
    }

    pub fn printed_info(mut self, info: &PrintableInfo) -> Self {
        self.expected.push(PrintedItem::Info(info.clone()));
        self
    }

    pub fn printed_warning(mut self, expected: &PrintableWarning) -> Self {
        self.expected.push(PrintedItem::Warning(expected.clone()));
        self
    }

    pub fn printed_error(mut self, expected: &PrintableError) -> Self {
        self.expected.push(PrintedItem::Error(expected.clone()));
        self
    }

    pub fn end(self) {
        let cmd = self.cmd;
        assert_eq!(
            // Note: the pretty_assertions crate switches the order of the
            // arguments to assert_eq!() so that the first argument is labeled
            // "right" and the second argument is labeled "left".
            //
            // The "left" argument is painted red, so we want that to be the
            // erroneous actual value when the assertion fails. The "right"
            // argument is painted green, showing what the output should be.
            &self.expected,
            self.actual,
            "Unexpected output from '{cmd}' (left: actual, right: expected)"
        );
    }
}

#[derive(Default)]
struct FakePrinter<'a> {
    record: Vec<PrintedItem<'a>>,
}

impl<'a, 'b> TodoPrinter<'b> for FakePrinter<'a>
where
    'b: 'a,
{
    fn print_task(&mut self, task: &PrintableTask<'b>) {
        self.record.push(PrintedItem::Task(task.clone()));
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

pub struct Fixture<'list> {
    pub list: TodoList<'list>,
    pub clock: FakeClock,
    pub text_editor: FakeTextEditor<'list>,
}

impl<'list> Default for Fixture<'list> {
    fn default() -> Self {
        Fixture {
            list: TodoList::default(),
            clock: FakeClock::new(Utc.ymd(2000, 01, 01).and_hms(00, 00, 00)),
            text_editor: FakeTextEditor::no_user_output(),
        }
    }
}

pub struct Validator<'test> {
    record: Vec<PrintedItem<'test>>,
    mutated: bool,
    cmd: String,
}

impl<'test> Validator<'test> {
    pub fn modified(self, expected: bool) -> Self {
        assert_eq!(
            self.mutated, expected,
            "Incorrect mutation from '{}'; expected {}, got {}",
            self.cmd, expected, self.mutated
        );
        self
    }

    pub fn validate(&mut self) -> Validation<'_, 'test> {
        Validation {
            cmd: &self.cmd,
            actual: &mut self.record,
            expected: Vec::new(),
        }
    }
}

impl<'list> Fixture<'list> {
    pub fn test(&mut self, s: &str) -> Validator<'_> {
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
