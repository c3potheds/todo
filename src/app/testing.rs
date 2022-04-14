#![allow(clippy::zero_prefixed_literal)]

use crate::{
    cli::Options, clock::FakeClock, model::TodoList, printing::FakePrinter,
    text_editing::FakeTextEditor,
};
use chrono::{DateTime, Local, TimeZone, Utc};
use structopt::StructOpt;

pub fn ymdhms(
    yr: i32,
    mon: u32,
    day: u32,
    hr: u32,
    min: u32,
    sec: u32,
) -> DateTime<Utc> {
    Local
        .ymd(yr, mon, day)
        .and_hms(hr, min, sec)
        .with_timezone(&Utc)
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

impl<'a> Fixture<'a> {
    pub fn test(&mut self, s: &str) -> FakePrinter {
        let mut printer = FakePrinter::default();
        let options = Options::from_iter_safe(s.split(' '))
            .expect("Could not parse args");
        crate::app::todo(
            &mut self.list,
            &mut printer,
            &self.text_editor,
            &self.clock,
            options,
        );
        printer
    }
}
