#![allow(clippy::zero_prefixed_literal)]

use {
    chrono::{TimeZone, Utc},
    clap::Parser,
    cli::Options,
    clock::FakeClock,
    model::TodoList,
    printing::{FakePrinter, Validation},
    text_editing::FakeTextEditor,
};

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
    printer: FakePrinter,
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
        self.printer.validate()
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
            printer,
            mutated,
            cmd: s.to_string(),
        }
    }
}
