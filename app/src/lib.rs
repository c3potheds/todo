use todo_clock::Clock;
use todo_model::TodoList;
use todo_text_editing::TextEditor;

pub trait Application {
    type Result<'a>: todo_printing::Printable<'a>;
    fn run<'a>(
        self,
        list: &'a mut TodoList,
        text_editor: &impl TextEditor,
        clock: &impl Clock,
    ) -> Self::Result<'a>;
}
