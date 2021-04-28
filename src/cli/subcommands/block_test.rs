use cli::testing::expect_parses_into;
use cli::Block;
use cli::Key::*;
use cli::SubCommand;

#[test]
fn block_one_on_one() {
    expect_parses_into(
        "todo block 2 --on 1",
        SubCommand::Block(Block {
            keys: vec![ByNumber(2)],
            on: vec![ByNumber(1)],
            include_done: false,
        }),
    );
}

#[test]
fn block_three_on_one() {
    expect_parses_into(
        "todo block 1 2 3 --on 4",
        SubCommand::Block(Block {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            on: vec![ByNumber(4)],
            include_done: false,
        }),
    );
}

#[test]
fn block_three_on_three() {
    expect_parses_into(
        "todo block 1 2 3 --on 4 5 6",
        SubCommand::Block(Block {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            on: vec![ByNumber(4), ByNumber(5), ByNumber(6)],
            include_done: false,
        }),
    );
}

#[test]
fn block_by_name() {
    expect_parses_into(
        "todo block a --on b",
        SubCommand::Block(Block {
            keys: vec![ByName("a".to_string())],
            on: vec![ByName("b".to_string())],
            include_done: false,
        }),
    );
}

#[test]
fn block_include_done_long() {
    expect_parses_into(
        "todo block 1 --on 2 --include-done",
        SubCommand::Block(Block {
            keys: vec![ByNumber(1)],
            on: vec![ByNumber(2)],
            include_done: true,
        }),
    );
}

#[test]
fn block_include_done_short() {
    expect_parses_into(
        "todo block 1 --on 2 -d",
        SubCommand::Block(Block {
            keys: vec![ByNumber(1)],
            on: vec![ByNumber(2)],
            include_done: true,
        }),
    );
}
