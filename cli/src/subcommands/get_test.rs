use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Get, SubCommand,
    },
    lookup_key::Key::*,
};

#[test]
fn get_missing_keys() {
    expect_error("todo get");
}

#[test]
fn get_mutually_exclusive_args() {
    expect_error("todo get --no-context --blocked-by 0");
    expect_error("todo get --no-context --blocking 0");
    expect_error("todo get --blocked-by --blocking 0");
    expect_error("todo get -nb 0");
    expect_error("todo get -np 0");
    expect_error("todo get -bp 0");
}

#[test]
fn get_one() {
    expect_parses_into(
        "todo get 1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn get_three() {
    expect_parses_into(
        "todo get 1 2 3",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            ..Default::default()
        }),
    );
}

#[test]
fn get_by_name() {
    expect_parses_into(
        "todo get a",
        SubCommand::Get(Get {
            keys: vec![ByName("a".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn get_negative() {
    expect_parses_into(
        "todo get -1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(-1)],
            ..Default::default()
        }),
    );
}

#[test]
fn get_include_done_long() {
    expect_parses_into(
        "todo get 1 --include-done",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn get_include_done_short() {
    expect_parses_into(
        "todo get 1 -d",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn get_no_context() {
    expect_parses_into(
        "todo get 1 --no-context",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            no_context: true,
            ..Default::default()
        }),
    );
}

#[test]
fn get_no_context_short() {
    expect_parses_into(
        "todo get 1 -n",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            no_context: true,
            ..Default::default()
        }),
    );
}

#[test]
fn get_blocked_by_long() {
    expect_parses_into(
        "todo get --blocked-by 1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            blocked_by: true,
            ..Default::default()
        }),
    );
}

#[test]
fn get_blocked_by_short() {
    expect_parses_into(
        "todo get -p 1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            blocked_by: true,
            ..Default::default()
        }),
    );
}

#[test]
fn get_blocking_long() {
    expect_parses_into(
        "todo get --blocking 1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            blocking: true,
            ..Default::default()
        }),
    );
}

#[test]
fn get_blocking_short() {
    expect_parses_into(
        "todo get -b 1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            blocking: true,
            ..Default::default()
        }),
    );
}
