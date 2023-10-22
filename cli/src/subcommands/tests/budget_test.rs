use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Budget, SubCommand,
    },
    todo_lookup_key::Key::*,
};

#[test]
fn missing_key_or_budget() {
    expect_error("todo budget");
    expect_error("todo budget --is 1 day");
    expect_error("todo budget 1 --is");
}

#[test]
fn assign_budget_to_one_key_by_number() {
    expect_parses_into(
        "todo budget 1 --is 1 day",
        SubCommand::Budget(Budget {
            keys: vec![ByNumber(1)],
            budget: vec!["1".to_string(), "day".to_string()],
            include_done: false,
        }),
    );
}

#[test]
fn assign_budget_to_three_keys() {
    expect_parses_into(
        "todo budget 1 2 3 --is 5 hours",
        SubCommand::Budget(Budget {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            budget: vec!["5".to_string(), "hours".to_string()],
            include_done: false,
        }),
    );
}

#[test]
fn assign_budget_by_range() {
    expect_parses_into(
        "todo budget 10..20 --is 1 week",
        SubCommand::Budget(Budget {
            keys: vec![ByRange(10, 20)],
            budget: vec!["1".to_string(), "week".to_string()],
            include_done: false,
        }),
    );
}

#[test]
fn assign_budget_by_name() {
    expect_parses_into(
        "todo budget a --is 30 min",
        SubCommand::Budget(Budget {
            keys: vec![ByName("a".to_string())],
            budget: vec!["30".to_string(), "min".to_string()],
            include_done: false,
        }),
    );
}

#[test]
fn reset_budget() {
    expect_parses_into(
        "todo budget 10 --is 0",
        SubCommand::Budget(Budget {
            keys: vec![ByNumber(10)],
            budget: vec!["0".to_string()],
            include_done: false,
        }),
    );
}

#[test]
fn budget_include_done_long() {
    expect_parses_into(
        "todo budget 10 --is 2 min --include-done",
        SubCommand::Budget(Budget {
            keys: vec![ByNumber(10)],
            budget: vec!["2".to_string(), "min".to_string()],
            include_done: true,
        }),
    )
}

#[test]
fn budget_include_done_short() {
    expect_parses_into(
        "todo budget 10 --is 2 min -d",
        SubCommand::Budget(Budget {
            keys: vec![ByNumber(10)],
            budget: vec!["2".to_string(), "min".to_string()],
            include_done: true,
        }),
    )
}
