use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
struct New {
    desc: Vec<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
struct Check {
    keys: Vec<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
enum SubCommand {
    /// Marks tasks as complete.
    Check(Check),
    /// Creates new tasks in the to-do list.
    New(New),
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo",
    about = "Maintains and manipulates your to-do list.",
    author = "Simeon Anfinrud",
    version = "0.1"
)]
struct Options {
    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_defaults_to_status() {
        let options = Options::from_iter_safe(&["todo"]).unwrap();
        assert_eq!(options.cmd, None);
    }

    #[test]
    fn new_one() {
        let options = Options::from_iter_safe(&["todo", "new", "a"]).unwrap();
        let cmd = options.cmd.unwrap();
        assert_eq!(
            cmd,
            SubCommand::New(New {
                desc: vec!["a".to_string()]
            })
        );
    }

    #[test]
    fn new_three() {
        let args = ["todo", "new", "a", "b", "c"];
        let options = Options::from_iter_safe(&args).unwrap();
        let cmd = options.cmd.unwrap();
        assert_eq!(
            cmd,
            SubCommand::New(New {
                desc: vec!["a".to_string(), "b".to_string(), "c".to_string()]
            })
        );
    }

    #[test]
    fn check_one() {
        let args = ["todo", "check", "1"];
        let options = Options::from_iter_safe(&args).unwrap();
        let cmd = options.cmd.unwrap();
        assert_eq!(
            cmd,
            SubCommand::Check(Check {
                keys: vec!["1".to_string()]
            })
        );
    }

    #[test]
    fn check_three() {
        let args = ["todo", "check", "1", "2", "3"];
        let options = Options::from_iter_safe(&args).unwrap();
        let cmd = options.cmd.unwrap();
        assert_eq!(
            cmd,
            SubCommand::Check(Check {
                keys: vec!["1".to_string(), "2".to_string(), "3".to_string()]
            })
        );
    }
}
