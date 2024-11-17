use todo_lookup_key::Key::*;

use crate::testing::expect_error;
use crate::testing::expect_parses_into;
use crate::SubCommand;
use crate::Unblock;

#[test]
fn unblock_no_keys() {
    expect_error("todo unblock");
    expect_error("todo unblock a --from");
}

#[test]
fn unblock_one_from_one() {
    expect_parses_into(
        "todo unblock 2 --from 1",
        SubCommand::Unblock(Unblock {
            keys: vec![ByNumber(2)],
            from: vec![ByNumber(1)],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_three_from_one() {
    expect_parses_into(
        "todo unblock 2 3 4 --from 0",
        SubCommand::Unblock(Unblock {
            keys: vec![ByNumber(2), ByNumber(3), ByNumber(4)],
            from: vec![ByNumber(0)],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_three_from_three() {
    expect_parses_into(
        "todo unblock 4 5 6 --from 1 2 3",
        SubCommand::Unblock(Unblock {
            keys: vec![ByNumber(4), ByNumber(5), ByNumber(6)],
            from: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_by_name() {
    expect_parses_into(
        "todo unblock a --from b",
        SubCommand::Unblock(Unblock {
            keys: vec![ByName("a".to_string())],
            from: vec![ByName("b".to_string())],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_include_done_long() {
    expect_parses_into(
        "todo unblock 2 --from 1 --include-done",
        SubCommand::Unblock(Unblock {
            keys: vec![ByNumber(2)],
            from: vec![ByNumber(1)],
            include_done: true,
        }),
    );
}

#[test]
fn unblock_include_done_short() {
    expect_parses_into(
        "todo unblock 2 --from 1 -d",
        SubCommand::Unblock(Unblock {
            keys: vec![ByNumber(2)],
            from: vec![ByNumber(1)],
            include_done: true,
        }),
    );
}
