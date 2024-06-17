use {
    crate::{testing::expect_parses_into, Due, SubCommand},
    todo_lookup_key::Key::*,
};

#[test]
fn due_no_keys_no_date() {
    expect_parses_into("todo due", SubCommand::Due(Due::default()));
}

#[test]
fn due_with_keys_but_no_date() {
    expect_parses_into(
        "todo due 1",
        SubCommand::Due(Due {
            keys: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn due_with_date_but_no_keys() {
    expect_parses_into(
        "todo due --in 2 days",
        SubCommand::Due(Due {
            due: Some(vec!["2".to_string(), "days".to_string()]),
            ..Default::default()
        }),
    );
}

#[test]
fn due_with_keys_and_date() {
    expect_parses_into(
        "todo due 10 --on friday",
        SubCommand::Due(Due {
            keys: vec![ByNumber(10)],
            due: Some(vec!["friday".to_string()]),
            ..Default::default()
        }),
    );
}

#[test]
fn due_set_none() {
    expect_parses_into(
        "todo due 1 2 --none",
        SubCommand::Due(Due {
            keys: vec![ByNumber(1), ByNumber(2)],
            none: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_get_none() {
    expect_parses_into(
        "todo due --none",
        SubCommand::Due(Due {
            none: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_done_long() {
    expect_parses_into(
        "todo due --include-done",
        SubCommand::Due(Due {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_done_short() {
    expect_parses_into(
        "todo due -d",
        SubCommand::Due(Due {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_blocked_long() {
    expect_parses_into(
        "todo due --include-blocked",
        SubCommand::Due(Due {
            include_blocked: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_blocked_short() {
    expect_parses_into(
        "todo due -b",
        SubCommand::Due(Due {
            include_blocked: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_include_done_and_blocked() {
    expect_parses_into(
        "todo due -bd",
        SubCommand::Due(Due {
            include_done: true,
            include_blocked: true,
            ..Default::default()
        }),
    );
}

#[test]
fn due_none_include_done_and_blocked() {
    expect_parses_into(
        "todo due --none -bd",
        SubCommand::Due(Due {
            none: true,
            include_done: true,
            include_blocked: true,
            ..Default::default()
        }),
    );
}
