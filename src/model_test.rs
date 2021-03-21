use model::*;

#[test]
fn no_tasks() {
    let list = TodoList::new();
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), None);
}

#[test]
fn new_task_has_creation_time() {
    let task = Task::new("task");
    assert!(task.creation_time.is_some());
}

#[test]
fn new_task_has_no_completion_time() {
    let task = Task::new("task");
    assert!(task.completion_time.is_none());
}

#[test]
fn deserialize_task_with_missing_creation_time() {
    let task = serde_json::from_str::<Task>("{\"desc\":\"hi\"}")
        .ok()
        .unwrap();
    assert_eq!(task.desc, "hi");
    assert!(task.creation_time.is_none());
    assert!(task.completion_time.is_none());
}

#[test]
fn get_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}

#[test]
fn get_completed_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}

#[test]
fn add_one_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("hello, world"));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), None);
}

#[test]
fn add_multiple_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).expect("Could not check a");
    list.check(a)
        .expect_err("Shouldn't have been able to check a");
}

#[test]
fn checked_task_has_completion_time() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).expect("Could not check a");
    assert!(list.get(a).unwrap().completion_time.is_some());
}

#[test]
fn completion_time_of_completed_task_does_not_update_if_checked() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).expect("Could not check a");
    let original_completion_time =
        list.get(a).unwrap().completion_time.unwrap();
    list.check(a)
        .expect_err("Shouldn't have been able to check a");
    let new_completion_time = list.get(a).unwrap().completion_time.unwrap();
    assert_eq!(original_completion_time, new_completion_time);
}

#[test]
fn check_first_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    list.check(a).expect("Could not check a");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_second_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    list.check(b).expect("Could not check b");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_third_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("walk the dog"));
    let b = list.add(Task::new("do the dishes"));
    let c = list.add(Task::new("take out the trash"));
    list.check(c).expect("Could not check c");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), None);
}

#[test]
fn complete_task_shows_up_in_complete_list() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).expect("Could not check a");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
fn iterate_multiple_complete_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a).expect("Could not check a");
    list.check(c).expect("Could not check c");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(c));
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

fn all_tasks_ordered(list: &TodoList) -> Vec<TaskId> {
    list.complete_tasks()
        .chain(list.incomplete_tasks())
        .collect::<Vec<_>>()
}

// Serializes, then deserializes the list, and makes sure the reserialized
// version is equal to the original.
fn reload(list: &TodoList) {
    let serialized = serde_json::to_string(&list).unwrap();
    let deserialized = serde_json::from_str::<TodoList>(&serialized).unwrap();
    assert_eq!(all_tasks_ordered(list), all_tasks_ordered(&deserialized));
}

#[test]
fn reload_empty() {
    let list = TodoList::new();
    reload(&list);
}

#[test]
fn reload_single_task() {
    let mut list = TodoList::new();
    list.add(Task::new("pass this test"));
    reload(&list);
}

#[test]
fn reload_three_tasks() {
    let mut list = TodoList::new();
    list.add(Task::new("first"));
    list.add(Task::new("second"));
    list.add(Task::new("third"));
    reload(&list);
}

#[test]
fn number_of_incomplete_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    assert_eq!(list.position(a), Some(1));
    assert_eq!(list.position(b), Some(2));
    assert_eq!(list.position(c), Some(3));
}

#[test]
fn number_of_complete_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
    assert_eq!(list.position(c), Some(0));
    assert_eq!(list.position(b), Some(-1));
    assert_eq!(list.position(a), Some(-2));
}

#[test]
fn number_of_task_updates_when_predecessor_completes() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a).expect("Could not check a");
    assert_eq!(list.position(a), Some(0));
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(c), Some(2));
}

#[test]
fn existent_incomplete_task_by_number() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    assert_eq!(list.lookup_by_number(1), Some(a));
    assert_eq!(list.lookup_by_number(2), Some(b));
    assert_eq!(list.lookup_by_number(3), Some(c));
}

#[test]
fn nonexistent_incomplete_task_by_number() {
    let list = TodoList::new();
    assert_eq!(list.lookup_by_number(1), None);
}

#[test]
fn existent_complete_task_by_number() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
    assert_eq!(list.lookup_by_number(0), Some(c));
    assert_eq!(list.lookup_by_number(-1), Some(b));
    assert_eq!(list.lookup_by_number(-2), Some(a));
}

#[test]
fn nonexistent_complete_task_by_number() {
    let list = TodoList::new();
    assert_eq!(list.lookup_by_number(0), None);
}

#[test]
fn lookup_by_number_is_inverse_of_position() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.add(Task::new("d"));
    let e = list.add(Task::new("e"));
    list.check(a).expect("Could not check a");
    list.check(c).expect("Could not check c");
    list.check(e).expect("could not check e");
    for id in list.incomplete_tasks().chain(list.complete_tasks()) {
        let number = list.position(id).unwrap();
        let id_from_number = list.lookup_by_number(number).unwrap();
        assert_eq!(id_from_number, id);
    }
}

#[test]
fn restore_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    assert!(list.restore(a).is_err());
    assert_eq!(list.position(a), Some(1));
}

#[test]
fn restore_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).expect("Could not check a");
    list.restore(a).expect("Could not restore a");
    assert_eq!(list.position(a), Some(1));
}

#[test]
fn restore_complete_task_to_nonempty_list() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a).expect("Could not check a");
    list.restore(a).expect("Could not restore a");
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(c), Some(2));
    assert_eq!(list.position(a), Some(3));
}

#[test]
fn status_of_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
}

#[test]
fn status_of_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).expect("Could not check a");
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
}

#[test]
fn status_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn ordering_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.position(a), Some(1));
    assert_eq!(list.position(b), Some(2));
}

#[test]
fn blocked_task_appears_after_task_that_blocks_it() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(a).on(b).expect("Could not block a on b");
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
}

#[test]
fn cannot_block_blocking_task_on_task_it_blocks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(a).on(b).expect("Could not block a on b");
    assert!(list.block(b).on(a).is_err());
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
}

#[test]
fn cannot_block_on_self() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.block(a)
        .on(a)
        .expect_err("Shouldn't be able to block a task on itself.");
}

#[test]
fn incomplete_tasks_includes_blocked_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn chained_blocking() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(a).on(b).expect("Could not block a on b");
    list.block(b).on(c).expect("Could not block b on c");
    let mut incomplete_tasks = list.incomplete_tasks();
    let mut next = incomplete_tasks.next().unwrap();
    assert_eq!(list.get(next).unwrap().desc, "c");
    assert_eq!(list.status(next).unwrap(), TaskStatus::Incomplete);
    next = incomplete_tasks.next().unwrap();
    assert_eq!(list.get(next).unwrap().desc, "b");
    assert_eq!(list.status(next).unwrap(), TaskStatus::Blocked);
    next = incomplete_tasks.next().unwrap();
    assert_eq!(list.get(next).unwrap().desc, "a");
    assert_eq!(list.status(next).unwrap(), TaskStatus::Blocked);
}

#[test]
fn indirect_blocking_cycle() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    assert!(list.block(a).on(c).is_err());
    // Make sure the status is consistent.
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn cannot_check_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.check(b), Err(CheckError::TaskIsBlockedBy(vec![a])));
}

#[test]
fn can_check_task_whose_dependency_is_complete() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
}

#[test]
fn can_check_task_whose_dependencies_are_complete() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
}

#[test]
fn force_check_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let newly_unblocked = list.force_check(a).unwrap();
    itertools::assert_equal(newly_unblocked.iter_sorted(&list), vec![]);
}

#[test]
fn force_check_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let newly_unblocked = list.force_check(b).unwrap();
    itertools::assert_equal(newly_unblocked.iter_sorted(&list), vec![c]);
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
    assert_eq!(list.status(b), Some(TaskStatus::Complete));
    assert_eq!(list.status(c), Some(TaskStatus::Incomplete));
}

#[test]
fn force_check_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).unwrap();
    assert_eq!(list.force_check(a), Err(CheckError::TaskIsAlreadyComplete));
}

#[test]
fn task_becomes_blocked_if_dependency_is_restored() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    list.restore(a).expect("Could not restore a");
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn cannot_restore_task_with_complete_adeps() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).unwrap();
    list.check(a).unwrap();
    list.check(b).unwrap();
    assert_eq!(list.restore(a), Err(RestoreError::WouldRestore(vec![b])));
}

#[test]
fn complete_task_becomes_blocked_if_dependency_is_force_restored() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.force_restore(a).expect("Could not restore a");
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn complete_task_becomes_blocked_if_transitive_dependency_is_force_restored() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
    list.force_restore(a).expect("Could not restore a");
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn force_restore_returns_newly_blocked_tasks_on_success() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    list.check(a).unwrap();
    list.check(b).unwrap();
    list.check(c).unwrap();
    let newly_blocked = list.force_restore(a).unwrap();
    itertools::assert_equal(newly_blocked.iter_sorted(&list), vec![b, c]);
}

#[test]
fn force_restore_already_incomplete() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    assert_eq!(
        list.force_restore(a),
        Err(RestoreError::TaskIsAlreadyIncomplete)
    );
}

#[test]
fn blocked_task_comes_after_all_unblocked_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(a).on(b).expect("Could not block a on b");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn block_blocked_task_on_other_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(b).on(c).expect("Could not block b on c");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn block_complete_task_on_previously_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.block(b).on(a).expect("Could not block b on a");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(b));
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
#[ignore = "Do we need layers for complete tasks?"]
fn block_complete_task_on_later_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.block(a).on(b).expect("Could not block a on b");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), Some(b));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
fn unlbock_task_from_self_is_error() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.unblock(a)
        .from(a)
        .expect_err("Unblocking a task from itself is nonsensical");
}

#[test]
fn unblock_task_from_task_that_does_not_block_it() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.unblock(b)
        .from(a)
        .expect_err("Shouldn't be able to unblock b from a");
}

#[test]
fn unblock_task_from_blocking_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    list.unblock(b).from(a).expect("Could not unblock b from a");
}

#[test]
fn unblock_task_from_indirectly_blocking_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.unblock(c)
        .from(a)
        .expect_err("Shouldn't be able to unblock c from a");
}

#[test]
fn newly_unblocked_task_has_incomplete_status() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    list.unblock(b).from(a).expect("Could not unblock b from a");
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
}

#[test]
fn unblocked_task_is_still_blocked_if_it_has_remaining_dependencies() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.unblock(c).from(a).expect("Could not unblock c from a");
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    assert_eq!(list.position(c), Some(3));
}

#[test]
fn partially_unblocked_task_moves_to_lowest_possible_layer() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    let d = list.add(Task::new("d"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.block(d).on(b).expect("Could not block d on b");
    list.unblock(c).from(b).expect("Could not unblock c from b");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), Some(c));
    assert_eq!(incomplete_tasks.next(), Some(d));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn all_tasks_when_all_are_incomplete() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn all_tasks_when_all_are_complete() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn all_tasks_when_some_are_complete_and_some_are_blocked() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.check(a).expect("Could not check a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn deps_of_standalone_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    itertools::assert_equal(list.deps(a).iter_sorted(&list), Vec::new());
}

#[test]
fn deps_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![a, b]);
}

#[test]
fn deps_of_task_blocked_by_completed_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    itertools::assert_equal(list.deps(b).iter_sorted(&list), vec![a]);
}

#[test]
fn deps_of_task_with_depth_higher_than_one() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![b]);
}

#[test]
fn adeps_of_standalone_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![]);
}

#[test]
fn adeps_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b, c]);
}

#[test]
fn adeps_of_completed_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b]);
}

#[test]
fn adeps_of_task_with_depth_of_one() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.adeps(b).iter_sorted(&list), vec![c]);
}

#[test]
fn transitive_deps_of_standalone_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    itertools::assert_equal(list.transitive_deps(a).iter_sorted(&list), vec![]);
}

#[test]
fn transitive_deps_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    let d = list.add(Task::new("d"));
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
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
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
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![],
    );
}

#[test]
fn transitive_adeps_of_blocking_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    let d = list.add(Task::new("d"));
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
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![b, c],
    );
}

#[test]
fn punt_only_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.punt(a).expect("Cannot punt a");
    assert_eq!(list.position(a), Some(1));
}

#[test]
fn punt_first_of_three_tasks() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.punt(a).expect("Cannot punt a");
    itertools::assert_equal(list.incomplete_tasks(), vec![b, c, a]);
}

#[test]
fn cannot_punt_complete_task() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    list.check(a).expect("Cannot check a");
    list.punt(a)
        .expect_err("Punting a complete task should be an error");
}

#[test]
fn punt_blocked_task_moves_to_end_of_layer() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).expect("Cannot block b on a");
    list.block(c).on(a).expect("Cannot block c on a");
    list.punt(b).expect("Could not punt b");
    itertools::assert_equal(list.incomplete_tasks(), vec![a, c, b]);
}

#[test]
fn remove_task_does_not_invalidate_task_ids() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.remove(a);
    assert_eq!(list.get(a), None);
    assert_eq!(list.status(a), None);
    assert_eq!(list.get(b).unwrap().desc, "b");
    assert_eq!(list.get(c).unwrap().desc, "c");
}

#[test]
fn remove_task_updates_depth_of_adeps() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    list.block(b).on(a).unwrap();
    list.remove(a);
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
}

#[test]
fn remove_task_attaches_deps_to_adeps() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.remove(b);
    itertools::assert_equal(list.all_tasks(), vec![a, c]);
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
}

#[test]
fn remove_task_attaches_all_deps_to_adeps() {
    let mut list = TodoList::new();
    let a = list.add(Task::new("a"));
    let b = list.add(Task::new("b"));
    let c = list.add(Task::new("c"));
    let d = list.add(Task::new("d"));
    let e = list.add(Task::new("e"));
    list.block(c).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.block(d).on(c).unwrap();
    list.block(e).on(c).unwrap();
    list.remove(c);
    itertools::assert_equal(list.all_tasks(), vec![a, b, d, e]);
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(d), Some(TaskStatus::Blocked));
    assert_eq!(list.status(e), Some(TaskStatus::Blocked));
}
