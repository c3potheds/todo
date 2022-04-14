use cli::testing::expect_error;
use cli::testing::expect_parses_into;
use cli::Snoozed;
use cli::SubCommand;

#[test]
fn snoozed_extraneous() {
    expect_error("todo snoozed foo");
}

#[test]
fn snoozed_no_args() {
    expect_parses_into("todo snoozed", SubCommand::Snoozed(Snoozed {}));
}
