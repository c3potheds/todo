use todo_lookup_key::Key::*;

use crate::testing::expect_error;
use crate::testing::expect_parses_into;
use crate::New;
use crate::SubCommand;

#[test]
fn new_missing_keys() {
    expect_error("todo new");
    expect_error("todo new a -b");
    expect_error("todo new a -p");
    expect_error("todo new a --before");
    expect_error("todo new a --after");
    expect_error("todo new a --by");
    expect_error("todo new a --due");
    expect_error("todo new a --budget");
    expect_error("todo new a --snooze");
}

#[test]
fn new_one() {
    expect_parses_into(
        "todo new a",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            ..Default::default()
        }),
    );
}

#[test]
fn new_three() {
    expect_parses_into(
        "todo new a b c",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        }),
    );
}

#[test]
fn new_blocking_long() {
    expect_parses_into(
        "todo new b --blocking 1",
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocking: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn new_blocking_short() {
    expect_parses_into(
        "todo new c -b 1 2",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocking: vec![ByNumber(1), ByNumber(2)],
            ..Default::default()
        }),
    );
}

#[test]
fn new_blocking_by_name() {
    expect_parses_into(
        "todo new a.b -b b",
        SubCommand::New(New {
            desc: vec!["a.b".to_string()],
            blocking: vec![ByName("b".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn new_chain() {
    expect_parses_into(
        "todo new a b c --chain",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            chain: true,
            ..Default::default()
        }),
    );
}

#[test]
fn new_before_after() {
    expect_parses_into(
        "todo new c --before a --after b",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            before: vec![ByName("a".to_string())],
            after: vec![ByName("b".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn new_before_after_short() {
    expect_parses_into(
        "todo new c -B a -A b",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            before: vec![ByName("a".to_string())],
            after: vec![ByName("b".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn new_by_long() {
    expect_parses_into(
        "todo new c --by d",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            by: vec![ByName("d".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn new_by_long_many() {
    expect_parses_into(
        "todo new a b c --by d e f",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            by: vec![
                ByName("d".to_string()),
                ByName("e".to_string()),
                ByName("f".to_string()),
            ],
            ..Default::default()
        }),
    );
}

#[test]
fn new_by_short() {
    expect_parses_into(
        "todo new c -Y d",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            by: vec![ByName("d".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn new_by_short_many() {
    expect_parses_into(
        "todo new a b c -Y x y z",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            by: vec![
                ByName("x".to_string()),
                ByName("y".to_string()),
                ByName("z".to_string()),
            ],
            ..Default::default()
        }),
    );
}

#[test]
fn new_one_with_priority() {
    expect_parses_into(
        "todo new a --priority 1",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            priority: Some(1),
            ..Default::default()
        }),
    )
}

#[test]
fn new_three_with_priority() {
    expect_parses_into(
        "todo new a b c --priority 2",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            priority: Some(2),
            ..Default::default()
        }),
    )
}

#[test]
fn new_with_negative_priority() {
    expect_parses_into(
        "todo new a --priority -3",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            priority: Some(-3),
            ..Default::default()
        }),
    )
}

#[test]
fn new_with_due_date() {
    expect_parses_into(
        "todo new a --due 7 days",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            due: vec!["7".to_string(), "days".to_string()],
            ..Default::default()
        }),
    )
}

#[test]
fn new_with_budget() {
    expect_parses_into(
        "todo new a --budget 2 days",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            budget: vec!["2".to_string(), "days".to_string()],
            ..Default::default()
        }),
    );
}

#[test]
fn new_snooze_long() {
    expect_parses_into(
        "todo new a --snooze tomorrow",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            snooze: vec!["tomorrow".to_string()],
            ..Default::default()
        }),
    );
}

#[test]
fn new_snooze_short() {
    expect_parses_into(
        "todo new a -s 2 days",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            snooze: vec!["2".to_string(), "days".to_string()],
            ..Default::default()
        }),
    );
}

#[test]
fn new_done() {
    expect_parses_into(
        "todo new a --done",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn new_done_short() {
    expect_parses_into(
        "todo new a -d",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn new_blocked_by_range() {
    expect_parses_into(
        "todo new a -p 1..2",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: vec![ByRange(1, 2)],
            ..Default::default()
        }),
    )
}

#[test]
fn new_blocked_by_negative_range() {
    expect_parses_into(
        "todo new a -p [-20..-10]",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: vec![ByRange(-20, -10)],
            ..Default::default()
        }),
    )
}

#[test]
fn new_blocking_negative_range() {
    expect_parses_into(
        "todo new a -b [-2..0]",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocking: vec![ByRange(-2, 0)],
            ..Default::default()
        }),
    )
}

#[test]
fn new_single_tag() {
    expect_parses_into(
        "todo new a --tag",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            tag: true,
            ..Default::default()
        }),
    )
}

#[test]
fn new_multiple_tags() {
    expect_parses_into(
        "todo new a b --tag",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string()],
            tag: true,
            ..Default::default()
        }),
    )
}

#[test]
fn new_single_tag_short() {
    expect_parses_into(
        "todo new a -t",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            tag: true,
            ..Default::default()
        }),
    )
}

#[test]
fn new_multiple_tags_short() {
    expect_parses_into(
        "todo new a b -t",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string()],
            tag: true,
            ..Default::default()
        }),
    )
}
