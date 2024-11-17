use std::fmt::Display;
use std::fmt::Formatter;

use yansi::Paint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrintableInfo {
    Removed { desc: String },
}

impl Display for PrintableInfo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use self::PrintableInfo::*;
        write!(f, "{}: ", "info".white().bold().dim())?;
        match self {
            Removed { desc } => write!(f, "Removed \"{}\"", desc),
        }
    }
}
