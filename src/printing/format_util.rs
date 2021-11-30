use super::BriefPrintableTask;
use super::Status;
use ansi_term::Color;
use cli::Key;

pub fn format_key(key: &Key) -> String {
    match key {
        Key::ByNumber(n) => format!("\"{}\"", n),
        Key::ByName(ref name) => format!("\"{}\"", name),
        Key::ByRange(start, end) => format!("range({}..{})", start, end),
    }
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
