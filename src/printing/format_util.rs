use std::fmt;

use super::BriefPrintableTask;
use super::Status;
use ansi_term::Color;
use cli::Key;

pub fn format_key(key: &Key) -> impl fmt::Display + '_ {
    struct FormatKey<'a>(&'a Key);

    impl<'a> fmt::Display for FormatKey<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                Key::ByNumber(n) => write!(f, "\"{}\"", n),
                Key::ByName(ref name) => write!(f, "\"{}\"", name),
                Key::ByRange(start, end) => {
                    write!(f, "range({}..{})", start, end)
                }
            }
        }
    }

    FormatKey(key)
}

pub fn format_keys(keys: &[Key]) -> impl fmt::Display + '_ {
    struct FormatKeys<'a>(&'a [Key]);

    impl<'a> fmt::Display for FormatKeys<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut first = true;
            for key in self.0 {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{}", format_key(key))?;
            }
            Ok(())
        }
    }

    FormatKeys(keys)
}

pub fn format_number(number: i32, status: Status) -> String {
    let style = match &status {
        Status::Complete => Color::Green.normal(),
        Status::Incomplete => Color::Yellow.normal(),
        Status::Blocked => Color::Red.normal(),
        Status::Removed => Color::White.normal(),
    };
    let mut indexing = number.to_string();
    indexing.push(')');
    format!("{}", style.paint(&indexing))
}

pub fn format_numbers<'a, I: IntoIterator<Item = &'a BriefPrintableTask>>(
    numbers: I,
) -> String {
    numbers
        .into_iter()
        .map(|t| format_number(t.number, t.status))
        .collect::<Vec<_>>()
        .join(", ")
}
