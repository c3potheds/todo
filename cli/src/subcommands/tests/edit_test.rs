use todo_lookup_key::Key::*;

use crate::testing::expect_error;
use crate::testing::expect_parses_into;
use crate::Edit;
use crate::SubCommand;

#[test]
fn edit_missing_keys() {
    expect_error("todo edit");
}

#[test]
fn edit_with_description() {
    expect_parses_into(
        "todo edit 10 --desc hello",
        SubCommand::Edit(Edit {
            keys: vec![ByNumber(10)],
            desc: Some("hello".to_string()),
            ..Default::default()
        }),
    );
}

#[test]
fn edit_without_description() {
    expect_parses_into(
        "todo edit 1 2 3",
        SubCommand::Edit(Edit {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            ..Default::default()
        }),
    );
}

#[test]
fn edit_done_long() {
    expect_parses_into(
        "todo edit 1 2 3 --include-done",
        SubCommand::Edit(Edit {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            desc: None,
            include_done: true,
        }),
    );
}

#[test]
fn edit_done_short() {
    expect_parses_into(
        "todo edit 1 2 3 -d",
        SubCommand::Edit(Edit {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            desc: None,
            include_done: true,
        }),
    );
}
