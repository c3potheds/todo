use cli::testing::expect_parses_into;
use cli::Due;
use cli::Key::*;
use cli::SubCommand;

#[test]
fn due_no_keys_no_date() {
    expect_parses_into(
        "todo due",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_with_keys_but_no_date() {
    expect_parses_into(
        "todo due 1",
        SubCommand::Due(Due {
            keys: vec![ByNumber(1)],
            due: vec![],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_with_date_but_no_keys() {
    expect_parses_into(
        "todo due --in 2 days",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec!["2".to_string(), "days".to_string()],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_with_keys_and_date() {
    expect_parses_into(
        "todo due 10 --on friday",
        SubCommand::Due(Due {
            keys: vec![ByNumber(10)],
            due: vec!["friday".to_string()],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_set_none() {
    expect_parses_into(
        "todo due 1 2 --none",
        SubCommand::Due(Due {
            keys: vec![ByNumber(1), ByNumber(2)],
            due: vec![],
            none: true,
            include_done: false,
        }),
    );
}

#[test]
fn due_get_none() {
    expect_parses_into(
        "todo due --none",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: true,
            include_done: false,
        }),
    );
}

#[test]
fn due_include_done_long() {
    expect_parses_into(
        "todo due --include-done",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: false,
            include_done: true,
        }),
    );
}

#[test]
fn due_include_done_short() {
    expect_parses_into(
        "todo due -d",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: false,
            include_done: true,
        }),
    );
}
