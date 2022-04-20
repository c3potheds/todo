use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Snooze, SubCommand,
    },
    lookup_key::Key::*,
};

#[test]
fn snooze_missing_args() {
    expect_error("todo snooze");
    expect_error("todo snooze 1 --until");
}

#[test]
fn snooze_by_number() {
    expect_parses_into(
        "todo snooze 1 --until tomorrow",
        SubCommand::Snooze(Snooze {
            keys: vec![ByNumber(1)],
            until: vec!["tomorrow".to_string()],
        }),
    )
}

#[test]
fn snooze_by_name() {
    expect_parses_into(
        "todo snooze a --until saturday",
        SubCommand::Snooze(Snooze {
            keys: vec![ByName("a".to_string())],
            until: vec!["saturday".to_string()],
        }),
    )
}

#[test]
fn snooze_multiple_tasks() {
    expect_parses_into(
        "todo snooze a 1 --until 2 days",
        SubCommand::Snooze(Snooze {
            keys: vec![ByName("a".to_string()), ByNumber(1)],
            until: vec!["2".to_string(), "days".to_string()],
        }),
    )
}

#[test]
fn snooze_by_negative_number() {
    expect_parses_into(
        "todo snooze -1 --until august",
        SubCommand::Snooze(Snooze {
            keys: vec![ByNumber(-1)],
            until: vec!["august".to_string()],
        }),
    )
}
