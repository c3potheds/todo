use app::testing::Fixture;
use model::TaskStatus::*;
use printing::PrintableTask;

#[test]
fn top_empty() {
    let mut fix = Fixture::new();
    fix.test("todo top").validate().end();
}

#[test]
fn top_all_tasks_uncategorized() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo top")
        .validate()
        .printed_exact_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_exact_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_exact_task(&PrintableTask::new("c", 3, Incomplete))
        .end();
}

#[test]
fn top_all_tasks_categorized_the_same() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo new d -p a b c");
    fix.test("todo top")
        .validate()
        .printed_exact_task(&PrintableTask::new("d", 4, Blocked))
        .end();
}

#[test]
fn top_multiple_categories() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c d e f");
    fix.test("todo new g -p a b c");
    fix.test("todo new h -p d e f");
    fix.test("todo top")
        .validate()
        .printed_exact_task(&PrintableTask::new("g", 7, Blocked))
        .printed_exact_task(&PrintableTask::new("h", 8, Blocked))
        .end();
}

#[test]
fn top_deep_category() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo top")
        .validate()
        .printed_exact_task(&PrintableTask::new("c", 5, Blocked))
        .printed_exact_task(&PrintableTask::new("f", 6, Blocked))
        .end();
}

#[test]
fn top_does_not_show_complete_tasks_by_default() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo top")
        .validate()
        .printed_exact_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_exact_task(&PrintableTask::new("c", 2, Incomplete))
        .end();
}

#[test]
fn top_show_complete_tasks_with_option() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo check a");
    fix.test("todo top -d")
        .validate()
        .printed_exact_task(&PrintableTask::new("a", 0, Complete))
        .printed_exact_task(&PrintableTask::new("b", 1, Incomplete))
        .printed_exact_task(&PrintableTask::new("c", 2, Incomplete))
        .end();
}

#[test]
fn top_show_only_top_level_complete_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c --chain");
    fix.test("todo new d e f --chain");
    fix.test("todo check a b c d e f");
    fix.test("todo top -d")
        .validate()
        .printed_exact_task(&PrintableTask::new("c", -3, Complete))
        .printed_exact_task(&PrintableTask::new("f", 0, Complete))
        .end();
}

#[test]
fn top_underneath_one_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo top c")
        .validate()
        .printed_exact_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_exact_task(&PrintableTask::new("b", 2, Incomplete))
        .end();
}

#[test]
fn top_intersection_of_categories() {
    let mut fix = Fixture::new();
    fix.test("todo new x y");
    fix.test("todo new a b -b x");
    fix.test("todo new c d -b x y");
    fix.test("todo new d e -b y");
    fix.test("todo top x y")
        .validate()
        .printed_exact_task(&PrintableTask::new("c", 3, Incomplete))
        .printed_exact_task(&PrintableTask::new("d", 4, Incomplete))
        .end();
}
