use todo_lookup_key::Key::*;

use crate::testing::expect_parses_into;
use crate::SubCommand;
use crate::Top;

#[test]
fn top() {
    expect_parses_into(
        "todo top",
        SubCommand::Top(Top {
            include_done: false,
            ..Default::default()
        }),
    );
}

#[test]
fn top_include_done_long() {
    expect_parses_into(
        "todo top --include-done",
        SubCommand::Top(Top {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn top_include_done_short() {
    expect_parses_into(
        "todo top -d",
        SubCommand::Top(Top {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn top_with_keys() {
    expect_parses_into(
        "todo top 1 2",
        SubCommand::Top(Top {
            keys: vec![ByNumber(1), ByNumber(2)],
            ..Default::default()
        }),
    );
}
