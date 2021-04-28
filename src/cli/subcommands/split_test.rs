use cli::testing::expect_parses_into;
use cli::Key::*;
use cli::Split;
use cli::SubCommand;

#[test]
fn split_one_into_one() {
    expect_parses_into(
        "todo split 1 --into a",
        SubCommand::Split(Split {
            keys: vec![ByNumber(1)],
            into: vec!["a".to_string()],
            chain: false,
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
            chain: false,
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
            chain: false,
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
        }),
    );
}
