use {
    super::testing::task,
    super::testing::Fixture,
    todo_lookup_key::Key,
    todo_printing::{
        Action::*, BriefPrintableTask, PrintableWarning, Status::*,
    },
};

#[test]
fn path_between_tasks_with_no_path() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo path a b")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::NoPathFoundBetween(
            BriefPrintableTask::new(1, Incomplete),
            BriefPrintableTask::new(2, Incomplete),
        ))
        .end();
}

#[test]
fn path_between_tasks_with_direct_dependency() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo path a b")
        .modified(false)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).action(Select).adeps_stats(1, 1),
        )
        .printed_task(&task("b", 2, Blocked).action(Select).deps_stats(1, 1))
        .end();
}

#[test]
fn path_between_tasks_with_indirect_dependency() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo path a c")
        .modified(false)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).action(Select).adeps_stats(1, 2),
        )
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).action(Select).deps_stats(1, 2))
        .end();
}

#[test]
fn warn_if_key_is_ambiguous() {
    let mut fix = Fixture::default();
    fix.test("todo new a a b b");
    fix.test("todo path a b")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::AmbiguousKey {
            key: Key::ByName("a".to_string()),
            matches: vec![
                BriefPrintableTask::new(1, Incomplete),
                BriefPrintableTask::new(2, Incomplete),
            ],
        })
        .printed_warning(&PrintableWarning::AmbiguousKey {
            key: Key::ByName("b".to_string()),
            matches: vec![
                BriefPrintableTask::new(3, Incomplete),
                BriefPrintableTask::new(4, Incomplete),
            ],
        })
        .printed_warning(&PrintableWarning::NoPathFoundBetween(
            BriefPrintableTask::new(1, Incomplete),
            BriefPrintableTask::new(2, Incomplete),
        ))
        .end();
}

#[test]
fn warn_if_key_has_no_match() {
    let mut fix = Fixture::default();
    fix.test("todo path a b")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByName("a".to_string()),
        })
        .printed_warning(&PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByName("b".to_string()),
        })
        .end();
}

#[test]
fn path_between_task_with_one_match() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo path a")
        .modified(false)
        .validate()
        .printed_task(&task("a", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn path_between_tasks_of_same_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b a --chain");
    fix.test("todo path a")
        .modified(false)
        .validate()
        .printed_warning(&PrintableWarning::AmbiguousKey {
            key: Key::ByName("a".to_string()),
            matches: vec![
                BriefPrintableTask::new(1, Incomplete),
                BriefPrintableTask::new(3, Blocked),
            ],
        })
        .printed_task(
            &task("a", 1, Incomplete).action(Select).adeps_stats(1, 2),
        )
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("a", 3, Blocked).action(Select).deps_stats(1, 2))
        .end();
}

#[test]
fn path_between_three_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e --chain");
    fix.test("todo path a c e")
        .modified(false)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).action(Select).adeps_stats(1, 4),
        )
        .printed_task(&task("b", 2, Blocked).deps_stats(1, 1))
        .printed_task(&task("c", 3, Blocked).action(Select).deps_stats(1, 2))
        .printed_task(&task("d", 4, Blocked).deps_stats(1, 3))
        .printed_task(&task("e", 5, Blocked).action(Select).deps_stats(1, 4))
        .end();
}
