use crate::Key;
use crate::Key::*;
use std::str::FromStr;

#[test]
fn parse_key_from_number() {
    assert_eq!(Ok(ByNumber(1)), Key::from_str("1"));
}

#[test]
fn parse_key_from_range() {
    assert_eq!(Ok(ByRange(1, 2)), Key::from_str("1..2"));
}

#[test]
fn parse_key_from_name() {
    assert_eq!(Ok(ByName("foo".to_string())), Key::from_str("foo"));
}

#[test]
fn parse_key_from_name_with_spaces() {
    assert_eq!(Ok(ByName("foo bar".to_string())), Key::from_str("foo bar"));
}

#[test]
fn parse_key_from_name_with_spaces_and_dots() {
    assert_eq!(Ok(ByName("foo.bar".to_string())), Key::from_str("foo.bar"));
}

#[test]
fn parse_negative_range() {
    assert_eq!(Ok(Key::ByRange(-2, -1)), Key::from_str("[-2..-1]"));
}

#[test]
fn parse_key_from_empty_string() {
    assert_eq!(Ok(ByName("".to_string())), Key::from_str(""));
}

#[test]
fn parse_key_from_backwards_range() {
    assert_eq!(Ok(Key::ByRange(1, 2)), Key::from_str("2..1"));
}

#[test]
fn parse_key_from_redundant_range() {
    assert_eq!(Ok(Key::ByRange(1, 1)), Key::from_str("1..1"));
}
