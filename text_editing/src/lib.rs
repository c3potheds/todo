#[derive(Debug)]
pub enum Error {
    CouldNotOpenTextEditor(Box<dyn std::error::Error>),
    CouldNotReadResult(Box<dyn std::error::Error>),
    FakeErrorForTesting,
}

pub trait TextEditor {
    fn edit_text(&self, display: &str) -> Result<String, Error>;
}

pub struct ScrawlTextEditor<'a>(pub &'a str);

impl TextEditor for ScrawlTextEditor<'_> {
    fn edit_text(&self, display: &str) -> Result<String, Error> {
        scrawl::editor::new()
            .editor(self.0)
            .open(scrawl::editor::Contents::FromString(&display.as_bytes()))
            .map_err(Error::CouldNotOpenTextEditor)?
            .to_string()
            .map_err(Error::CouldNotReadResult)
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
        Ok(self
            .user_output
            .ok_or(Error::FakeErrorForTesting)?
            .to_string())
    }
}

#[cfg(test)]
mod test;
