use ansi_term::Color;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum PrintableInfo {
    Removed { desc: String },
}

impl Display for PrintableInfo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use self::PrintableInfo::*;
        write!(f, "{}: ", Color::White.bold().dimmed().paint("info"))?;
        match self {
            Removed { desc } => write!(f, "Removed \"{}\"", desc),
        }
    }
}
