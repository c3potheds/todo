use crate::{
    testing::{expect_error, expect_parses_into},
    Snoozed, SubCommand,
};

#[test]
fn snoozed_extraneous() {
    expect_error("todo snoozed foo");
    expect_error("todo snoozed --until");
}

#[test]
fn snoozed_no_args() {
    expect_parses_into(
        "todo snoozed",
        SubCommand::Snoozed(Snoozed {
            ..Default::default()
        }),
    );
}

#[test]
fn snoozed_until_tomorrow() {
    expect_parses_into(
        "todo snoozed --until tomorrow",
        SubCommand::Snoozed(Snoozed {
            until: Some(vec!["tomorrow".to_string()]),
        }),
    );
}

#[test]
fn snoozed_until_5_days() {
    expect_parses_into(
        "todo snoozed --until 5 days",
        SubCommand::Snoozed(Snoozed {
            until: Some(vec!["5".to_string(), "days".to_string()]),
        }),
    )
}
