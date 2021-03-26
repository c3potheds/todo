use chrono::TimeZone;
use chrono::Utc;
use cli::Options;
use clock::FakeClock;
use clock::SystemClock;
use model::TodoList;
use printing::FakePrinter;
use std::ffi::OsString;
use structopt::StructOpt;
use text_editing::FakeTextEditor;

pub fn test<I>(list: &mut TodoList, args: I) -> FakePrinter
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let mut text_editor = FakeTextEditor::no_user_output();
    test_with_text_editor(list, &mut text_editor, args)
}

pub fn test_with_text_editor<I>(
    list: &mut TodoList,
    text_editor: &FakeTextEditor,
    args: I,
) -> FakePrinter
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let mut printer = FakePrinter::new();
    let options = Options::from_iter_safe(args).expect("Could not parse args");
    crate::app::todo(list, &mut printer, text_editor, &SystemClock, &options);
    printer
}

pub struct Fixture<'a> {
    pub list: TodoList,
    pub clock: FakeClock,
    pub text_editor: FakeTextEditor<'a>,
}

impl<'a> Fixture<'a> {
    pub fn new() -> Self {
        Self {
            list: TodoList::new(),
            clock: FakeClock::new(Utc.ymd(2000, 01, 01).and_hms(00, 00, 00)),
            text_editor: FakeTextEditor::no_user_output(),
        }
    }

    pub fn test(&mut self, s: &str) -> FakePrinter {
        let mut printer = FakePrinter::new();
        let options = Options::from_iter_safe(s.split(" "))
            .expect("Could not parse args");
        crate::app::todo(
            &mut self.list,
            &mut printer,
            &self.text_editor,
            &self.clock,
            &options,
        );
        printer
    }
}
