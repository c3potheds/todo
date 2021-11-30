use super::format_util::format_number;
use super::Status;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct BriefPrintableTask {
    pub number: i32,
    pub status: Status,
}

impl BriefPrintableTask {
    pub fn new(number: i32, status: Status) -> Self {
        BriefPrintableTask { number, status }
    }
}

impl Display for BriefPrintableTask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", format_number(self.number, self.status))
    }
}
