use {
    super::testing::task,
    super::testing::Fixture,
    printing::{Action::*, Status::*},
};

#[test]
fn punt_first_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo punt 1")
        .modified(true)
        .validate()
        .printed_task(&task("a", 3, Incomplete).action(Punt))
        .end();
}

#[test]
fn punt_blocked_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo new b c -p 1");
    fix.test("todo punt 2")
        // TODO: Since the position of the task doesn't change, we don't
        // need to mark the session as modified.
        // .modified(false)
        .validate()
        .printed_task(&task("b", 3, Blocked).action(Punt).deps_stats(1, 1))
        .end();
}

#[test]
fn punt_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo punt a")
        .modified(true)
        .validate()
        .printed_task(&task("a", 3, Incomplete).action(Punt))
        .end();
}
