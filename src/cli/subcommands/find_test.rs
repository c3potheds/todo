use cli::testing::expect_parses_into;
use cli::Find;
use cli::SubCommand;

#[test]
fn find_with_single_string() {
    expect_parses_into(
        "todo find hello",
        SubCommand::Find(Find {
            terms: vec!["hello".to_string()],
            include_done: false,
        }),
    );
}

#[test]
fn find_include_done_long() {
    expect_parses_into(
        "todo find yo --include-done",
        SubCommand::Find(Find {
            terms: vec!["yo".to_string()],
            include_done: true,
        }),
    );
}

#[test]
fn find_include_done_short() {
    expect_parses_into(
        "todo find blah -d",
        SubCommand::Find(Find {
            terms: vec!["blah".to_string()],
            include_done: true,
        }),
    );
}

#[test]
fn find_with_multiple_strings() {
    expect_parses_into(
        "todo find hello goodbye",
        SubCommand::Find(Find {
            terms: vec!["hello".to_string(), "goodbye".to_string()],
            include_done: false,
        }),
    );
}
