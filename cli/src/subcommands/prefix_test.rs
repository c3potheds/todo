use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Prefix, SubCommand,
    },
    lookup_key::Key::*,
};

#[test]
fn prefix_missing_args() {
    expect_error("todo prefix");
    expect_error("todo prefix 1");
    expect_error("todo prefix 1 -P");
}

#[test]
fn prefix_one_long() {
    expect_parses_into(
        "todo prefix 1 --prefix x",
        SubCommand::Prefix(Prefix {
            keys: vec![ByNumber(1)],
            prefix: vec!["x".to_string()],
        }),
    );
}

#[test]
fn prefix_three_long() {
    expect_parses_into(
        "todo prefix 1 2 3 --prefix x",
        SubCommand::Prefix(Prefix {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            prefix: vec!["x".to_string()],
        }),
    );
}

#[test]
fn prefix_task_with_negative_number() {
    expect_parses_into(
        "todo prefix -1 -P x",
        SubCommand::Prefix(Prefix {
            keys: vec![ByNumber(-1)],
            prefix: vec!["x".to_string()],
        }),
    );
}

#[test]
fn prefix_with_multiple_prefixes() {
    expect_parses_into(
        "todo prefix 1 -P x y z",
        SubCommand::Prefix(Prefix {
            keys: vec![ByNumber(1)],
            prefix: vec!["x".to_string(), "y".to_string(), "z".to_string()],
        }),
    );
}
