use super::Options;
use super::SubCommand;
use structopt::StructOpt;

pub fn expect_parses_into<'a, S: Into<&'a str>>(args: S, expected: SubCommand) {
    let s = args.into();
    let options =
        Options::from_iter_safe(s.split(' ')).expect("Could not parse args");
    let cmd = options.cmd.unwrap();
    assert_eq!(cmd, expected);
}

pub fn expect_error<'a, S: Into<&'a str>>(args: S) {
    let s = args.into();
    Options::from_iter_safe(s.split(' '))
        .expect_err(&format!("Was not a parse error: '{}'", s));
}
