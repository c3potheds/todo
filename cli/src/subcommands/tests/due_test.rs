#![allow(clippy::zero_prefixed_literal)]

use todo_lookup_key::Key::*;
use todo_testing::ymdhms;

use crate::testing::expect_parses;
use crate::testing::expect_parses_into;
use crate::Due;
use crate::SubCommand;

#[test]
fn due_no_keys_no_date() {
    let now = ymdhms(2024, 12, 31, 0, 0, 0);
    expect_parses("todo due")
        .at_time(now)
        .into(SubCommand::Due(Due::default()));
}

#[test]
fn due_with_keys_but_no_date() {
    let now = ymdhms(2024, 12, 31, 00, 00, 00);
    expect_parses("todo due 1")
        .at_time(now)
        .into(SubCommand::Due(Due {
            keys: vec![ByNumber(1)],
            ..Default::default()
        }));
}

#[test]
fn due_with_date_but_no_keys() {
    let now = ymdhms(2024, 12, 31, 00, 00, 00);
    let end_of_2_days_later = ymdhms(2025, 01, 02, 23, 59, 59);
    expect_parses("todo due --in '2 days'")
        .at_time(now)
        .into(SubCommand::Due(Due {
            due: Some(end_of_2_days_later),
            ..Default::default()
        }));
}

#[test]
fn due_with_keys_and_date() {
    let fixed_now = ymdhms(2025, 01, 01, 00, 00, 00);
    let end_of_friday = ymdhms(2025, 01, 03, 23, 59, 59);
    expect_parses("todo due 10 --on friday")
        .at_time(fixed_now)
        .into(SubCommand::Due(Due {
            keys: vec![ByNumber(10)],
            due: Some(end_of_friday),
            ..Default::default()
        }));
}

#[test]
fn due_set_none() {
    expect_parses_into(
        "todo due 1 2 --none",
        SubCommand::Due(Due {
            keys: vec![ByNumber(1), ByNumber(2)],
            none: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_get_none() {
    expect_parses_into(
        "todo due --none",
        SubCommand::Due(Due {
            none: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_done_long() {
    expect_parses_into(
        "todo due --include-done",
        SubCommand::Due(Due {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_done_short() {
    expect_parses_into(
        "todo due -d",
        SubCommand::Due(Due {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_blocked_long() {
    expect_parses_into(
        "todo due --include-blocked",
        SubCommand::Due(Due {
            include_blocked: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_blocked_short() {
    expect_parses_into(
        "todo due -b",
        SubCommand::Due(Due {
            include_blocked: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_done_and_blocked() {
    expect_parses_into(
        "todo due -bd",
        SubCommand::Due(Due {
            include_done: true,
            include_blocked: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_none_include_done_and_blocked() {
    expect_parses_into(
        "todo due --none -bd",
        SubCommand::Due(Due {
            none: true,
            include_done: true,
            include_blocked: true,
            ..Default::default()
        }),
    );
}
