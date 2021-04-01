use chrono::TimeZone;
use chrono::Utc;
use cli::Options;
use clock::FakeClock;
use model::TodoList;
use printing::FakePrinter;
use structopt::StructOpt;
use text_editing::FakeTextEditor;

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
            options,
        );
        printer
    }
}
