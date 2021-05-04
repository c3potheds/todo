extern crate scrawl;

#[derive(Debug)]
pub struct Error();

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self()
    }
}

pub trait TextEditor {
    fn edit_text(&self, display: &str) -> Result<String, Error>;
}

pub struct ScrawlTextEditor<'a>(pub &'a str);

impl<'a> TextEditor for ScrawlTextEditor<'a> {
    fn edit_text(&self, display: &str) -> Result<String, Error> {
        scrawl::editor::new()
            .editor(self.0)
            .contents(display)
            .open()
            .map_err(|_| Error())
    }
}

pub struct FakeTextEditor<'a> {
    user_output: Option<&'a str>,
    recorded_input: std::cell::RefCell<String>,
}

impl<'a> FakeTextEditor<'a> {
    pub fn user_will_enter(s: &'a str) -> Self {
        FakeTextEditor {
            user_output: Some(s),
            recorded_input: std::cell::RefCell::default(),
        }
    }

    pub fn no_user_output() -> Self {
        FakeTextEditor {
            user_output: None,
            recorded_input: std::cell::RefCell::default(),
        }
    }

    pub fn recorded_input(&self) -> std::cell::Ref<String> {
        self.recorded_input.borrow()
    }
}

impl TextEditor for FakeTextEditor<'_> {
    fn edit_text(&self, display: &str) -> Result<String, Error> {
        self.recorded_input.replace(display.to_string());
        match &self.user_output {
            Some(ref output) => Ok(output.to_string()),
            None => Err(Error()),
        }
    }
}
