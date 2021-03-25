use cli::Options;
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
    crate::app::todo(list, &mut printer, text_editor, &options);
    printer
}
