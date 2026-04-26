#![allow(clippy::zero_prefixed_literal)]

use todo_testing::ymdhms;

use crate::testing::expect_error;
use crate::testing::expect_parses;
use crate::testing::expect_parses_into;
use crate::Snoozed;
use crate::SubCommand;

#[test]
fn snoozed_extraneous() {
    expect_error("todo snoozed foo");
    expect_error("todo snoozed --until");
}

#[test]
fn snoozed_no_args() {
    expect_parses_into(
        "todo snoozed",
        SubCommand::Snoozed(Snoozed {
            ..Default::default()
        }),
    );
}

#[test]
fn snoozed_until_tomorrow() {
    let now = ymdhms(2025, 01, 04, 13, 00, 00);
    let tomorrow = ymdhms(2025, 01, 05, 23, 59, 59);
    expect_parses("todo snoozed --until tomorrow")
        .at_time(now)
        .into(SubCommand::Snoozed(Snoozed {
            until: Some(tomorrow),
        }));
}

#[test]
fn snoozed_until_5_days() {
    let now = ymdhms(2025, 01, 04, 13, 00, 00);
    let in_5_days = ymdhms(2025, 01, 09, 23, 59, 59);
    expect_parses("todo snoozed --until '5 days'")
        .at_time(now)
        .into(SubCommand::Snoozed(Snoozed {
            until: Some(in_5_days),
        }));
}
