use todo_lookup_key::Key::*;

use crate::testing::expect_parses_into;
use crate::Bottom;
use crate::SubCommand;

#[test]
fn bottom() {
    expect_parses_into(
        "todo bottom",
        SubCommand::Bottom(Bottom {
            include_done: false,
            ..Default::default()
        }),
    );
}

#[test]
fn bottom_include_done_long() {
    expect_parses_into(
        "todo bottom --include-done",
        SubCommand::Bottom(Bottom {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn bottom_include_done_short() {
    expect_parses_into(
        "todo bottom -d",
        SubCommand::Bottom(Bottom {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn bottom_with_keys() {
    expect_parses_into(
        "todo bottom 1 2",
        SubCommand::Bottom(Bottom {
            keys: vec![ByNumber(1), ByNumber(2)],
            ..Default::default()
        }),
    );
}
