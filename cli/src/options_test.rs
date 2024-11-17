use clap::Parser;

use crate::testing::expect_parses_into;
use crate::Options;
use crate::SubCommand;

fn parse<I>(args: I) -> Options
where
    I: IntoIterator,
    I::Item: Into<std::ffi::OsString> + Clone,
{
    Options::try_parse_from(args).expect("Could not parse args")
}

#[test]
fn empty_defaults_to_status() {
    let options = parse(&["todo"]);
    assert_eq!(options.cmd, None);
}

#[test]
fn status_include_blocked() {
    let options = parse(&["todo", "-b"]);
    assert_eq!(options.cmd, None);
    assert!(options.include_blocked);
    assert!(!options.include_done);
}

#[test]
fn status_include_done() {
    let options = parse(&["todo", "-d"]);
    assert_eq!(options.cmd, None);
    assert!(!options.include_blocked);
    assert!(options.include_done);
}

#[test]
fn status_include_all() {
    let options = parse(&["todo", "-a"]);
    assert_eq!(options.cmd, None);
    assert!(options.include_all);
}

#[test]
fn log() {
    expect_parses_into("todo log", SubCommand::Log);
}
