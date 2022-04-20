use crate::cli::testing::expect_parses_into;
use crate::cli::Options;
use crate::cli::SubCommand;
use structopt::StructOpt;

fn parse<I>(args: I) -> Options
where
    I: IntoIterator,
    I::Item: Into<std::ffi::OsString> + Clone,
{
    Options::from_iter_safe(args).expect("Could not parse args")
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
