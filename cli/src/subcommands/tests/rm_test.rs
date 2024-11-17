use todo_lookup_key::Key::*;

use crate::testing::expect_error;
use crate::testing::expect_parses_into;
use crate::Rm;
use crate::SubCommand;

#[test]
fn rm_no_keys() {
    expect_error("todo rm");
}

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
