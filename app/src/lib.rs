use {
    todo_clock::Clock, todo_model::TodoList, todo_printing::TodoPrinter,
    todo_text_editing::TextEditor,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Mutated {
    Yes,
    No,
}

pub trait Application {
    fn run<'a, P>(
        self,
        list: &'a mut TodoList,
        text_editor: &impl TextEditor,
        clock: &impl Clock,
        printer_factory: impl FnOnce(usize) -> P,
    ) -> Mutated
    where
        P: TodoPrinter<'a>;
}
