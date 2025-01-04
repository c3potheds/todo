use chrono::DateTime;
use chrono::Utc;
use clap::Parser;
use pretty_assertions::assert_eq;

use crate::time_utils::override_now;
use crate::time_utils::OverridingNow;
use crate::Options;
use crate::SubCommand;

#[track_caller]
pub fn expect_parses_into<'a, S: Into<&'a str>>(args: S, expected: SubCommand) {
    let caller = std::panic::Location::caller();
    expect_parses_into_impl(caller, args, expected)
}

fn expect_parses_into_impl<'a, S: Into<&'a str>>(
    caller: &std::panic::Location<'_>,
    args: S,
    expected: SubCommand,
) {
    let s = args.into();
    let args = shlex::split(s).unwrap_or_else(|| {
        panic!("In {caller}\n\tCould not split args {s:?}");
    });
    let options = Options::try_parse_from(args.iter()).unwrap_or_else(|e| {
        panic!("In {caller}\n\tCould not parse args {s:?}\n\t{e:#?}");
    });
    let cmd = options.cmd.unwrap();
    assert_eq!(cmd, expected, "In {caller}");
}

pub fn expect_error<'a, S: Into<&'a str>>(args: S) {
    let s = args.into();
    Options::try_parse_from(s.split(' '))
        .expect_err(&format!("Was not a parse error: '{}'", s));
}

pub struct ExpectParses<'a> {
    caller: &'static std::panic::Location<'static>,
    args: &'a str,
    now_guard: Option<OverridingNow>,
}

impl ExpectParses<'_> {
    pub fn at_time(mut self, now: DateTime<Utc>) -> Self {
        self.now_guard = Some(override_now(now));
        self
    }

    pub fn into(self, expected: SubCommand) {
        expect_parses_into_impl(self.caller, self.args, expected);
    }
}

#[track_caller]
pub fn expect_parses(args: &str) -> ExpectParses<'_> {
    let caller = std::panic::Location::caller();
    ExpectParses {
        caller,
        args,
        now_guard: None,
    }
}
