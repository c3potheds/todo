#[derive(Debug, PartialEq)]
pub enum Cmd<'a> {
    Check { keys: Vec<&'a str> },
    New { desc: Vec<&'a str> },
    Status {},
}

pub fn clap_app<'a, 'b>() -> clap::App<'a, 'b>
where
    'a: 'b,
{
    clap::App::new("todo")
        .version("0.1")
        .author("Simeon Anfinrud")
        .about("Maintains and manipulates your to-do list.")
        .subcommand(
            clap::SubCommand::with_name("new")
                .about("Creates new tasks in the to-do list.")
                .arg(clap::Arg::with_name("desc").multiple(true).min_values(1)),
        )
        .subcommand(
            clap::SubCommand::with_name("check")
                .about("Marks tasks as complete.")
                .arg(clap::Arg::with_name("keys").multiple(true).min_values(1)),
        )
}

pub fn parse_args<'a>(matches: &'a clap::ArgMatches<'a>) -> Cmd<'a> {
    match matches.subcommand() {
        ("new", Some(new)) => {
            let desc = new
                .values_of("desc")
                .map(|c| c.collect())
                .unwrap_or_else(|| Vec::new());
            Cmd::New { desc: desc }
        }
        ("check", Some(check)) => {
            let keys = check
                .values_of("keys")
                .map(|c| c.collect())
                .unwrap_or_else(|| Vec::new());
            Cmd::Check { keys: keys }
        }
        _ => Cmd::Status {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_defaults_to_status() {
        let app = clap_app();
        let args = ["todo"];
        let matches = app.get_matches_from_safe(&args).unwrap();
        let opts = parse_args(&matches);
        assert_eq!(opts, Cmd::Status {});
    }

    #[test]
    fn new_one() {
        let app = clap_app();
        let args = ["rodo", "new", "abc"];
        let matches = app.get_matches_from_safe(&args).unwrap();
        let opts = parse_args(&matches);
        assert_eq!(opts, Cmd::New { desc: vec!["abc"] });
    }

    #[test]
    fn new_three() {
        let app = clap_app();
        let args = ["todo", "new", "a", "b", "c"];
        let matches = app.get_matches_from_safe(&args).unwrap();
        let opts = parse_args(&matches);
        assert_eq!(
            opts,
            Cmd::New {
                desc: vec!["a", "b", "c"]
            }
        );
    }

    #[test]
    fn check_one() {
        let app = clap_app();
        let args = ["todo", "check", "1"];
        let matches = app.get_matches_from_safe(&args).unwrap();
        let opts = parse_args(&matches);
        assert_eq!(opts, Cmd::Check { keys: vec!["1"] });
    }

    #[test]
    fn check_three() {
        let app = clap_app();
        let args = ["todo", "check", "1", "2", "3"];
        let matches = app.get_matches_from_safe(&args).unwrap();
        let opts = parse_args(&matches);
        assert_eq!(
            opts,
            Cmd::Check {
                keys: vec!["1", "2", "3"]
            }
        );
    }
}
