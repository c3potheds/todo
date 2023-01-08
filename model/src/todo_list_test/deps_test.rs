use super::*;
use ::pretty_assertions::assert_eq;

#[test]
fn deps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.deps(a).as_sorted_vec(&list), []);
    Ok(())
}

#[test]
fn deps_of_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.deps(c).as_sorted_vec(&list), [a, b]);
    Ok(())
}

#[test]
fn deps_of_task_blocked_by_completed_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.check(a)?;
    assert_eq!(list.deps(b).as_sorted_vec(&list), [a]);
    Ok(())
}

#[test]
fn deps_of_task_with_depth_higher_than_one() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.deps(c).as_sorted_vec(&list), [b]);
    Ok(())
}

#[test]
fn adeps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.adeps(a).as_sorted_vec(&list), []);
    Ok(())
}

#[test]
fn adeps_of_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(a)?;
    assert_eq!(list.adeps(a).as_sorted_vec(&list), [b, c]);
    Ok(())
}

#[test]
fn adeps_of_completed_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.check(a)?;
    assert_eq!(list.adeps(a).as_sorted_vec(&list), [b]);
    Ok(())
}

#[test]
fn adeps_of_task_with_depth_of_one() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.adeps(b).as_sorted_vec(&list), [c]);
    Ok(())
}

#[test]
fn transitive_deps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.transitive_deps(a).as_sorted_vec(&list), []);
    Ok(())
}

#[test]
fn transitive_deps_of_blocked_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a)?;
    list.block(c).on(a)?;
    list.block(d).on(b)?;
    list.block(d).on(c)?;
    assert_eq!(list.transitive_deps(d).as_sorted_vec(&list), [a, b, c]);
    Ok(())
}

#[test]
fn transitive_deps_includes_complete_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    list.check(a)?;
    list.check(b)?;
    assert_eq!(list.transitive_deps(c).as_sorted_vec(&list), [a, b]);
    Ok(())
}

#[test]
fn transitive_adeps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.transitive_adeps(a).as_sorted_vec(&list), []);
    Ok(())
}

#[test]
fn transitive_adeps_of_blocking_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a)?;
    list.block(c).on(a)?;
    list.block(d).on(b)?;
    list.block(d).on(c)?;
    assert_eq!(list.transitive_adeps(a).as_sorted_vec(&list), [b, c, d]);
    Ok(())
}

#[test]
fn transitive_adeps_includes_complete_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    list.check(a)?;
    list.check(b)?;
    assert_eq!(list.transitive_adeps(a).as_sorted_vec(&list), [b, c]);
    Ok(())
}
