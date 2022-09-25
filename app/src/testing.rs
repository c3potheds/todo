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
}

impl Validator {
    pub fn modified(self, expected: bool) -> Self {
        assert_eq!(self.mutated, expected);
        self
    }

    pub fn validate(&mut self) -> Validation<'_> {
        self.printer.validate()
    }
}

impl<'a> Fixture<'a> {
    pub fn test(&mut self, s: &str) -> Validator {
        let mut printer = FakePrinter::default();
        let options = Options::try_parse_from(s.split(' '))
            .expect("Could not parse args");
        let mutated = crate::todo(
            &mut self.list,
            &mut printer,
            &self.text_editor,
            &self.clock,
            options,
        );
        Validator { printer, mutated }
    }
}
