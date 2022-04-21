use {
    crate::{format_util::format_number, Status},
    std::{
        fmt,
        fmt::{Display, Formatter},
    },
};

/// Represents a task in the to-do list without its description.
///
/// When formatted, the representation will use ANSI colors to display the
/// task number (its position in the list) with a color corresponding to its
/// status.
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
