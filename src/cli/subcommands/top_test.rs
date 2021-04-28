use cli::testing::expect_parses_into;
use cli::SubCommand;
use cli::Top;

#[test]
fn top() {
    expect_parses_into(
        "todo top",
        SubCommand::Top(Top {
            keys: Vec::new(),
            include_done: false,
        }),
    );
}

#[test]
fn top_include_done_long() {
    expect_parses_into(
        "todo top --include-done",
        SubCommand::Top(Top {
            keys: Vec::new(),
            include_done: true,
        }),
    );
}

#[test]
fn top_include_done_short() {
    expect_parses_into(
        "todo top -d",
        SubCommand::Top(Top {
            keys: Vec::new(),
            include_done: true,
        }),
    );
}
