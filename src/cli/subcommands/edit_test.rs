use cli::testing::expect_parses_into;
use cli::Edit;
use cli::Key::*;
use cli::SubCommand;

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
