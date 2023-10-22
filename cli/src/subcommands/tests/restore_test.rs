use {
    crate::{
        testing::{expect_error, expect_parses_into},
        Restore, SubCommand,
    },
    todo_lookup_key::Key::*,
};

#[test]
fn restore_no_keys() {
    expect_error("todo restore");
}

#[test]
fn restore_one_task() {
    expect_parses_into(
        "todo restore 1",
        SubCommand::Restore(Restore {
            keys: vec![ByNumber(1)],
            force: false,
        }),
    );
}

#[test]
fn restore_task_with_negative_number() {
    expect_parses_into(
        "todo restore -1",
        SubCommand::Restore(Restore {
            keys: vec![ByNumber(-1)],
            force: false,
        }),
    );
}

#[test]
fn restore_multiple_tasks() {
    expect_parses_into(
        "todo restore 0 -1 -2",
        SubCommand::Restore(Restore {
            keys: vec![ByNumber(0), ByNumber(-1), ByNumber(-2)],
            force: false,
        }),
    );
}

#[test]
fn restore_by_name() {
    expect_parses_into(
        "todo restore b",
        SubCommand::Restore(Restore {
            keys: vec![ByName("b".to_string())],
            force: false,
        }),
    );
}

#[test]
fn restore_force() {
    expect_parses_into(
        "todo restore -10 --force",
        SubCommand::Restore(Restore {
            keys: vec![ByNumber(-10)],
            force: true,
        }),
    )
}
