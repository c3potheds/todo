use cli::testing::expect_error;
use cli::testing::expect_parses_into;
use cli::Key::*;
use cli::New;
use cli::SubCommand;

#[test]
fn new_missing_keys() {
    expect_error("todo new");
    expect_error("todo new a -b");
    expect_error("todo new a -p");
    expect_error("todo new a --before");
    expect_error("todo new a --after");
    expect_error("todo new a --due");
    expect_error("todo new a --budget");
}

#[test]
fn new_one() {
    expect_parses_into(
        "todo new a",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_three() {
    expect_parses_into(
        "todo new a b c",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_blocked_by_long() {
    expect_parses_into(
        "todo new b --blocked-by 1",
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![ByNumber(1)],
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_blocked_by_short() {
    expect_parses_into(
        "todo new b -p 1",
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![ByNumber(1)],
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_blocking_long() {
    expect_parses_into(
        "todo new b --blocking 1",
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![ByNumber(1)],
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_blocking_short() {
    expect_parses_into(
        "todo new c -b 1 2",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![ByNumber(1), ByNumber(2)],
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_blocking_by_name() {
    expect_parses_into(
        "todo new a.b -b b",
        SubCommand::New(New {
            desc: vec!["a.b".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![ByName("b".to_string())],
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_chain() {
    expect_parses_into(
        "todo new a b c --chain",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: true,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_before_after() {
    expect_parses_into(
        "todo new c --before a --after b",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: vec![ByName("a".to_string())],
            after: vec![ByName("b".to_string())],
            chain: false,
            priority: None,
            due: vec![],
            budget: vec![],
        }),
    );
}

#[test]
fn new_one_with_priority() {
    expect_parses_into(
        "todo new a --priority 1",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(1),
            due: vec![],
            budget: vec![],
        }),
    )
}

#[test]
fn new_three_with_priority() {
    expect_parses_into(
        "todo new a b c --priority 2",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(2),
            due: vec![],
            budget: vec![],
        }),
    )
}

#[test]
fn new_with_negative_priority() {
    expect_parses_into(
        "todo new a --priority -3",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(-3),
            due: vec![],
            budget: vec![],
        }),
    )
}

#[test]
fn new_with_due_date() {
    expect_parses_into(
        "todo new a --due 7 days",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec!["7".to_string(), "days".to_string()],
            budget: vec![],
        }),
    )
}

#[test]
fn new_with_budget() {
    expect_parses_into(
        "todo new a --budget 2 days",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: vec![],
            blocking: vec![],
            before: vec![],
            after: vec![],
            chain: false,
            priority: None,
            due: vec![],
            budget: vec!["2".to_string(), "days".to_string()],
        }),
    );
}