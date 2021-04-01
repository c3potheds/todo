use app::testing::Fixture;
use model::TaskStatus;
use printing::Action;
use printing::Expect;

#[test]
fn find_with_exact_match() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo find b")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn find_with_substring_match() {
    let mut fix = Fixture::new();
    fix.test("todo new aaa aba aca");
    fix.test("todo find b")
        .validate()
        .printed_task(&[
            Expect::Desc("aba"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn find_with_multiple_matches() {
    let mut fix = Fixture::new();
    fix.test("todo new aaa aba aca");
    fix.test("todo find a")
        .validate()
        .printed_task(&[
            Expect::Desc("aaa"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("aba"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("aca"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn find_includes_complete_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new aaa aba aca");
    fix.test("todo check 2");
    fix.test("todo find b")
        .validate()
        .printed_task(&[
            Expect::Desc("aba"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Complete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn find_includes_blocked_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new aaa aba aca --chain");
    fix.test("todo find b")
        .validate()
        .printed_task(&[
            Expect::Desc("aba"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn find_case_insensitive() {
    let mut fix = Fixture::new();
    fix.test("todo new AAA aaa");
    fix.test("todo find aa")
        .validate()
        .printed_task(&[
            Expect::Desc("AAA"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("aaa"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}
