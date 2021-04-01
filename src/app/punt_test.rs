use app::testing::Fixture;
use model::TaskStatus;
use printing::Action;
use printing::Expect;

#[test]
fn punt_first_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo punt 1")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Punt),
        ])
        .end();
}

#[test]
fn punt_blocked_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo new b c -p 1");
    fix.test("todo punt 2")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::Punt),
        ])
        .end();
}

#[test]
fn punt_by_name() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo punt a")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::Punt),
        ])
        .end();
}
