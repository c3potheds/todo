use super::super::testing::expect_error;
use super::super::testing::expect_parses_into;
use super::super::Key::*;
use super::super::Path;
use super::super::SubCommand;

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
