use crate::{
    testing::{expect_error, expect_parses_into},
    Snoozed, SubCommand,
};

#[test]
fn snoozed_extraneous() {
    expect_error("todo snoozed foo");
}

#[test]
fn snoozed_no_args() {
    expect_parses_into("todo snoozed", SubCommand::Snoozed(Snoozed {}));
}
