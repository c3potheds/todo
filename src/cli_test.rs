use cli::*;
use structopt::StructOpt;

fn parse<I>(args: I) -> Options
where
    I: IntoIterator,
    I::Item: Into<std::ffi::OsString> + Clone,
{
    Options::from_iter_safe(args).expect("Could not parse args")
}

fn expect_parses_into<'a, S: Into<&'a str>>(args: S, expected: SubCommand) {
    let s = args.into();
    let options = parse(s.split(" "));
    let cmd = options.cmd.unwrap();
    assert_eq!(cmd, expected);
}

fn expect_error<'a, S: Into<&'a str>>(args: S) {
    let s = args.into();
    Options::from_iter_safe(s.split(" "))
        .expect_err(&format!("Was not a parse error: '{}'", s));
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
    expect_parses_into(
        "todo new a",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_three() {
    expect_parses_into(
        "todo new a b c",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_blocked_by_long() {
    expect_parses_into(
        "todo new b --blocked-by 1",
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![Key::ByNumber(1)],
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_blocked_by_short() {
    expect_parses_into(
        "todo new b -p 1",
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: vec![Key::ByNumber(1)],
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_blocking_long() {
    expect_parses_into(
        "todo new b --blocking 1",
        SubCommand::New(New {
            desc: vec!["b".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByNumber(1)],
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_blocking_short() {
    expect_parses_into(
        "todo new c -b 1 2",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByNumber(1), Key::ByNumber(2)],
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_blocking_by_name() {
    expect_parses_into(
        "todo new a.b -b b",
        SubCommand::New(New {
            desc: vec!["a.b".to_string()],
            blocked_by: Vec::new(),
            blocking: vec![Key::ByName("b".to_string())],
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_chain() {
    expect_parses_into(
        "todo new a b c --chain",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: true,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_before_after() {
    expect_parses_into(
        "todo new c --before a --after b",
        SubCommand::New(New {
            desc: vec!["c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: vec![Key::ByName("a".to_string())],
            after: vec![Key::ByName("b".to_string())],
            chain: false,
            priority: None,
            due: vec![],
        }),
    );
}

#[test]
fn new_one_with_priority() {
    expect_parses_into(
        "todo new a --priority 1",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(1),
            due: vec![],
        }),
    )
}

#[test]
fn new_three_with_priority() {
    expect_parses_into(
        "todo new a b c --priority 2",
        SubCommand::New(New {
            desc: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(2),
            due: vec![],
        }),
    )
}

#[test]
fn new_with_negative_priority() {
    expect_parses_into(
        "todo new a --priority -3",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: Some(-3),
            due: vec![],
        }),
    )
}

#[test]
fn new_with_due_date() {
    expect_parses_into(
        "todo new a --due 7 days",
        SubCommand::New(New {
            desc: vec!["a".to_string()],
            blocked_by: Vec::new(),
            blocking: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            chain: false,
            priority: None,
            due: vec!["7".to_string(), "days".to_string()],
        }),
    )
}

#[test]
fn check_one() {
    expect_parses_into(
        "todo check 1",
        SubCommand::Check(Check {
            keys: vec![Key::ByNumber(1)],
            force: false,
        }),
    );
}

#[test]
fn check_three() {
    expect_parses_into(
        "todo check 1 2 3",
        SubCommand::Check(Check {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            force: false,
        }),
    );
}

#[test]
fn check_by_name() {
    expect_parses_into(
        "todo check a",
        SubCommand::Check(Check {
            keys: vec![Key::ByName("a".to_string())],
            force: false,
        }),
    )
}

#[test]
fn check_force() {
    expect_parses_into(
        "todo check 10 --force",
        SubCommand::Check(Check {
            keys: vec![Key::ByNumber(10)],
            force: true,
        }),
    )
}

#[test]
fn log() {
    expect_parses_into("todo log", SubCommand::Log);
}

#[test]
fn restore_one_task() {
    expect_parses_into(
        "todo restore 1",
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(1)],
            force: false,
        }),
    );
}

#[test]
fn restore_task_with_negative_number() {
    expect_parses_into(
        "todo restore -1",
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(-1)],
            force: false,
        }),
    );
}

#[test]
fn restore_multiple_tasks() {
    expect_parses_into(
        "todo restore 0 -1 -2",
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(0), Key::ByNumber(-1), Key::ByNumber(-2)],
            force: false,
        }),
    );
}

#[test]
fn restore_by_name() {
    expect_parses_into(
        "todo restore b",
        SubCommand::Restore(Restore {
            keys: vec![Key::ByName("b".to_string())],
            force: false,
        }),
    );
}

#[test]
fn restore_force() {
    expect_parses_into(
        "todo restore -10 --force",
        SubCommand::Restore(Restore {
            keys: vec![Key::ByNumber(-10)],
            force: true,
        }),
    )
}

#[test]
fn block_one_on_one() {
    expect_parses_into(
        "todo block 2 --on 1",
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(2)],
            on: vec![Key::ByNumber(1)],
            include_done: false,
        }),
    );
}

#[test]
fn block_three_on_one() {
    expect_parses_into(
        "todo block 1 2 3 --on 4",
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            on: vec![Key::ByNumber(4)],
            include_done: false,
        }),
    );
}

#[test]
fn block_three_on_three() {
    expect_parses_into(
        "todo block 1 2 3 --on 4 5 6",
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            on: vec![Key::ByNumber(4), Key::ByNumber(5), Key::ByNumber(6)],
            include_done: false,
        }),
    );
}

#[test]
fn block_by_name() {
    expect_parses_into(
        "todo block a --on b",
        SubCommand::Block(Block {
            keys: vec![Key::ByName("a".to_string())],
            on: vec![Key::ByName("b".to_string())],
            include_done: false,
        }),
    );
}

#[test]
fn block_include_done_long() {
    expect_parses_into(
        "todo block 1 --on 2 --include-done",
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(1)],
            on: vec![Key::ByNumber(2)],
            include_done: true,
        }),
    );
}

#[test]
fn block_include_done_short() {
    expect_parses_into(
        "todo block 1 --on 2 -d",
        SubCommand::Block(Block {
            keys: vec![Key::ByNumber(1)],
            on: vec![Key::ByNumber(2)],
            include_done: true,
        }),
    );
}

#[test]
fn unblock_one_from_one() {
    expect_parses_into(
        "todo unblock 2 --from 1",
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(2)],
            from: vec![Key::ByNumber(1)],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_three_from_one() {
    expect_parses_into(
        "todo unblock 2 3 4 --from 0",
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(2), Key::ByNumber(3), Key::ByNumber(4)],
            from: vec![Key::ByNumber(0)],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_three_from_three() {
    expect_parses_into(
        "todo unblock 4 5 6 --from 1 2 3",
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(4), Key::ByNumber(5), Key::ByNumber(6)],
            from: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_by_name() {
    expect_parses_into(
        "todo unblock a --from b",
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByName("a".to_string())],
            from: vec![Key::ByName("b".to_string())],
            include_done: false,
        }),
    );
}

#[test]
fn unblock_include_done_long() {
    expect_parses_into(
        "todo unblock 2 --from 1 --include-done",
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(2)],
            from: vec![Key::ByNumber(1)],
            include_done: true,
        }),
    );
}

#[test]
fn unblock_include_done_short() {
    expect_parses_into(
        "todo unblock 2 --from 1 -d",
        SubCommand::Unblock(Unblock {
            keys: vec![Key::ByNumber(2)],
            from: vec![Key::ByNumber(1)],
            include_done: true,
        }),
    );
}

#[test]
fn get_one() {
    expect_parses_into(
        "todo get 1",
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(1)],
            include_done: false,
        }),
    );
}

#[test]
fn get_three() {
    expect_parses_into(
        "todo get 1 2 3",
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            include_done: false,
        }),
    );
}

#[test]
fn get_by_name() {
    expect_parses_into(
        "todo get a",
        SubCommand::Get(Get {
            keys: vec![Key::ByName("a".to_string())],
            include_done: false,
        }),
    );
}

#[test]
fn get_negative() {
    expect_parses_into(
        "todo get -1",
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(-1)],
            include_done: false,
        }),
    );
}

#[test]
fn get_include_done_long() {
    expect_parses_into(
        "todo get 1 --include-done",
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(1)],
            include_done: true,
        }),
    );
}

#[test]
fn get_include_done_short() {
    expect_parses_into(
        "todo get 1 -d",
        SubCommand::Get(Get {
            keys: vec![Key::ByNumber(1)],
            include_done: true,
        }),
    );
}

#[test]
fn punt_one() {
    expect_parses_into(
        "todo punt 1",
        SubCommand::Punt(Punt {
            keys: vec![Key::ByNumber(1)],
        }),
    );
}

#[test]
fn punt_three() {
    expect_parses_into(
        "todo punt 1 2 3",
        SubCommand::Punt(Punt {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
        }),
    );
}

#[test]
fn punt_by_name() {
    expect_parses_into(
        "todo punt a",
        SubCommand::Punt(Punt {
            keys: vec![Key::ByName("a".to_string())],
        }),
    )
}

#[test]
fn edit_with_description() {
    expect_parses_into(
        "todo edit 10 --desc hello",
        SubCommand::Edit(Edit {
            keys: vec![Key::ByNumber(10)],
            desc: Some("hello".to_string()),
        }),
    );
}

#[test]
fn edit_without_description() {
    expect_parses_into(
        "todo edit 1 2 3",
        SubCommand::Edit(Edit {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            desc: None,
        }),
    );
}

#[test]
fn put_one_before() {
    expect_parses_into(
        "todo put a --before b",
        SubCommand::Put(Put {
            keys: vec![Key::ByName("a".to_string())],
            before: vec![Key::ByName("b".to_string())],
            after: vec![],
            include_done: false,
        }),
    );
}

#[test]
fn put_one_after() {
    expect_parses_into(
        "todo put a --after b",
        SubCommand::Put(Put {
            keys: vec![Key::ByName("a".to_string())],
            before: vec![],
            after: vec![Key::ByName("b".to_string())],
            include_done: false,
        }),
    );
}

#[test]
fn put_multiple_before_and_after() {
    expect_parses_into(
        "todo put a b c --before d e f --after g h i",
        SubCommand::Put(Put {
            keys: vec![
                Key::ByName("a".to_string()),
                Key::ByName("b".to_string()),
                Key::ByName("c".to_string()),
            ],
            before: vec![
                Key::ByName("d".to_string()),
                Key::ByName("e".to_string()),
                Key::ByName("f".to_string()),
            ],
            after: vec![
                Key::ByName("g".to_string()),
                Key::ByName("h".to_string()),
                Key::ByName("i".to_string()),
            ],
            include_done: false,
        }),
    );
}

#[test]
fn put_include_done_long() {
    expect_parses_into(
        "todo put a --before b --include-done",
        SubCommand::Put(Put {
            keys: vec![Key::ByName("a".to_string())],
            before: vec![Key::ByName("b".to_string())],
            after: vec![],
            include_done: true,
        }),
    );
}

#[test]
fn put_include_done_short() {
    expect_parses_into(
        "todo put a --before b -d",
        SubCommand::Put(Put {
            keys: vec![Key::ByName("a".to_string())],
            before: vec![Key::ByName("b".to_string())],
            after: vec![],
            include_done: true,
        }),
    );
}

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

#[test]
fn chain_one() {
    expect_parses_into(
        "todo chain 1",
        SubCommand::Chain(Chain {
            keys: vec![Key::ByNumber(1)],
            include_done: false,
        }),
    );
}

#[test]
fn chain_three() {
    expect_parses_into(
        "todo chain 10 20 30",
        SubCommand::Chain(Chain {
            keys: vec![Key::ByNumber(10), Key::ByNumber(20), Key::ByNumber(30)],
            include_done: false,
        }),
    );
}

#[test]
fn chain_by_range() {
    expect_parses_into(
        "todo chain 1..5",
        SubCommand::Chain(Chain {
            keys: vec![Key::ByRange(1, 5)],
            include_done: false,
        }),
    );
}

#[test]
fn path_by_number() {
    expect_parses_into(
        "todo path 10 20",
        SubCommand::Path(Path {
            from: Key::ByNumber(10),
            to: Key::ByNumber(20),
        }),
    );
}

#[test]
fn path_by_name() {
    expect_parses_into(
        "todo path a b",
        SubCommand::Path(Path {
            from: Key::ByName("a".to_string()),
            to: Key::ByName("b".to_string()),
        }),
    );
}

#[test]
fn path_missing_to() {
    expect_error("todo path a");
}

#[test]
fn path_missing_from() {
    expect_error("todo path");
}

#[test]
fn path_too_many_args() {
    expect_error("todo path 1 2 3");
}

#[test]
fn rm_by_number() {
    expect_parses_into(
        "todo rm 1 2",
        SubCommand::Rm(Rm {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2)],
        }),
    );
}

#[test]
fn rm_by_name() {
    expect_parses_into(
        "todo rm a b c",
        SubCommand::Rm(Rm {
            keys: vec![
                Key::ByName("a".to_string()),
                Key::ByName("b".to_string()),
                Key::ByName("c".to_string()),
            ],
        }),
    );
}

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

#[test]
fn priority_query_all() {
    expect_parses_into(
        "todo priority",
        SubCommand::Priority(Priority {
            keys: vec![],
            priority: None,
            include_done: false,
        }),
    );
}

#[test]
fn priority_query_task() {
    expect_parses_into(
        "todo priority 1",
        SubCommand::Priority(Priority {
            keys: vec![Key::ByNumber(1)],
            priority: None,
            include_done: false,
        }),
    );
}

#[test]
fn priority_query_priority() {
    expect_parses_into(
        "todo priority --is 1",
        SubCommand::Priority(Priority {
            keys: vec![],
            priority: Some(1),
            include_done: false,
        }),
    );
}

#[test]
fn priority_assign_to_one_task() {
    expect_parses_into(
        "todo priority 1 --is 2",
        SubCommand::Priority(Priority {
            keys: vec![Key::ByNumber(1)],
            priority: Some(2),
            include_done: false,
        }),
    );
}

#[test]
fn priority_assign_to_three_tasks() {
    expect_parses_into(
        "todo priority 1 2 3 --is -1",
        SubCommand::Priority(Priority {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            priority: Some(-1),
            include_done: false,
        }),
    );
}

#[test]
fn split_one_into_one() {
    expect_parses_into(
        "todo split 1 --into a",
        SubCommand::Split(Split {
            keys: vec![Key::ByNumber(1)],
            into: vec!["a".to_string()],
            chain: false,
        }),
    );
}

#[test]
fn split_one_into_three() {
    expect_parses_into(
        "todo split 1 --into a b c",
        SubCommand::Split(Split {
            keys: vec![Key::ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            chain: false,
        }),
    );
}

#[test]
fn split_three_into_two() {
    expect_parses_into(
        "todo split 1 2 3 --into a b",
        SubCommand::Split(Split {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
            into: vec!["a".to_string(), "b".to_string()],
            chain: false,
        }),
    );
}

#[test]
fn split_into_chain() {
    expect_parses_into(
        "todo split 1 --into a b c --chain",
        SubCommand::Split(Split {
            keys: vec![Key::ByNumber(1)],
            into: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            chain: true,
        }),
    );
}

#[test]
fn due_no_keys_no_date() {
    expect_parses_into(
        "todo due",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_with_keys_but_no_date() {
    expect_parses_into(
        "todo due 1",
        SubCommand::Due(Due {
            keys: vec![Key::ByNumber(1)],
            due: vec![],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_with_date_but_no_keys() {
    expect_parses_into(
        "todo due --in 2 days",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec!["2".to_string(), "days".to_string()],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_with_keys_and_date() {
    expect_parses_into(
        "todo due 10 --on friday",
        SubCommand::Due(Due {
            keys: vec![Key::ByNumber(10)],
            due: vec!["friday".to_string()],
            none: false,
            include_done: false,
        }),
    );
}

#[test]
fn due_set_none() {
    expect_parses_into(
        "todo due 1 2 --none",
        SubCommand::Due(Due {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2)],
            due: vec![],
            none: true,
            include_done: false,
        }),
    );
}

#[test]
fn due_get_none() {
    expect_parses_into(
        "todo due --none",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: true,
            include_done: false,
        }),
    );
}

#[test]
fn due_include_done_long() {
    expect_parses_into(
        "todo due --include-done",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: false,
            include_done: true,
        }),
    );
}

#[test]
fn due_include_done_short() {
    expect_parses_into(
        "todo due -d",
        SubCommand::Due(Due {
            keys: vec![],
            due: vec![],
            none: false,
            include_done: true,
        }),
    );
}

#[test]
fn merge_requires_at_least_two_and_into() {
    expect_error("todo merge");
    expect_error("todo merge 1");
    expect_error("todo merge 1 2");
    expect_error("todo merge --into aa");
    expect_error("todo merge 1 --into aa");
}

#[test]
fn merge_two() {
    expect_parses_into(
        "todo merge 1 2 --into ab",
        SubCommand::Merge(Merge {
            keys: vec![Key::ByNumber(1), Key::ByNumber(2)],
            into: "ab".to_string(),
        }),
    );
}

#[test]
fn merge_three() {
    expect_parses_into(
        "todo merge -1 -2 -3 --into abc",
        SubCommand::Merge(Merge {
            keys: vec![Key::ByNumber(-1), Key::ByNumber(-2), Key::ByNumber(-3)],
            into: "abc".to_string(),
        }),
    );
}
