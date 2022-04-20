use super::super::testing::expect_error;
use super::super::testing::expect_parses_into;
use super::super::SubCommand;
use super::Snoozed;

#[test]
fn snoozed_extraneous() {
    expect_error("todo snoozed foo");
}

#[test]
fn snoozed_no_args() {
    expect_parses_into("todo snoozed", SubCommand::Snoozed(Snoozed {}));
}
