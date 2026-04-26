#![allow(clippy::zero_prefixed_literal)]

use todo_lookup_key::Key::*;
use todo_testing::ymdhms;

use crate::testing::expect_error;
use crate::testing::expect_parses;
use crate::Snooze;
use crate::SubCommand;

#[test]
fn snooze_missing_args() {
    expect_error("todo snooze");
    expect_error("todo snooze 1 --until");
}

#[test]
fn snooze_by_number() {
    let now = ymdhms(2025, 01, 03, 11, 00, 00);
    expect_parses("todo snooze 1 --until tomorrow")
        .at_time(now)
        .into(SubCommand::Snooze(Snooze {
            keys: vec![ByNumber(1)],
            until: ymdhms(2025, 01, 04, 00, 00, 00),
        }))
}

#[test]
fn snooze_by_name() {
    let now = ymdhms(2025, 01, 03, 11, 00, 00);
    expect_parses("todo snooze a --until saturday")
        .at_time(now)
        .into(SubCommand::Snooze(Snooze {
            keys: vec![ByName("a".to_string())],
            until: ymdhms(2025, 01, 04, 00, 00, 00),
        }))
}

#[test]
fn snooze_multiple_tasks() {
    let now = ymdhms(2025, 01, 03, 11, 00, 00);
    expect_parses("todo snooze a 1 --until '2 days'")
        .at_time(now)
        .into(SubCommand::Snooze(Snooze {
            keys: vec![ByName("a".to_string()), ByNumber(1)],
            until: ymdhms(2025, 01, 05, 00, 00, 00),
        }))
}

#[test]
fn snooze_by_negative_number() {
    let now = ymdhms(2025, 01, 03, 11, 00, 00);
    expect_parses("todo snooze -1 --until august")
        .at_time(now)
        .into(SubCommand::Snooze(Snooze {
            keys: vec![ByNumber(-1)],
            until: ymdhms(2025, 08, 01, 00, 00, 00),
        }))
}
