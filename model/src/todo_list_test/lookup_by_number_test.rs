use super::*;

#[test]
fn existent_incomplete_task_by_number() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    assert_eq!(list.lookup_by_number(1), Some(a));
    assert_eq!(list.lookup_by_number(2), Some(b));
    assert_eq!(list.lookup_by_number(3), Some(c));
    Ok(())
}

#[test]
fn nonexistent_incomplete_task_by_number() -> TestResult {
    let list = TodoList::default();
    assert_eq!(list.lookup_by_number(1), None);
    Ok(())
}

#[test]
fn existent_complete_task_by_number() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a)?;
    list.check(b)?;
    list.check(c)?;
    assert_eq!(list.lookup_by_number(0), Some(c));
    assert_eq!(list.lookup_by_number(-1), Some(b));
    assert_eq!(list.lookup_by_number(-2), Some(a));
    Ok(())
}

#[test]
fn nonexistent_complete_task_by_number() -> TestResult {
    let list = TodoList::default();
    assert_eq!(list.lookup_by_number(0), None);
    Ok(())
}

#[test]
fn lookup_by_number_is_inverse_of_position() -> TestResult {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.add("b");
    let c = list.add("c");
    list.add("d");
    let e = list.add("e");
    list.check(a)?;
    list.check(c)?;
    list.check(e)?;
    for id in list.incomplete_tasks().chain(list.complete_tasks()) {
        let number = list.position(id).unwrap();
        let id_from_number = list.lookup_by_number(number).unwrap();
        assert_eq!(id_from_number, id);
    }
    Ok(())
}
