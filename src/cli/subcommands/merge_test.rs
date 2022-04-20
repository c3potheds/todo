use super::super::testing::expect_error;
use super::super::testing::expect_parses_into;
use super::super::Key::*;
use super::super::Merge;
use super::super::SubCommand;

#[test]
fn merge_requires_at_least_two_and_into() {
    expect_error("todo merge");
    expect_error("todo merge 1");
    expect_error("todo merge 1 2");
    expect_error("todo merge --into aa");
    expect_error("todo merge 1 --into aa");
}

#[test]
fn merge_two() {
    expect_parses_into(
        "todo merge 1 2 --into ab",
        SubCommand::Merge(Merge {
            keys: vec![ByNumber(1), ByNumber(2)],
            into: "ab".to_string(),
        }),
    );
}

#[test]
fn merge_three() {
    expect_parses_into(
        "todo merge -1 -2 -3 --into abc",
        SubCommand::Merge(Merge {
            keys: vec![ByNumber(-1), ByNumber(-2), ByNumber(-3)],
            into: "abc".to_string(),
        }),
    );
}
