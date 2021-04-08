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
fn status_include_all() {
    let options = parse(&["todo", "-a"]);
    assert_eq!(options.cmd, None);
    assert!(options.include_all);
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
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
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
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
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
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
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
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
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
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
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
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
        }),
    );
}

#[test]
fn new_blocking_by_name() {
    let options = parse(&["todo", "new", "a.b", "-b", "b"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a.b".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByName("b".to_string())],
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
        })
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
            before: Vec::new(),
            after: Vec::new(),
            chain: true,
            priority: None,
        })
    );
}

#[test]
fn new_before_after() {
    let args = ["todo", "new", "c", "--before", "a", "--after", "b"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: vec![Key::ByName("a".to_string())],
            after: vec![Key::ByName("b".to_string())],
            chain: false,
            priority: None,
        })
    );
}

#[test]
fn new_one_with_priority() {
    let options = parse(&["todo", "new", "a", "--priority", "1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(1),
        })
    )
}

#[test]
fn new_three_with_priority() {
    let options = parse(&["todo", "new", "a", "b", "c", "--priority", "2"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(2),
        })
    )
}

#[test]
fn new_with_negative_priority() {
    let options = parse(&["todo", "new", "a", "--priority", "-3"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(-3),
        })
    )
}

#[test]
fn check_one() {
    let args = ["todo", "check", "1"];
    let options = parse(&args);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Check(Check {
            keys: vec![Key::ByNumber(1)],
            force: false,
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
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            force: false,
        })
    );
}

#[test]
fn check_by_name() {
    let options = parse(&["todo", "check", "a"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Check(Check {
            keys: vec![Key::ByName("a".to_string())],
            force: false,
        })
    )
}

#[test]
fn check_force() {
    let options = parse(&["todo", "check", "10", "--force"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Check(Check {
            keys: vec![Key::ByNumber(10)],
            force: true,
        })
    )
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
            keys: vec![Key::ByNumber(1)],
            force: false,
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
            force: false,
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
            force: false,
        })
    );
}

#[test]
fn restore_by_name() {
    let options = parse(&["todo", "restore", "b"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Restore(Restore {
            keys: vec![Key::ByName("b".to_string())],
            force: false,
        })
    );
}

#[test]
fn restore_force() {
    let options = parse(&["todo", "restore", "-10", "--force"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(-10)],
            force: true,
        })
    )
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
fn block_by_name() {
    let options = parse(&["todo", "block", "a", "--on", "b"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Block(Block {
            keys: vec![Key::ByName("a".to_string())],
            on: vec![Key::ByName("b".to_string())],
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
fn unblock_by_name() {
    let options = parse(&["todo", "unblock", "a", "--from", "b"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByName("a".to_string())],
            from: vec![Key::ByName("b".to_string())],
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
fn get_by_name() {
    let options = parse(&["todo", "get", "a"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Get(Get {
            keys: vec![Key::ByName("a".to_string())],
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

#[test]
fn punt_one() {
    let options = parse(&["todo", "punt", "1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Punt(Punt {
            keys: vec![Key::ByNumber(1)],
        })
    );
}

#[test]
fn punt_three() {
    let options = parse(&["todo", "punt", "1", "2", "3"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Punt(Punt {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
        })
    );
}

#[test]
fn punt_by_name() {
    let options = parse(&["todo", "punt", "a"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Punt(Punt {
            keys: vec![Key::ByName("a".to_string())],
        })
    )
}

#[test]
fn edit_with_description() {
    let options = parse(&["todo", "edit", "10", "--desc", "hello"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Edit(Edit {
            keys: vec![Key::ByNumber(10)],
            desc: Some("hello".to_string()),
        })
    );
}

#[test]
fn edit_without_description() {
    let options = parse(&["todo", "edit", "1", "2", "3"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Edit(Edit {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            desc: None,
        })
    );
}

#[test]
fn put_one_before() {
    let options = parse(&["todo", "put", "a", "--before", "b"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Put(Put {
            keys: vec![Key::ByName("a".to_string())],
            before: vec![Key::ByName("b".to_string())],
            after: vec![],
        })
    );
}

#[test]
fn put_one_after() {
    let options = parse(&["todo", "put", "a", "--after", "b"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Put(Put {
            keys: vec![Key::ByName("a".to_string())],
            before: vec![],
            after: vec![Key::ByName("b".to_string())],
        })
    );
}

#[test]
fn put_multiple_before_and_after() {
    let options = parse(&[
        "todo", "put", "a", "b", "c", "--before", "d", "e", "f", "--after",
        "g", "h", "i",
    ]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Put(Put {
            keys: vec![
                Key::ByName("a".to_string()),
                Key::ByName("b".to_string()),
                Key::ByName("c".to_string())
            ],
            before: vec![
                Key::ByName("d".to_string()),
                Key::ByName("e".to_string()),
                Key::ByName("f".to_string())
            ],
            after: vec![
                Key::ByName("g".to_string()),
                Key::ByName("h".to_string()),
                Key::ByName("i".to_string())
            ],
        })
    );
}

#[test]
fn find_with_single_string() {
    let options = parse(&["todo", "find", "hello"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Find(Find {
            terms: vec!["hello".to_string()],
        })
    );
}

#[test]
fn find_with_multiple_strings() {
    let options = parse(&["todo", "find", "hello", "goodbye"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Find(Find {
            terms: vec!["hello".to_string(), "goodbye".to_string()],
        })
    );
}

#[test]
fn chain_one() {
    let options = parse(&["todo", "chain", "1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Chain(Chain {
            keys: vec![Key::ByNumber(1)],
        })
    );
}

#[test]
fn chain_three() {
    let options = parse(&["todo", "chain", "10", "20", "30"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Chain(Chain {
            keys: vec![Key::ByNumber(10), Key::ByNumber(20), Key::ByNumber(30)],
        })
    );
}

#[test]
fn chain_by_range() {
    let options = parse(&["todo", "chain", "1..5"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Chain(Chain {
            keys: vec![Key::ByRange(1, 5)],
        })
    );
}

#[test]
fn path_by_number() {
    let options = parse(&["todo", "path", "10", "20"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Path(Path {
            from: Key::ByNumber(10),
            to: Key::ByNumber(20),
        })
    );
}

#[test]
fn path_by_name() {
    let options = parse(&["todo", "path", "a", "b"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Path(Path {
            from: Key::ByName("a".to_string()),
            to: Key::ByName("b".to_string()),
        })
    );
}

#[test]
fn path_missing_to() {
    Options::from_iter_safe(&["todo", "path", "a"]).unwrap_err();
}

#[test]
fn path_missing_from() {
    Options::from_iter_safe(&["todo", "path"]).unwrap_err();
}

#[test]
fn path_too_many_args() {
    Options::from_iter_safe(&["todo", "path", "1", "2", "3"]).unwrap_err();
}

#[test]
fn rm_by_number() {
    let options = parse(&["todo", "rm", "1", "2"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Rm(Rm {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2)],
        })
    );
}

#[test]
fn rm_by_name() {
    let options = parse(&["todo", "rm", "a", "b", "c"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Rm(Rm {
            keys: vec![
                Key::ByName("a".to_string()),
                Key::ByName("b".to_string()),
                Key::ByName("c".to_string()),
            ],
        })
    );
}

#[test]
fn top() {
    let options = parse(&["todo", "top"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Top(Top {
            keys: Vec::new(),
            include_done: false,
        })
    );
}

#[test]
fn top_include_done_long() {
    let options = parse(&["todo", "top", "--include-done"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Top(Top {
            keys: Vec::new(),
            include_done: true,
        })
    );
}

#[test]
fn top_include_done_short() {
    let options = parse(&["todo", "top", "-d"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Top(Top {
            keys: Vec::new(),
            include_done: true,
        })
    );
}

#[test]
fn priority_assign_to_one_task() {
    let options = parse(&["todo", "priority", "1", "--is", "2"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Priority(Priority {
            keys: vec![Key::ByNumber(1)],
            priority: 2,
        })
    );
}

#[test]
fn priority_assign_to_three_tasks() {
    let options = parse(&["todo", "priority", "1", "2", "3", "--is", "-1"]);
    let cmd = options.cmd.unwrap();
    assert_eq!(
        cmd,
        SubCommand::Priority(Priority {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            priority: -1,
        })
    );
}
