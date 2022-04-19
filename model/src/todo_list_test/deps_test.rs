use super::*;

#[test]
fn deps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(list.deps(a).iter_sorted(&list), Vec::new());
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
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![a, b]);
    Ok(())
}

#[test]
fn deps_of_task_blocked_by_completed_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.check(a)?;
    itertools::assert_equal(list.deps(b).iter_sorted(&list), vec![a]);
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
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![b]);
    Ok(())
}

#[test]
fn adeps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![]);
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
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b, c]);
    Ok(())
}

#[test]
fn adeps_of_completed_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    list.check(a)?;
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b]);
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
    itertools::assert_equal(list.adeps(b).iter_sorted(&list), vec![c]);
    Ok(())
}

#[test]
fn transitive_deps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(list.transitive_deps(a).iter_sorted(&list), vec![]);
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
    itertools::assert_equal(
        list.transitive_deps(d).iter_sorted(&list),
        vec![a, b, c],
    );
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
    itertools::assert_equal(
        list.transitive_deps(c).iter_sorted(&list),
        vec![a, b],
    );
    Ok(())
}

#[test]
fn transitive_adeps_of_standalone_task() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![],
    );
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
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![b, c, d],
    );
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
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![b, c],
    );
    Ok(())
}
