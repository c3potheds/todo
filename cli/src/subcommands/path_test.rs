use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Path, SubCommand,
    },
    todo_lookup_key::Key::*,
};

#[test]
fn path_by_number() {
    expect_parses_into(
        "todo path 10 20",
        SubCommand::Path(Path {
            keys: vec![ByNumber(10), ByNumber(20)],
        }),
    );
}

#[test]
fn path_by_name() {
    expect_parses_into(
        "todo path a b",
        SubCommand::Path(Path {
            keys: vec![ByName("a".to_string()), ByName("b".to_string())],
        }),
    );
}

#[test]
fn path_missing_keys() {
    expect_error("todo path");
}
