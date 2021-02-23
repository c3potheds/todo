use cli::*;
use structopt::StructOpt;

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
            keys: vec![Key::ByNumber(1)]
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
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)]
        })
    );
}

#[test]
fn log() {
    let options = Options::from_iter_safe(&["todo", "log"]).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(cmd, SubCommand::Log);
}

#[test]
fn restore_one_task() {
    let options = Options::from_iter_safe(&["todo", "restore", "1"]).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(1)]
        })
    );
}

#[test]
#[ignore = "Figure out how to parse negative numbers."]
fn restore_task_with_negative_number() {
    let options = Options::from_iter_safe(&["todo", "restore", "-1"]).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(-1)],
        })
    );
}

#[test]
#[ignore = "Figure out how to parse negative numbers."]
fn restore_multiple_tasks() {
    let options =
        Options::from_iter_safe(&["todo", "restore", "0", "-1", "-2"]).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(0), Key::ByNumber(-1), Key::ByNumber(-2)],
        })
    );
}
