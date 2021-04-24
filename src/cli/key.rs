use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Key {
    ByNumber(i32),
    ByName(String),
    ByRange(i32, i32),
}

fn split_once<'a>(s: &'a str, pattern: &'a str) -> Option<(&'a str, &'a str)> {
    let mut iter = s.splitn(2, pattern);
    match iter.next() {
        Some(first) => match iter.next() {
            Some(rest) => Some((first, rest)),
            _ => None,
        },
        _ => None,
    }
}

impl FromStr for Key {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i32>() {
            return Ok(Key::ByNumber(n));
        }
        if let Some((prefix, suffix)) = split_once(s, "..") {
            if let (Ok(start), Ok(end)) =
                (prefix.parse::<i32>(), suffix.parse::<i32>())
            {
                return Ok(Key::ByRange(start, end));
            }
        }
        Ok(Key::ByName(s.to_string()))
    }
}
