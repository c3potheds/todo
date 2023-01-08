use super::*;

#[test]
fn without_tag() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a"));
    assert!(!list.get(a).unwrap().tag);
}

#[test]
fn add_tag() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    assert!(list.get(a).unwrap().tag);
}

#[test]
fn dependency_of_tag_has_tag_as_implicit_tag() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add("b");
    list.block(a).on(b)?;
    assert!(!list.get(b).unwrap().tag);
    assert_eq!(list.get(b).unwrap().implicit_tags, vec![a]);
    Ok(())
}

#[test]
fn transitive_dependency_of_tag_has_tag_as_implicit_tag() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add("b");
    let c = list.add("c");
    list.block(a).on(b)?;
    list.block(b).on(c)?;
    assert!(!list.get(c).unwrap().tag);
    assert_eq!(list.get(c).unwrap().implicit_tags, vec![a]);
    Ok(())
}

#[test]
fn dependency_of_multiple_tags() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add("c");
    list.block(a).on(c)?;
    list.block(b).on(c)?;
    assert!(!list.get(c).unwrap().tag);
    assert_eq!(list.get(c).unwrap().implicit_tags, vec![a, b]);
    Ok(())
}

#[test]
fn transitive_dependency_of_multiple_tags() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add("c");
    list.block(a).on(c)?;
    list.block(b).on(c)?;
    let d = list.add("d");
    list.block(c).on(d)?;
    assert!(!list.get(d).unwrap().tag);
    assert_eq!(list.get(d).unwrap().implicit_tags, vec![a, b]);
    Ok(())
}

#[test]
fn unblock_first_tag_removes_implicit_tag_from_dep() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add("c");
    list.block(a).on(c)?;
    list.block(b).on(c)?;
    list.unblock(a).from(c)?;
    assert_eq!(list.get(c).unwrap().implicit_tags, vec![b]);
    Ok(())
}

#[test]
fn unblock_second_tag_removes_implicit_tag_from_dep() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add("c");
    list.block(a).on(c)?;
    list.block(b).on(c)?;
    list.unblock(b).from(c)?;
    assert_eq!(list.get(c).unwrap().implicit_tags, vec![a]);
    Ok(())
}

#[test]
fn unblock_tag_removes_implicit_tag_from_transitive_dep() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add("b");
    let c = list.add("c");
    list.block(a).on(b)?;
    list.block(b).on(c)?;
    list.unblock(a).from(b)?;
    assert_eq!(list.get(c).unwrap().implicit_tags, vec![]);
    Ok(())
}

#[test]
fn complete_task_has_tags_from_adeps() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").as_tag());
    list.block(b).on(a)?;
    list.check(a)?;
    assert_eq!(list.get(a).unwrap().implicit_tags, vec![b]);
    Ok(())
}

#[test]
fn diamond_dependency() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add(NewOptions::new().desc("d").as_tag());
    list.block(b).on(a)?;
    list.block(c).on(a)?;
    list.block(d).on(b)?;
    list.block(d).on(c)?;
    assert_eq!(list.get(a).unwrap().implicit_tags, vec![d]);
    Ok(())
}

#[test]
fn subtags() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add(NewOptions::new().desc("c").as_tag());
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.get(a).unwrap().implicit_tags, vec![c, b]);
    Ok(())
}

#[test]
fn set_tag() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    assert_eq!(list.set_tag(a, true).as_sorted_vec(&list), [a]);
    Ok(())
}

#[test]
fn set_blocked_task_as_tag_returns_affected_deps() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.set_tag(b, true).as_sorted_vec(&list), [a, b]);
    Ok(())
}

#[test]
fn set_tag_on_tag_is_no_op() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add(NewOptions::new().desc("c").as_tag());
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.set_tag(b, true).as_sorted_vec(&list), []);
    Ok(())
}

#[test]
fn unmark_tag() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add(NewOptions::new().desc("c").as_tag());
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    assert_eq!(list.set_tag(b, false).as_sorted_vec(&list), [a, b]);
    Ok(())
}

#[test]
fn unmark_task_that_is_not_tag_is_no_op() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a)?;
    assert_eq!(list.set_tag(a, false).as_sorted_vec(&list), []);
    assert_eq!(list.set_tag(b, false).as_sorted_vec(&list), []);
    Ok(())
}

#[test]
fn set_tag_includes_complete_affected_tasks() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a)?;
    list.block(c).on(b)?;
    list.check(a)?;
    assert_eq!(list.set_tag(b, true).as_sorted_vec(&list), [a, b],);
    Ok(())
}
