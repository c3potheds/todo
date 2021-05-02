use cli::testing::expect_error;
use cli::testing::expect_parses_into;
use cli::Check;
use cli::Key::*;
use cli::SubCommand;

#[test]
fn check_missing_keys() {
    expect_error("todo check");
}

#[test]
fn check_one() {
    expect_parses_into(
        "todo check 1",
        SubCommand::Check(Check {
            keys: vec![ByNumber(1)],
            force: false,
        }),
    );
}

#[test]
fn check_three() {
    expect_parses_into(
        "todo check 1 2 3",
        SubCommand::Check(Check {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            force: false,
        }),
    );
}

#[test]
fn check_by_name() {
    expect_parses_into(
        "todo check a",
        SubCommand::Check(Check {
            keys: vec![ByName("a".to_string())],
            force: false,
        }),
    )
}

#[test]
fn check_force() {
    expect_parses_into(
        "todo check 10 --force",
        SubCommand::Check(Check {
            keys: vec![ByNumber(10)],
            force: true,
        }),
    )
}
