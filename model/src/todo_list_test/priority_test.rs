use super::*;

use ::pretty_assertions::assert_eq;

#[test]
fn sorted_by_priority_two_tasks() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add(NewOptions::new().desc("b").priority(2));
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [b, a]);
}

#[test]
fn sorted_by_priority_three_tasks() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add(NewOptions::new().desc("b").priority(2));
    let c = list.add(NewOptions::new().desc("c").priority(3));
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [c, b, a]);
}

#[test]
fn sort_by_implicit_priority() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add("b");
    let c = list.add(NewOptions::new().desc("c").priority(2));
    list.block(c).on(b).unwrap();
    // b appears first because it has an implicit priority of 2, higher than
    // a's priority of 1.
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [b, a, c]);
}

#[test]
fn transitive_deps_sorted_by_priority() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add(NewOptions::new().desc("c").priority(1));
    list.block(c).on(b).unwrap();
    list.block(d).on(c).unwrap();
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [b, a, c, d]);
}

#[test]
fn priority_tasks_before_no_priority_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("a").priority(1));
    let c = list.add("c");
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [b, a, c]);
}

#[test]
fn tasks_with_negative_priority_appear_last() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").priority(-1));
    let c = list.add("c");
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, c, b]);
}

#[test]
fn implicit_priority_resets_if_adep_with_priority_is_unblocked() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add("b");
    let c = list.add(NewOptions::new().desc("c").priority(2));
    list.block(c).on(b).unwrap();
    list.unblock(c).from(b).unwrap();
    // c appears first because it has the highest priority, followed by a, which
    // has its own explicit priority. b had c's implicit priority before, but
    // because c was unblocked from b, b no longer has an implicit priority, and
    // so goes last.
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [c, a, b]);
}

#[test]
fn implicit_priority_of_unprioritized_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.implicit_priority(a), Some(0));
}

#[test]
fn implicit_priority_of_task_with_explicit_priority() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    assert_eq!(list.implicit_priority(a), Some(1));
}

#[test]
fn implicit_priority_of_task_with_prioritized_adep() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").priority(1));
    assert_eq!(list.implicit_priority(a), Some(0));
    list.block(b).on(a).unwrap();
    assert_eq!(list.implicit_priority(a), Some(1));
}

#[test]
fn set_priority_simple() {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.get(a).map(|task| task.priority), Some(0));
    list.set_priority(a, 1);
    assert_eq!(list.get(a).map(|task| task.priority), Some(1));
}

#[test]
fn set_priority_updates_deps_position() {
    let mut list = TodoList::default();
    list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    assert_eq!(list.position(b), Some(2));
    list.set_priority(c, 1);
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.implicit_priority(b), Some(1));
}

#[test]
fn set_priority_updates_transitive_deps_position() {
    let mut list = TodoList::default();
    list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(c).on(b).unwrap();
    list.block(d).on(c).unwrap();
    list.set_priority(d, 1);
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.implicit_priority(b), Some(1));
}

#[test]
fn set_priority_does_not_return_unaffected_tasks() {
    let mut list = TodoList::default();
    list.add("a");
    let b = list.add("b");
    let affected = list.set_priority(b, 1);
    assert_eq!(affected.as_sorted_vec(&list), [b]);
}

#[test]
fn set_priority_returns_empty_set_if_priority_is_same() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let affected = list.set_priority(a, 0);
    assert_eq!(affected.as_sorted_vec(&list), []);
}

#[test]
fn set_priority_returns_affected_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let affected = list.set_priority(c, 1);
    assert_eq!(affected.as_sorted_vec(&list), [a, b, c]);
}

#[test]
fn set_priority_returns_transitively_affected_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    list.block(b).on(a).unwrap();
    let affected = list.set_priority(c, 1);
    assert_eq!(affected.as_sorted_vec(&list), [a, b, c]);
}

#[test]
fn set_priority_returns_empty_set_if_task_is_removed() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.remove(a);
    let affected = list.set_priority(a, 1);
    assert_eq!(affected.as_sorted_vec(&list), []);
}

#[test]
fn set_priority_includes_complete_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.check(a).unwrap();
    let affected = list.set_priority(b, 1);
    assert_eq!(affected.as_sorted_vec(&list), [a, b]);
}

#[test]
fn set_priority_shows_affected_deps_without_b() {
    #![allow(clippy::many_single_char_names)]
    let mut list = TodoList::default();
    let b = list.add("b");
    let e = list.add("e");
    let a = list.add("a");
    let c = list.add("c");
    let d = list.add("d");
    list.block(d).on(a).unwrap();
    list.block(d).on(c).unwrap();
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [b, e, a, c, d]);
    let affected = list.set_priority(d, 1);
    assert_eq!(
        affected.iter_sorted(&list).collect::<Vec<_>>(),
        [a, c, d]
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, c, b, e, d]);
}

#[test]
fn set_priority_shows_affected_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(d).on(a).unwrap();
    list.block(d).on(c).unwrap();
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, b, c, d]);
    let affected = list.set_priority(d, 1);
    assert_eq!(
        affected.iter_sorted(&list).collect::<Vec<_>>(),
        [a, c, d]
    );
    // a and c have a higher implicit priority than b, so should appear first.
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), [a, c, b, d]);
}

#[test]
fn set_priority_with_no_affected_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.set_priority(a, 1);
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.set_priority(b, 1)
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        [b]
    );
}
