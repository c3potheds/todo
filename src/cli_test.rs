use cli::*;
use structopt::StructOpt;

#[test]
fn empty_defaults_to_status() {
    let options = Options::from_iter_safe(&["todo"]).unwrap();
    assert_eq!(options.cmd, None);
}

#[test]
fn status_include_blocked() {
    let options = Options::from_iter_safe(&["todo", "-b"]).unwrap();
    assert_eq!(options.cmd, None);
    assert!(options.include_blocked);
}

#[test]
fn new_one() {
    let options = Options::from_iter_safe(&["todo", "new", "a"]).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
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
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
        })
    );
}

#[test]
fn new_blocked_by_long() {
    let args = ["todo", "new", "b", "--blocked-by", "1"];
    let options = Options::from_iter_safe(&args).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![Key::ByNumber(1)],
            blocking: Vec::new(),
        }),
    );
}

#[test]
fn new_blocked_by_short() {
    let args = ["todo", "new", "b", "-p", "1"];
    let options = Options::from_iter_safe(&args).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![Key::ByNumber(1)],
            blocking: Vec::new(),
        }),
    );
}

#[test]
fn new_blocking_long() {
    let args = ["todo", "new", "b", "--blocking", "1"];
    let options = Options::from_iter_safe(&args).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByNumber(1)],
        }),
    );
}

#[test]
fn new_blocking_short() {
    let args = ["todo", "new", "c", "-b", "1", "2"];
    let options = Options::from_iter_safe(&args).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByNumber(1), Key::ByNumber(2)],
        }),
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

#[test]
fn block_one_on_one() {
    let options =
        Options::from_iter_safe(&["todo", "block", "2", "--on", "1"]).unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(2)],
            on: vec![Key::ByNumber(1)],
        })
    );
}

#[test]
fn block_three_on_one() {
    let options =
        Options::from_iter_safe(&["todo", "block", "1", "2", "3", "--on", "4"])
            .unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            on: vec![Key::ByNumber(4)],
        })
    );
}

#[test]
fn block_three_on_three() {
    let options = Options::from_iter_safe(&[
        "todo", "block", "1", "2", "3", "--on", "4", "5", "6",
    ])
    .unwrap();
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            on: vec![Key::ByNumber(4), Key::ByNumber(5), Key::ByNumber(6)],
        })
    );
}
