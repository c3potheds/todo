use {
    crate::{
        testing::{expect_error, expect_parses_into},
        SubCommand, Unsnooze,
    },
    lookup_key::Key::*,
};

#[test]
fn unsnooze_no_keys_is_error() {
    expect_error("todo unsnooze");
}

#[test]
fn unsnooze_one_key() {
    expect_parses_into(
        "todo unsnooze 1",
        SubCommand::Unsnooze(Unsnooze {
            keys: vec![ByNumber(1)],
        }),
    );
}

#[test]
fn unsnooze_three_keys() {
    expect_parses_into(
        "todo unsnooze 1 2 3",
        SubCommand::Unsnooze(Unsnooze {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
        }),
    );
}

#[test]
fn unsnooze_by_name() {
    expect_parses_into(
        "todo unsnooze a",
        SubCommand::Unsnooze(Unsnooze {
            keys: vec![ByName("a".to_string())],
        }),
    );
}

#[test]
fn unsnooze_negative_number_key() {
    expect_parses_into(
        "todo unsnooze -1",
        SubCommand::Unsnooze(Unsnooze {
            keys: vec![ByNumber(-1)],
        }),
    );
}

#[test]
fn unsnooze_range() {
    expect_parses_into(
        "todo unsnooze 1..3",
        SubCommand::Unsnooze(Unsnooze {
            keys: vec![ByRange(1, 3)],
        }),
    );
}

#[test]
fn unsnooze_range_with_negative_number() {
    expect_parses_into(
        "todo unsnooze [-1..3]",
        SubCommand::Unsnooze(Unsnooze {
            keys: vec![ByRange(-1, 3)],
        }),
    );
}
