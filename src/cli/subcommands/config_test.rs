use cli::testing::expect_parses_into;
use cli::Config;
use cli::SubCommand;

#[test]
fn just_show_config() {
    expect_parses_into(
        "todo config",
        SubCommand::Config(Config {
            key: None,
            value: vec![],
            reset: false,
        }),
    );
}

#[test]
fn show_value_for_key() {
    expect_parses_into(
        "todo config pager",
        SubCommand::Config(Config {
            key: Some("pager".to_string()),
            value: vec![],
            reset: false,
        }),
    );
}

#[test]
fn set_value_for_key() {
    expect_parses_into(
        "todo config pager more",
        SubCommand::Config(Config {
            key: Some("pager".to_string()),
            value: vec!["more".to_string()],
            reset: false,
        }),
    );
}

#[test]
fn reset_value_for_key() {
    expect_parses_into(
        "todo config pager --reset",
        SubCommand::Config(Config {
            key: Some("pager".to_string()),
            value: vec![],
            reset: true,
        }),
    );
}
