use app::testing::*;
use model::TaskStatus;
use printing::Action;
use printing::Expect;

#[test]
fn rm_nonexistent_task() {
    let mut fix = Fixture::new();
    fix.test("todo rm a").validate().end();
}

#[test]
fn rm_only_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo rm a")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Removed),
            Expect::Action(Action::Delete),
        ])
        .end();
}

#[test]
fn rm_task_with_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo rm a")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Removed),
            Expect::Action(Action::Delete),
        ])
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn rm_task_with_deps_and_adeps() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo rm b")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Removed),
            Expect::Action(Action::Delete),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Blocked),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn rm_three_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e");
    fix.test("todo rm a c e")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Removed),
            Expect::Action(Action::Delete),
        ])
        .printed_task(&[
            Expect::Desc("c"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Removed),
            Expect::Action(Action::Delete),
        ])
        .printed_task(&[
            Expect::Desc("e"),
            Expect::Number(5),
            Expect::Status(TaskStatus::Removed),
            Expect::Action(Action::Delete),
        ])
        .end();
    fix.test("todo")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn rm_complete_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo rm a")
        .validate()
        .printed_task(&[
            Expect::Desc("a"),
            Expect::Number(0),
            Expect::Status(TaskStatus::Removed),
            Expect::Action(Action::Delete),
        ])
        .end();
}
