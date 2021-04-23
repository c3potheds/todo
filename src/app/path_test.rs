use app::testing::Fixture;
use cli::Key;
use printing::Action::*;
use printing::BriefPrintableTask;
use printing::PrintableTask;
use printing::PrintableWarning;
use printing::Status::*;

#[test]
fn path_between_tasks_with_no_path() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo path a b")
        .validate()
        .printed_warning(&PrintableWarning::NoPathFoundBetween(
            BriefPrintableTask::new(1, Incomplete),
            BriefPrintableTask::new(2, Incomplete),
        ))
        .end();
}

#[test]
fn path_between_tasks_with_direct_dependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b --chain");
    fix.test("todo path a b")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("b", 2, Blocked).action(Select))
        .end();
}

#[test]
fn path_between_tasks_with_indirect_dependency() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo path a c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(Select))
        .end();
}

#[test]
fn warn_if_key_is_ambiguous() {
    let mut fix = Fixture::new();
    fix.test("todo new a a b b");
    fix.test("todo path a b")
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

#[test]
fn path_between_task_with_one_match() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo path a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .end();
}

#[test]
fn path_between_tasks_of_same_name() {
    let mut fix = Fixture::new();
    fix.test("todo new a b a --chain");
    fix.test("todo path a")
        .validate()
        .printed_warning(&PrintableWarning::AmbiguousKey {
            key: Key::ByName("a".to_string()),
            matches: vec![
                BriefPrintableTask::new(1, Incomplete),
                BriefPrintableTask::new(3, Blocked),
            ],
        })
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("a", 3, Blocked).action(Select))
        .end();
}

#[test]
fn path_between_three_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e --chain");
    fix.test("todo path a c e")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .printed_task(&PrintableTask::new("b", 2, Blocked))
        .printed_task(&PrintableTask::new("c", 3, Blocked).action(Select))
        .printed_task(&PrintableTask::new("d", 4, Blocked))
        .printed_task(&PrintableTask::new("e", 5, Blocked).action(Select))
        .end();
}
