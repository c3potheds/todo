use clap::Parser;
use todo_cli::Options;
use todo_clock::Clock;
use todo_model::TodoList;
use todo_printing::Printable;
use todo_printing::TodoPrinter;
use todo_runner::Application;
use todo_runner::Mutated;
use todo_runner::TodoError;
use todo_runner::TodoResult;
use todo_text_editing::TextEditor;

struct App {
    options: Options,
}

impl Application for App {
    fn run<'a, P>(
        self,
        list: &'a mut TodoList,
        text_editor: &impl TextEditor,
        clock: &impl Clock,
        printer_factory: impl FnOnce(usize) -> Result<P, TodoError>,
    ) -> Mutated
    where
        P: TodoPrinter<'a>,
    {
        let app_result = todo_app::todo(list, text_editor, clock, self.options);
        let mut printer =
            printer_factory(app_result.max_index_digits()).unwrap();
        match app_result.print(&mut printer) {
            true => Mutated::Yes,
            false => Mutated::No,
        }
    }
}

fn main() -> TodoResult {
    todo_runner::run(App {
        options: Options::parse(),
    })
}
