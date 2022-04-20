use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Edit, SubCommand,
    },
    lookup_key::Key::*,
};

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
        }),
    );
}

#[test]
fn edit_without_description() {
    expect_parses_into(
        "todo edit 1 2 3",
        SubCommand::Edit(Edit {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            desc: None,
        }),
    );
}
