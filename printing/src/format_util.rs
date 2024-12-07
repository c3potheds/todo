use std::fmt;

use todo_lookup_key::Key;
use yansi::Paint;
use yansi::Style;

use crate::BriefPrintableTask;
use crate::Status;

pub fn format_key(key: &Key) -> impl fmt::Display + '_ {
    struct FormatKey<'a>(&'a Key);

    impl fmt::Display for FormatKey<'_> {
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

    impl fmt::Display for FormatKeys<'_> {
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
        Status::Complete => Style::new().green(),
        Status::Incomplete => Style::new().yellow(),
        Status::Blocked => Style::new().red(),
    };
    let mut indexing = number.to_string();
    indexing.push(')');
    format!("{}", indexing.paint(style))
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
