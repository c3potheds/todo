use todo_lookup_key::Key::*;

use crate::testing::expect_error;
use crate::testing::expect_parses_into;
use crate::Split;
use crate::SubCommand;

#[test]
fn split_no_keys_or_prepositions() {
    expect_error("todo split");
    expect_error("todo split 1");
    expect_error("todo split 1 --into");
}

#[test]
fn split_one_into_one() {
    expect_parses_into(
        "todo split 1 --into a",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string()],
            ..Default::default()
        }),
    );
}

#[test]
fn split_one_into_three() {
    expect_parses_into(
        "todo split 1 --into a b c",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            ..Default::default()
        }),
    );
}

#[test]
fn split_three_into_two() {
    expect_parses_into(
        "todo split 1 2 3 --into a b",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            into: vec!["a".to_string(), "b".to_string()],
            ..Default::default()
        }),
    );
}

#[test]
fn split_into_chain() {
    expect_parses_into(
        "todo split 1 --into a b c --chain",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            chain: true,
            ..Default::default()
        }),
    );
}

#[test]
fn split_no_keep() {
    expect_parses_into(
        "todo split 1 --into a b",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string()],
            keep: false,
            ..Default::default()
        }),
    );
}

#[test]
fn split_keep() {
    expect_parses_into(
        "todo split 1 --into a b --keep",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string()],
            keep: true,
            ..Default::default()
        }),
    );
}

#[test]
fn split_keep_short() {
    expect_parses_into(
        "todo split 2 --into x y -k",
        SubCommand::Split(Split {
            keys: vec![ByNumber(2)],
            into: vec!["x".to_string(), "y".to_string()],
            keep: true,
            ..Default::default()
        }),
    );
}

#[test]
fn split_tag_long_true() {
    expect_parses_into(
        "todo split 1 --into a b --tag true",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string()],
            tag: Some(true),
            ..Default::default()
        }),
    )
}

#[test]
fn split_tag_short_true() {
    expect_parses_into(
        "todo split 1 --into a b -t true",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string()],
            tag: Some(true),
            ..Default::default()
        }),
    )
}

#[test]
fn split_tag_long_false() {
    expect_parses_into(
        "todo split 1 --into a b --tag false",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string()],
            tag: Some(false),
            ..Default::default()
        }),
    )
}

#[test]
fn split_tag_short_false() {
    expect_parses_into(
        "todo split 1 --into a b -t false",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string()],
            tag: Some(false),
            ..Default::default()
        }),
    )
}
