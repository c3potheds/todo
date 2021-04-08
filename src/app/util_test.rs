use super::util::*;
use cli::Key;
use model::Task;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::PrintableTask;

#[test]
fn format_task_basic() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let actual = format_task(&list, a);
    let expected = PrintableTask::new("a", 1, TaskStatus::Incomplete);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_action() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let actual = format_task(&list, a).action(Action::Punt);
    let expected =
        PrintableTask::new("a", 1, TaskStatus::Incomplete).action(Action::Punt);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_priority() {
    let mut list = TodoList::new();
    let a = list.add({
        let mut task = Task::new("a");
        task.priority = Some(1);
        task.implicit_priority = Some(1);
        task
    });
    let actual = format_task(&list, a);
    let expected =
        PrintableTask::new("a", 1, TaskStatus::Incomplete).priority(1);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_zero_priority() {
    let mut list = TodoList::new();
    let a = list.add({
        let mut task = Task::new("a");
        task.priority = Some(0);
        task
    });
    let actual = format_task(&list, a);
    let expected = PrintableTask::new("a", 1, TaskStatus::Incomplete);
    assert_eq!(actual, expected);
}

#[test]
fn lookup_by_number() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
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
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
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
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    let lookup_all = lookup_tasks(
        &list,
        &[Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
    );
    itertools::assert_equal(lookup_all, vec![a, b, c]);
}

#[test]
fn lookup_by_range() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
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
