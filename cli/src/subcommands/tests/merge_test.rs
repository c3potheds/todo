use todo_lookup_key::Key::*;

use crate::testing::expect_error;
use crate::testing::expect_parses_into;
use crate::Merge;
use crate::SubCommand;

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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        }),
    );
}

#[test]
fn merge_into_tag_true_long() {
    expect_parses_into(
        "todo merge 1 2 --into c --tag true",
        SubCommand::Merge(Merge {
            keys: vec![ByNumber(1), ByNumber(2)],
            into: "c".to_string(),
            tag: Some(true),
        }),
    )
}

#[test]
fn merge_into_tag_false_long() {
    expect_parses_into(
        "todo merge 1 2 --into c --tag false",
        SubCommand::Merge(Merge {
            keys: vec![ByNumber(1), ByNumber(2)],
            into: "c".to_string(),
            tag: Some(false),
        }),
    )
}

#[test]
fn merge_into_tag_true_short() {
    expect_parses_into(
        "todo merge 1 2 --into c -t true",
        SubCommand::Merge(Merge {
            keys: vec![ByNumber(1), ByNumber(2)],
            into: "c".to_string(),
            tag: Some(true),
        }),
    )
}

#[test]
fn merge_into_tag_false_short() {
    expect_parses_into(
        "todo merge 1 2 --into c -t false",
        SubCommand::Merge(Merge {
            keys: vec![ByNumber(1), ByNumber(2)],
            into: "c".to_string(),
            tag: Some(false),
        }),
    )
}
