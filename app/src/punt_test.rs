use {
    super::testing::Fixture,
    printing::{Action::*, PrintableTask, Status::*},
};

#[test]
fn punt_first_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo punt 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Incomplete).action(Punt))
        .end();
}

#[test]
fn punt_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b c -p 1");
    fix.test("todo punt 2")
        .validate()
        .printed_task(&PrintableTask::new("b", 3, Blocked).action(Punt))
        .end();
}

#[test]
fn punt_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo punt a")
        .validate()
        .printed_task(&PrintableTask::new("a", 3, Incomplete).action(Punt))
        .end();
}
