#![allow(clippy::zero_prefixed_literal)]
use std::cell::RefCell;
use std::rc::Rc;
use todo_app::{Application, Mutated};

use {
    chrono::{TimeZone, Utc},
    clap::Parser,
    pretty_assertions::assert_eq,
    todo_cli::Options,
    todo_clock::FakeClock,
    todo_model::TodoList,
    todo_printing::{
        PrintableError, PrintableInfo, PrintableTask, PrintableWarning, Status,
        TodoPrinter,
    },
    todo_text_editing::FakeTextEditor,
};

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Default, Clone)]
struct FakePrinter<'a> {
    record: Rc<RefCell<Vec<PrintedItem<'a>>>>,
}

impl<'a, 'b> TodoPrinter<'b> for FakePrinter<'a>
where
    'b: 'a,
{
    fn print_task(&mut self, task: &PrintableTask<'b>) {
        self.record
            .borrow_mut()
            .push(PrintedItem::Task(task.clone()));
    }

    fn print_info(&mut self, info: &PrintableInfo) {
        self.record
            .borrow_mut()
            .push(PrintedItem::Info(info.clone()));
    }

    fn print_warning(&mut self, warning: &PrintableWarning) {
        self.record
            .borrow_mut()
            .push(PrintedItem::Warning(warning.clone()));
    }

    fn print_error(&mut self, error: &PrintableError) {
        self.record
            .borrow_mut()
            .push(PrintedItem::Error(error.clone()));
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
            clock: FakeClock::new(
                Utc.with_ymd_and_hms(2000, 01, 01, 00, 00, 00).unwrap(),
            ),
            text_editor: FakeTextEditor::no_user_output(),
        }
    }
}

pub struct Validator<'test> {
    record: Vec<PrintedItem<'test>>,
    mutated: Mutated,
    cmd: String,
}

impl<'test> Validator<'test> {
    pub fn modified(self, expected: Mutated) -> Self {
        assert_eq!(
            self.mutated, expected,
            "Incorrect mutation from '{}'; expected {:?}, got {:?}",
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
        let printer = FakePrinter::default();
        let mutated = {
            let args = shlex::split(s).expect("Could not split args");
            let options =
                Options::try_parse_from(args).expect("Could not parse args");
            let app = crate::App::new(options);
            app.run(&mut self.list, &self.text_editor, &self.clock, |_| {
                printer.clone()
            })
        };
        let record = printer.record.borrow().clone();
        Validator {
            record,
            mutated,
            cmd: s.to_string(),
        }
    }
}

pub fn task(desc: &str, pos: i32, status: Status) -> PrintableTask<'_> {
    PrintableTask::new(desc, pos, status)
}
