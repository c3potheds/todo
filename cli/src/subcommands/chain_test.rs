use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Chain, SubCommand,
    },
    lookup_key::Key::*,
};

#[test]
fn chain_missing_keys() {
    expect_error("todo chain");
}

#[test]
fn chain_one() {
    expect_parses_into(
        "todo chain 1",
        SubCommand::Chain(Chain {
            keys: vec![ByNumber(1)],
            include_done: false,
        }),
    );
}

#[test]
fn chain_three() {
    expect_parses_into(
        "todo chain 10 20 30",
        SubCommand::Chain(Chain {
            keys: vec![ByNumber(10), ByNumber(20), ByNumber(30)],
            include_done: false,
        }),
    );
}

#[test]
fn chain_by_range() {
    expect_parses_into(
        "todo chain 1..5",
        SubCommand::Chain(Chain {
            keys: vec![ByRange(1, 5)],
            include_done: false,
        }),
    );
}
