use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Key {
    ByNumber(i32),
    ByName(String),
    ByRange(i32, i32),
}

impl FromStr for Key {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i32>() {
            return Ok(Key::ByNumber(n));
        }
        if let Some((prefix, suffix)) = s.split_once("..") {
            let prefix = prefix.strip_prefix("[").unwrap_or(prefix);
            let suffix = suffix.strip_suffix("]").unwrap_or(suffix);
            if let (Ok(start), Ok(end)) =
                (prefix.parse::<i32>(), suffix.parse::<i32>())
            {
                use std::cmp::{max, min};
                return Ok(Key::ByRange(min(start, end), max(start, end)));
            }
        }
        Ok(Key::ByName(s.to_string()))
    }
}
