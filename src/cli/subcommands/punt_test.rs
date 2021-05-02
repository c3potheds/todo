use cli::testing::expect_error;
use cli::testing::expect_parses_into;
use cli::Key::*;
use cli::Punt;
use cli::SubCommand;

#[test]
fn punt_no_keys() {
    expect_error("todo punt");
}

#[test]
fn punt_one() {
    expect_parses_into(
        "todo punt 1",
        SubCommand::Punt(Punt {
            keys: vec![ByNumber(1)],
        }),
    );
}

#[test]
fn punt_three() {
    expect_parses_into(
        "todo punt 1 2 3",
        SubCommand::Punt(Punt {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
        }),
    );
}

#[test]
fn punt_by_name() {
    expect_parses_into(
        "todo punt a",
        SubCommand::Punt(Punt {
            keys: vec![ByName("a".to_string())],
        }),
    )
}
