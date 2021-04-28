use cli::testing::expect_parses_into;
use cli::Key::*;
use cli::Rm;
use cli::SubCommand;

#[test]
fn rm_by_number() {
    expect_parses_into(
        "todo rm 1 2",
        SubCommand::Rm(Rm {
            keys: vec![ByNumber(1), ByNumber(2)],
        }),
    );
}

#[test]
fn rm_by_name() {
    expect_parses_into(
        "todo rm a b c",
        SubCommand::Rm(Rm {
            keys: vec![
                ByName("a".to_string()),
                ByName("b".to_string()),
                ByName("c".to_string()),
            ],
        }),
    );
}
