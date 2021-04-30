use super::util::*;
use chrono::Duration;
use cli::Key;
use model::NewOptions;
use model::TodoList;
use printing::Action::*;
use printing::PrintableTask;
use printing::Status::*;

#[test]
fn format_task_basic() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let actual = format_task(&list, a);
    let expected = PrintableTask::new("a", 1, Incomplete);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_action() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let actual = format_task(&list, a).action(Punt);
    let expected = PrintableTask::new("a", 1, Incomplete).action(Punt);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_priority() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let actual = format_task(&list, a);
    let expected = PrintableTask::new("a", 1, Incomplete).priority(1);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_zero_priority() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a").priority(0));
    let actual = format_task(&list, a);
    let expected = PrintableTask::new("a", 1, Incomplete);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_budget() {
    let mut list = TodoList::new();
    let now = ::app::testing::ymdhms(2021, 04, 30, 09, 00, 00);
    let due = now + Duration::hours(5);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .budget(Duration::hours(10))
            .due_date(due),
    );
    let actual = format_task(&list, a);
    let expected = PrintableTask::new("a", 1, Incomplete)
        .due_date(due)
        .budget(Duration::hours(10));
    assert_eq!(actual, expected);
}

#[test]
fn lookup_by_number() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup1 = lookup_tasks(&list, std::iter::once(&Key::ByNumber(1)));
    itertools::assert_equal(lookup1, vec![a]);
    let lookup2 = lookup_tasks(&list, std::iter::once(&Key::ByNumber(2)));
    itertools::assert_equal(lookup2, vec![b]);
    let lookup3 = lookup_tasks(&list, std::iter::once(&Key::ByNumber(3)));
    itertools::assert_equal(lookup3, vec![c]);
}

#[test]
fn lookup_by_name() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup1 = lookup_tasks(&list, &[Key::ByName("a".to_string())]);
    itertools::assert_equal(lookup1, vec![a]);
    let lookup2 = lookup_tasks(&list, &[Key::ByName("b".to_string())]);
    itertools::assert_equal(lookup2, vec![b]);
    let lookup3 = lookup_tasks(&list, &[Key::ByName("c".to_string())]);
    itertools::assert_equal(lookup3, vec![c]);
}

#[test]
fn lookup_multiple_keys() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup_all = lookup_tasks(
        &list,
        &[Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
    );
    itertools::assert_equal(lookup_all, vec![a, b, c]);
}

#[test]
fn lookup_by_range() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup_1_2 = lookup_tasks(&list, &[Key::ByRange(1, 2)]);
    itertools::assert_equal(lookup_1_2, vec![a, b]);
    let lookup_2_3 = lookup_tasks(&list, &[Key::ByRange(2, 3)]);
    itertools::assert_equal(lookup_2_3, vec![b, c]);
    let lookup_1_3 = lookup_tasks(&list, &[Key::ByRange(1, 3)]);
    itertools::assert_equal(lookup_1_3, vec![a, b, c]);
}

#[test]
fn pairwise_empty() {
    let empty: Vec<i32> = Vec::new();
    itertools::assert_equal(pairwise(empty), vec![]);
}

#[test]
fn pairwise_single() {
    itertools::assert_equal(pairwise(vec![1]), vec![]);
}

#[test]
fn pairwise_two() {
    itertools::assert_equal(pairwise(vec![1, 2]), vec![(1, 2)]);
}

#[test]
fn pairwise_many() {
    itertools::assert_equal(
        pairwise(vec![1, 2, 3, 4, 5]),
        vec![(1, 2), (2, 3), (3, 4), (4, 5)],
    );
}
