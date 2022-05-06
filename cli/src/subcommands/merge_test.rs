use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Merge, SubCommand,
    },
    lookup_key::Key::*,
};

#[test]
fn merge_requires_at_least_one_and_into() {
    expect_error("todo merge");
    expect_error("todo merge 1");
    expect_error("todo merge 1 2");
    expect_error("todo merge --into aa");
}

#[test]
fn merge_one() {
    expect_parses_into(
        "todo merge 1 --into aa",
        SubCommand::Merge(Merge {
            keys: vec![ByNumber(1)],
            into: "aa".to_string(),
        }),
    );
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
