use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommand {
    /// Marks tasks as complete.
    Check { keys: Vec<String> },
    /// Creates new tasks in the to-do list.
    New { desc: Vec<String> },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo",
    about = "Maintains and manipulates your to-do list.",
    author = "Simeon Anfinrud",
    version = "0.1"
)]
pub struct Options {
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
            SubCommand::New {
                desc: vec!["a".to_string()]
            }
        );
    }

    #[test]
    fn new_three() {
        let args = ["todo", "new", "a", "b", "c"];
        let options = Options::from_iter_safe(&args).unwrap();
        let cmd = options.cmd.unwrap();
        assert_eq!(
            cmd,
            SubCommand::New {
                desc: vec!["a".to_string(), "b".to_string(), "c".to_string()]
            }
        );
    }

    #[test]
    fn check_one() {
        let args = ["todo", "check", "1"];
        let options = Options::from_iter_safe(&args).unwrap();
        let cmd = options.cmd.unwrap();
        assert_eq!(
            cmd,
            SubCommand::Check {
                keys: vec!["1".to_string()]
            }
        );
    }

    #[test]
    fn check_three() {
        let args = ["todo", "check", "1", "2", "3"];
        let options = Options::from_iter_safe(&args).unwrap();
        let cmd = options.cmd.unwrap();
        assert_eq!(
            cmd,
            SubCommand::Check {
                keys: vec!["1".to_string(), "2".to_string(), "3".to_string()]
            }
        );
    }
}
