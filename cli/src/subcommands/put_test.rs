use {
    crate::{
        subcommands::put::Prepositions,
        testing::{expect_error, expect_parses_into},
        Put, SubCommand,
    },
    lookup_key::Key::*,
};

#[test]
fn punt_missing_keys_or_prepositions() {
    expect_error("todo put");
    // TODO(cli.put.preposition-arggroup): Require either --before or --after.
    // Blocked on representing a clap::ArgGroup in a clap Parser.
    // expect_error("todo put a");
    expect_error("todo put a --after");
    expect_error("todo put a --before");
    expect_error("todo put --after a");
    expect_error("todo put --before a");
}

#[test]
fn put_one_before() {
    expect_parses_into(
        "todo put a --before b",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                before: vec![ByName("b".to_string())],
                ..Default::default()
            },
            include_done: false,
        }),
    );
}

#[test]
fn put_one_before_short() {
    expect_parses_into(
        "todo put a -B b",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                before: vec![ByName("b".to_string())],
                ..Default::default()
            },
            include_done: false,
        }),
    );
}

#[test]
fn put_one_after() {
    expect_parses_into(
        "todo put a --after b",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                after: vec![ByName("b".to_string())],
                ..Default::default()
            },
            include_done: false,
        }),
    );
}

#[test]
fn put_one_after_short() {
    expect_parses_into(
        "todo put a -A b",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                after: vec![ByName("b".to_string())],
                ..Default::default()
            },
            include_done: false,
        }),
    );
}

#[test]
fn put_multiple_before_and_after() {
    expect_parses_into(
        "todo put a b c --before d e f --after g h i",
        SubCommand::Put(Put {
            keys: vec![
                ByName("a".to_string()),
                ByName("b".to_string()),
                ByName("c".to_string()),
            ],
            preposition: Prepositions {
                before: vec![
                    ByName("d".to_string()),
                    ByName("e".to_string()),
                    ByName("f".to_string()),
                ],
                after: vec![
                    ByName("g".to_string()),
                    ByName("h".to_string()),
                    ByName("i".to_string()),
                ],
                ..Default::default()
            },
            include_done: false,
        }),
    );
}

#[test]
fn put_include_done_long() {
    expect_parses_into(
        "todo put a --before b --include-done",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                before: vec![ByName("b".to_string())],
                ..Default::default()
            },
            include_done: true,
        }),
    );
}

#[test]
fn put_include_done_short() {
    expect_parses_into(
        "todo put a --before b -d",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                before: vec![ByName("b".to_string())],
                ..Default::default()
            },
            include_done: true,
        }),
    );
}

#[test]
fn put_by_long() {
    expect_parses_into(
        "todo put a --by b c",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                by: vec![ByName("b".to_string()), ByName("c".to_string())],
                ..Default::default()
            },
            include_done: false,
        }),
    );
}

#[test]
fn put_by_short() {
    expect_parses_into(
        "todo put a -Y b c",
        SubCommand::Put(Put {
            keys: vec![ByName("a".to_string())],
            preposition: Prepositions {
                by: vec![ByName("b".to_string()), ByName("c".to_string())],
                ..Default::default()
            },
            include_done: false,
        }),
    );
}
