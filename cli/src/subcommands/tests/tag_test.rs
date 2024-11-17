use todo_lookup_key::Key::*;

use crate::testing::expect_error;
use crate::testing::expect_parses_into;
use crate::SubCommand;
use crate::Tag;

#[test]
fn tag_not_enough_keys_to_unmark() {
    expect_error("todo tag --unmark");
    expect_error("todo tag 1 --unmark");
    expect_error("todo tag 1 2 --unmark");
}

#[test]
fn tag_show_all() {
    expect_parses_into(
        "todo tag",
        SubCommand::Tag(Tag {
            ..Default::default()
        }),
    );
}

#[test]
fn tag_mark_single() {
    expect_parses_into(
        "todo tag 1",
        SubCommand::Tag(Tag {
            keys: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_mark_multiple() {
    expect_parses_into(
        "todo tag 1 2 3",
        SubCommand::Tag(Tag {
            keys: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_mark_by_name() {
    expect_parses_into(
        "todo tag a",
        SubCommand::Tag(Tag {
            keys: vec![ByName("a".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_unmark_single_long() {
    expect_parses_into(
        "todo tag --unmark 1",
        SubCommand::Tag(Tag {
            unmark: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_unmark_multiple_long() {
    expect_parses_into(
        "todo tag --unmark 1 2 3",
        SubCommand::Tag(Tag {
            unmark: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_unmark_single_short() {
    expect_parses_into(
        "todo tag -u 1",
        SubCommand::Tag(Tag {
            unmark: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_unmark_multiple_short() {
    expect_parses_into(
        "todo tag -u 1 2 3",
        SubCommand::Tag(Tag {
            unmark: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_unmark_by_name_long() {
    expect_parses_into(
        "todo tag --unmark a",
        SubCommand::Tag(Tag {
            unmark: vec![ByName("a".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_unmark_by_name_short() {
    expect_parses_into(
        "todo tag -u a",
        SubCommand::Tag(Tag {
            unmark: vec![ByName("a".to_string())],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_mark_and_unmark_single_long() {
    expect_parses_into(
        "todo tag 1 --unmark 2",
        SubCommand::Tag(Tag {
            keys: vec![ByNumber(1)],
            unmark: vec![ByNumber(2)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_include_done_long() {
    expect_parses_into(
        "todo tag --include-done",
        SubCommand::Tag(Tag {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn tag_include_done_short() {
    expect_parses_into(
        "todo tag -d",
        SubCommand::Tag(Tag {
            include_done: true,
            ..Default::default()
        }),
    );
}

#[test]
fn tag_include_done_and_unmark_single_long() {
    expect_parses_into(
        "todo tag --include-done --unmark 1",
        SubCommand::Tag(Tag {
            include_done: true,
            unmark: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_include_done_and_unmark_single_short() {
    expect_parses_into(
        "todo tag -d -u 1",
        SubCommand::Tag(Tag {
            include_done: true,
            unmark: vec![ByNumber(1)],
            ..Default::default()
        }),
    );
}

#[test]
fn tag_include_done_and_unmark_multiple_long() {
    expect_parses_into(
        "todo tag --include-done --unmark 1 2 3",
        SubCommand::Tag(Tag {
            include_done: true,
            unmark: vec![ByNumber(1), ByNumber(2), ByNumber(3)],
            ..Default::default()
        }),
    );
}
