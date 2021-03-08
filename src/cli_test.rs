use cli::*;
use structopt::StructOpt;

fn parse<I>(args: I) -> Options
where
    I: IntoIterator,
    I::Item: Into<std::ffi::OsString> + Clone,
{
    Options::from_iter_safe(args).expect("Could not parse args")
}

#[test]
fn empty_defaults_to_status() {
    let options = parse(&["todo"]);
    assert_eq!(options.cmd, None);
}

#[test]
fn status_include_blocked() {
    let options = parse(&["todo", "-b"]);
    assert_eq!(options.cmd, None);
    assert!(options.include_blocked);
    assert!(!options.include_done);
}

#[test]
fn status_include_done() {
    let options = parse(&["todo", "-d"]);
    assert_eq!(options.cmd, None);
    assert!(!options.include_blocked);
    assert!(options.include_done);
}

#[test]
fn new_one() {
    let options = parse(&["todo", "new", "a"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            chain: false,
        })
    );
}

#[test]
fn new_three() {
    let args = ["todo", "new", "a", "b", "c"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            chain: false,
        })
    );
}

#[test]
fn new_blocked_by_long() {
    let args = ["todo", "new", "b", "--blocked-by", "1"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![Key::ByNumber(1)],
            blocking: Vec::new(),
            chain: false,
        }),
    );
}

#[test]
fn new_blocked_by_short() {
    let args = ["todo", "new", "b", "-p", "1"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![Key::ByNumber(1)],
            blocking: Vec::new(),
            chain: false,
        }),
    );
}

#[test]
fn new_blocking_long() {
    let args = ["todo", "new", "b", "--blocking", "1"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByNumber(1)],
            chain: false,
        }),
    );
}

#[test]
fn new_blocking_short() {
    let args = ["todo", "new", "c", "-b", "1", "2"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByNumber(1), Key::ByNumber(2)],
            chain: false,
        }),
    );
}

#[test]
fn new_chain() {
    let args = ["todo", "new", "a", "b", "c", "--chain"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            chain: true,
        })
    );
}

#[test]
fn check_one() {
    let args = ["todo", "check", "1"];
    let options = parse(&args);
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
    let options = parse(&args);
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
    let options = parse(&["todo", "log"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(cmd, SubCommand::Log);
}

#[test]
fn restore_one_task() {
    let options = parse(&["todo", "restore", "1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(1)]
        })
    );
}

#[test]
fn restore_task_with_negative_number() {
    let options = parse(&["todo", "restore", "-1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(-1)],
        })
    );
}

#[test]
fn restore_multiple_tasks() {
    let options = parse(&["todo", "restore", "0", "-1", "-2"]);
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
    let options = parse(&["todo", "block", "2", "--on", "1"]);
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
    let options = parse(&["todo", "block", "1", "2", "3", "--on", "4"]);
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
    let options =
        parse(&["todo", "block", "1", "2", "3", "--on", "4", "5", "6"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            on: vec![Key::ByNumber(4), Key::ByNumber(5), Key::ByNumber(6)],
        })
    );
}

#[test]
fn unblock_one_from_one() {
    let options = parse(&["todo", "unblock", "2", "--from", "1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(2)],
            from: vec![Key::ByNumber(1)],
        })
    );
}

#[test]
fn unblock_three_from_one() {
    let options = parse(&["todo", "unblock", "2", "3", "4", "--from", "0"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(2), Key::ByNumber(3), Key::ByNumber(4)],
            from: vec![Key::ByNumber(0)],
        })
    );
}

#[test]
fn unblock_three_from_three() {
    let options =
        parse(&["todo", "unblock", "4", "5", "6", "--from", "1", "2", "3"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(4), Key::ByNumber(5), Key::ByNumber(6)],
            from: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
        })
    );
}

#[test]
fn get_one() {
    let options = parse(&["todo", "get", "1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(1)],
        })
    );
}

#[test]
fn get_three() {
    let options = parse(&["todo", "get", "1", "2", "3"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
        })
    );
}

#[test]
fn get_negative() {
    let options = parse(&["todo", "get", "-1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(-1)],
        })
    );
}
