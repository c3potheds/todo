use app::testing::Fixture;
use cli::Key;
use model::TaskStatus;
use printing::Action;
use printing::Expect;
use printing::PrintableError;
use printing::PrintableWarning;

#[test]
fn path_between_tasks_with_no_path() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo path a b").validate().end();
}

#[test]
fn path_between_tasks_with_direct_dependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo path a b")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Select),
        ])
        .end();
}

#[test]
fn path_between_tasks_with_indirect_dependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo path a c")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Select),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Select),
        ])
        .end();
}

#[test]
fn key_must_not_be_ambiguous() {
    let mut fix = Fixture::new();
    fix.test("todo new a a b b");
    fix.test("todo path a b")
        .validate()
        .printed_error(&PrintableError::AmbiguousKey {
            key: Key::ByName("a".to_string()),
            matches: vec![1, 2],
        })
        .printed_error(&PrintableError::AmbiguousKey {
            key: Key::ByName("b".to_string()),
            matches: vec![3, 4],
        })
        .end();
}

#[test]
fn key_must_match_a_task() {
    let mut fix = Fixture::new();
    fix.test("todo path a b")
        .validate()
        .printed_warning(&PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByName("a".to_string()),
        })
        .printed_warning(&PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByName("b".to_string()),
        })
        .end();
}
