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
fn get_one() {
    expect_parses_into(
        "todo get 1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            include_done: false,
            no_context: false,
        }),
    );
}

#[test]
fn get_three() {
    expect_parses_into(
        "todo get 1 2 3",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            include_done: false,
            no_context: false,
        }),
    );
}

#[test]
fn get_by_name() {
    expect_parses_into(
        "todo get a",
        SubCommand::Get(Get {
            keys: vec![ByName("a".to_string())],
            include_done: false,
            no_context: false,
        }),
    );
}

#[test]
fn get_negative() {
    expect_parses_into(
        "todo get -1",
        SubCommand::Get(Get {
            keys: vec![ByNumber(-1)],
            include_done: false,
            no_context: false,
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
            no_context: false,
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
            no_context: false,
        }),
    );
}

#[test]
fn get_no_context() {
    expect_parses_into(
        "todo get 1 --no-context",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            include_done: false,
            no_context: true,
        }),
    );
}

#[test]
fn get_no_context_short() {
    expect_parses_into(
        "todo get 1 -n",
        SubCommand::Get(Get {
            keys: vec![ByNumber(1)],
            include_done: false,
            no_context: true,
        }),
    );
}
