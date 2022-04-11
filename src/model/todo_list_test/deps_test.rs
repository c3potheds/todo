use super::*;
#[test]
fn deps_of_standalone_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(list.deps(a).iter_sorted(&list), Vec::new());
}

#[test]
fn deps_of_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![a, b]);
}

#[test]
fn deps_of_task_blocked_by_completed_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    itertools::assert_equal(list.deps(b).iter_sorted(&list), vec![a]);
}

#[test]
fn deps_of_task_with_depth_higher_than_one() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![b]);
}

#[test]
fn adeps_of_standalone_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![]);
}

#[test]
fn adeps_of_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b, c]);
}

#[test]
fn adeps_of_completed_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b]);
}

#[test]
fn adeps_of_task_with_depth_of_one() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.adeps(b).iter_sorted(&list), vec![c]);
}

#[test]
fn transitive_deps_of_standalone_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(list.transitive_deps(a).iter_sorted(&list), vec![]);
}

#[test]
fn transitive_deps_of_blocked_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(d).on(b).expect("Could not block d on b");
    list.block(d).on(c).expect("Could not block d on c");
    itertools::assert_equal(
        list.transitive_deps(d).iter_sorted(&list),
        vec![a, b, c],
    );
}

#[test]
fn transitive_deps_includes_complete_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    itertools::assert_equal(
        list.transitive_deps(c).iter_sorted(&list),
        vec![a, b],
    );
}

#[test]
fn transitive_adeps_of_standalone_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![],
    );
}

#[test]
fn transitive_adeps_of_blocking_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(d).on(b).expect("Could not block d on b");
    list.block(d).on(c).expect("Could not block d on c");
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![b, c, d],
    );
}

#[test]
fn transitive_adeps_includes_complete_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![b, c],
    );
}
