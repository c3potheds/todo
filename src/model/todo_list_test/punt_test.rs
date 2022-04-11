use super::*;

#[test]
fn punt_only_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.punt(a).expect("Cannot punt a");
    assert_eq!(list.position(a), Some(1));
}

#[test]
fn punt_first_of_three_tasks() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.punt(a).expect("Cannot punt a");
    itertools::assert_equal(list.incomplete_tasks(), vec![b, c, a]);
}

#[test]
fn cannot_punt_complete_task() {
    let mut list = TodoList::default();
    let a = list.add("a");
    list.check(a).expect("Cannot check a");
    list.punt(a)
        .expect_err("Punting a complete task should be an error");
}

#[test]
fn punt_blocked_task_moves_to_end_of_layer() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Cannot block b on a");
    list.block(c).on(a).expect("Cannot block c on a");
    list.punt(b).expect("Could not punt b");
    itertools::assert_equal(list.incomplete_tasks(), vec![a, c, b]);
}
