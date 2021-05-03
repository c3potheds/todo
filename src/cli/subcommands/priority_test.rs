use cli::testing::expect_parses_into;
use cli::Key::*;
use cli::Priority;
use cli::SubCommand;

#[test]
fn priority_query_all() {
    expect_parses_into(
        "todo priority",
        SubCommand::Priority(Priority {
            keys: vec![],
            priority: None,
            include_done: false,
        }),
    );
}

#[test]
fn priority_query_task() {
    expect_parses_into(
        "todo priority 1",
        SubCommand::Priority(Priority {
            keys: vec![ByNumber(1)],
            priority: None,
            include_done: false,
        }),
    );
}

#[test]
fn priority_query_priority() {
    expect_parses_into(
        "todo priority --is 1",
        SubCommand::Priority(Priority {
            keys: vec![],
            priority: Some(1),
            include_done: false,
        }),
    );
}

#[test]
fn priority_assign_to_one_task() {
    expect_parses_into(
        "todo priority 1 --is 2",
        SubCommand::Priority(Priority {
            keys: vec![ByNumber(1)],
            priority: Some(2),
            include_done: false,
        }),
    );
}

#[test]
fn priority_assign_to_three_tasks() {
    expect_parses_into(
        "todo priority 1 2 3 --is -1",
        SubCommand::Priority(Priority {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            priority: Some(-1),
            include_done: false,
        }),
    );
}