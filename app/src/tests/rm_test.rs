use {
    super::testing::task,
    super::testing::Fixture,
    todo_printing::{PrintableInfo, Status::*},
};

fn info_removed(desc: &str) -> PrintableInfo {
    PrintableInfo::Removed {
        desc: desc.to_string(),
    }
}

#[test]
fn rm_nonexistent_task() {
    let mut fix = Fixture::default();
    fix.test("todo rm a").modified(false).validate().end();
}

#[test]
fn rm_only_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo rm a")
        .modified(true)
        .validate()
        .printed_info(&info_removed("a"))
        .end();
}

#[test]
fn rm_task_with_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo rm a")
        .modified(true)
        .validate()
        .printed_info(&info_removed("a"))
        .printed_task(&task("b", 1, Incomplete))
        .end();
}

#[test]
fn rm_task_with_deps_and_adeps() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo rm b")
        .modified(true)
        .validate()
        .printed_info(&info_removed("b"))
        .printed_task(&task("a", 1, Incomplete).adeps_stats(1, 1))
        .printed_task(&task("c", 2, Blocked).deps_stats(1, 1))
        .end();
}

#[test]
fn rm_three_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c d e");
    fix.test("todo rm a c e")
        .modified(true)
        .validate()
        .printed_info(&info_removed("a"))
        .printed_info(&info_removed("c"))
        .printed_info(&info_removed("e"))
        .end();
    fix.test("todo")
        .modified(false)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .printed_task(&task("d", 2, Incomplete))
        .end();
}

#[test]
fn rm_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo rm a")
        .modified(true)
        .validate()
        .printed_info(&info_removed("a"))
        .end();
}
