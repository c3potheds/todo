use chrono::Duration;
use chrono::TimeZone;
use chrono::Utc;
use model::*;

#[test]
fn no_tasks() {
    let list = TodoList::new();
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), None);
}

#[test]
fn deserialize_task_with_missing_creation_time() {
    let task = serde_json::from_str::<Task>("{\"desc\":\"hi\"}")
        .ok()
        .unwrap();
    assert_eq!(task.desc, "hi");
    assert!(task.creation_time != Utc.ymd(1970, 01, 01).and_hms(00, 00, 00));
    assert_eq!(task.completion_time, None);
    assert_eq!(task.priority, 0);
    assert_eq!(task.implicit_priority, 0);
    assert_eq!(task.due_date, None);
    assert_eq!(task.implicit_due_date, None);
}

#[test]
fn get_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}

#[test]
fn get_completed_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    assert_eq!(list.get(a).unwrap().desc, "a");
    assert_eq!(list.get(b).unwrap().desc, "b");
}

#[test]
fn add_one_task() {
    let mut list = TodoList::new();
    let a = list.add("hello, world");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), None);
}

#[test]
fn add_multiple_tasks() {
    let mut list = TodoList::new();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_complete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    list.check(a)
        .expect_err("Shouldn't have been able to check a");
}

#[test]
fn checked_task_has_completion_time() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    assert!(list.get(a).unwrap().completion_time.is_some());
}

#[test]
fn completion_time_of_completed_task_does_not_update_if_checked() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    let original_completion_time =
        list.get(a).unwrap().completion_time.unwrap();
    list.check(a)
        .expect_err("Shouldn't have been able to check a");
    let new_completion_time = list.get(a).unwrap().completion_time.unwrap();
    assert_eq!(original_completion_time, new_completion_time);
}

#[test]
fn check_by_options_uses_injected_completion_time() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let now = Utc.ymd(2021, 03, 26).and_hms(04, 27, 00);
    list.check(CheckOptions { id: a, now: now }).unwrap();
    assert_eq!(list.get(a).unwrap().completion_time, Some(now));
}

#[test]
fn check_first_task() {
    let mut list = TodoList::new();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    list.check(a).expect("Could not check a");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_second_task() {
    let mut list = TodoList::new();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    list.check(b).expect("Could not check b");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(c));
    assert_eq!(tasks.next(), None);
}

#[test]
fn check_third_task() {
    let mut list = TodoList::new();
    let a = list.add("walk the dog");
    let b = list.add("do the dishes");
    let c = list.add("take out the trash");
    list.check(c).expect("Could not check c");
    let mut tasks = list.incomplete_tasks();
    assert_eq!(tasks.next(), Some(a));
    assert_eq!(tasks.next(), Some(b));
    assert_eq!(tasks.next(), None);
}

#[test]
fn complete_task_shows_up_in_complete_list() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    let mut complete_tasks = list.complete_tasks();
    assert_eq!(complete_tasks.next(), Some(a));
    assert_eq!(complete_tasks.next(), None);
}

#[test]
fn iterate_multiple_complete_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    list.add("pass this test");
    reload(&list);
}

#[test]
fn reload_three_tasks() {
    let mut list = TodoList::new();
    list.add("first");
    list.add("second");
    list.add("third");
    reload(&list);
}

#[test]
fn number_of_incomplete_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    assert_eq!(list.position(a), Some(1));
    assert_eq!(list.position(b), Some(2));
    assert_eq!(list.position(c), Some(3));
}

#[test]
fn number_of_complete_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a).expect("Could not check a");
    assert_eq!(list.position(a), Some(0));
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(c), Some(2));
}

#[test]
fn existent_incomplete_task_by_number() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    let a = list.add("a");
    list.add("b");
    let c = list.add("c");
    list.add("d");
    let e = list.add("e");
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
    let a = list.add("a");
    assert!(list.restore(a).is_err());
    assert_eq!(list.position(a), Some(1));
}

#[test]
fn restore_complete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    list.restore(a).expect("Could not restore a");
    assert_eq!(list.position(a), Some(1));
}

#[test]
fn restore_complete_task_to_nonempty_list() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a).expect("Could not check a");
    list.restore(a).expect("Could not restore a");
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(c), Some(2));
    assert_eq!(list.position(a), Some(3));
}

#[test]
fn status_of_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
}

#[test]
fn status_of_complete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).expect("Could not check a");
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
}

#[test]
fn status_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn ordering_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.position(a), Some(1));
    assert_eq!(list.position(b), Some(2));
}

#[test]
fn blocked_task_appears_after_task_that_blocks_it() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(a).on(b).expect("Could not block a on b");
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
}

#[test]
fn cannot_block_blocking_task_on_task_it_blocks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(a).on(b).expect("Could not block a on b");
    assert!(list.block(b).on(a).is_err());
    assert_eq!(list.position(b), Some(1));
    assert_eq!(list.position(a), Some(2));
}

#[test]
fn cannot_block_on_self() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.block(a)
        .on(a)
        .expect_err("Shouldn't be able to block a task on itself.");
}

#[test]
fn incomplete_tasks_includes_blocked_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    let mut incomplete_tasks = list.incomplete_tasks();
    assert_eq!(incomplete_tasks.next(), Some(a));
    assert_eq!(incomplete_tasks.next(), Some(b));
    assert_eq!(incomplete_tasks.next(), None);
}

#[test]
fn chained_blocking() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
fn block_returns_affected_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    let affected = list
        .block(c)
        .on(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![b, c]);
}

#[test]
fn block_returns_affected_task_priority_update() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.set_priority(c, 1);
    list.block(b).on(a).unwrap();
    let affected = list
        .block(c)
        .on(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![a, b, c]);
}

#[test]
fn block_does_not_return_unaffected_task_priority_update() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.set_priority(a, 1);
    list.set_priority(c, 1);
    list.block(b).on(a).unwrap();
    let affected = list
        .block(c)
        .on(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![b, c]);
}

#[test]
fn cannot_check_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    assert_eq!(list.check(b), Err(CheckError::TaskIsBlockedBy(vec![a])));
}

#[test]
fn can_check_task_whose_dependency_is_complete() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
}

#[test]
fn can_check_task_whose_dependencies_are_complete() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
}

#[test]
fn force_check_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let result = list.force_check(a).unwrap();
    itertools::assert_equal(result.completed.iter_sorted(&list), vec![a]);
    itertools::assert_equal(result.unblocked.iter_sorted(&list), vec![]);
}

#[test]
fn force_check_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let result = list.force_check(b).unwrap();
    itertools::assert_equal(result.completed.iter_sorted(&list), vec![a, b]);
    itertools::assert_equal(result.unblocked.iter_sorted(&list), vec![c]);
    assert_eq!(list.status(a), Some(TaskStatus::Complete));
    assert_eq!(list.status(b), Some(TaskStatus::Complete));
    assert_eq!(list.status(c), Some(TaskStatus::Incomplete));
}

#[test]
fn force_check_complete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).unwrap();
    assert_eq!(list.force_check(a), Err(CheckError::TaskIsAlreadyComplete));
}

#[test]
fn task_becomes_blocked_if_dependency_is_restored() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    list.restore(a).expect("Could not restore a");
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn cannot_restore_task_with_complete_adeps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.check(a).unwrap();
    list.check(b).unwrap();
    assert_eq!(
        list.restore(a),
        Err(RestoreError::WouldRestore(TaskSet::of(b)))
    );
}

#[test]
fn complete_task_becomes_blocked_if_dependency_is_force_restored() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
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
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    list.check(a).unwrap();
    list.check(b).unwrap();
    list.check(c).unwrap();
    let result = list.force_restore(a).unwrap();
    itertools::assert_equal(result.restored.iter_sorted(&list), vec![a, b, c]);
    itertools::assert_equal(result.blocked.iter_sorted(&list), vec![b, c]);
}

#[test]
fn force_restore_already_incomplete() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert_eq!(
        list.force_restore(a),
        Err(RestoreError::TaskIsAlreadyIncomplete)
    );
}

#[test]
fn blocked_task_comes_after_all_unblocked_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
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
    let a = list.add("a");
    let b = list.add("b");
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
    let a = list.add("a");
    let b = list.add("b");
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
    let a = list.add("a");
    list.unblock(a)
        .from(a)
        .expect_err("Unblocking a task from itself is nonsensical");
}

#[test]
fn unblock_task_from_task_that_does_not_block_it() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.unblock(b)
        .from(a)
        .expect_err("Shouldn't be able to unblock b from a");
}

#[test]
fn unblock_task_from_blocking_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.unblock(b).from(a).expect("Could not unblock b from a");
}

#[test]
fn unblock_task_from_indirectly_blocking_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.unblock(c)
        .from(a)
        .expect_err("Shouldn't be able to unblock c from a");
}

#[test]
fn newly_unblocked_task_has_incomplete_status() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.unblock(b).from(a).expect("Could not unblock b from a");
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
}

#[test]
fn unblocked_task_is_still_blocked_if_it_has_remaining_dependencies() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    list.unblock(c).from(a).expect("Could not unblock c from a");
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    assert_eq!(list.position(c), Some(3));
}

#[test]
fn partially_unblocked_task_moves_to_lowest_possible_layer() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
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
fn unblock_returns_affected_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    let affected = list
        .unblock(b)
        .from(a)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![a, b]);
}

#[test]
fn unblock_returns_affected_tasks_priority_update() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.set_priority(c, 1);
    let affected = list
        .unblock(c)
        .from(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![c, a, b]);
}

#[test]
fn unblock_does_not_return_unaffected_tasks_priority_update() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.set_priority(a, 1);
    list.set_priority(c, 1);
    let affected = list
        .unblock(c)
        .from(b)
        .unwrap()
        .iter_sorted(&list)
        .collect::<Vec<_>>();
    assert_eq!(affected, vec![c, b]);
}

#[test]
fn all_tasks_when_all_are_incomplete() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn all_tasks_when_all_are_complete() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a).expect("Could not check a");
    list.check(b).expect("Could not check b");
    list.check(c).expect("Could not check c");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn all_tasks_when_some_are_complete_and_some_are_blocked() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.check(a).expect("Could not check a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.all_tasks(), vec![a, b, c]);
}

#[test]
fn deps_of_standalone_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    itertools::assert_equal(list.deps(a).iter_sorted(&list), Vec::new());
}

#[test]
fn deps_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).expect("Could not block c on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![a, b]);
}

#[test]
fn deps_of_task_blocked_by_completed_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    itertools::assert_equal(list.deps(b).iter_sorted(&list), vec![a]);
}

#[test]
fn deps_of_task_with_depth_higher_than_one() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.deps(c).iter_sorted(&list), vec![b]);
}

#[test]
fn adeps_of_standalone_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![]);
}

#[test]
fn adeps_of_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(a).expect("Could not block c on a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b, c]);
}

#[test]
fn adeps_of_completed_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).expect("Could not block b on a");
    list.check(a).expect("Could not check a");
    itertools::assert_equal(list.adeps(a).iter_sorted(&list), vec![b]);
}

#[test]
fn adeps_of_task_with_depth_of_one() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Could not block b on a");
    list.block(c).on(b).expect("Could not block c on b");
    itertools::assert_equal(list.adeps(b).iter_sorted(&list), vec![c]);
}

#[test]
fn transitive_deps_of_standalone_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    itertools::assert_equal(list.transitive_deps(a).iter_sorted(&list), vec![]);
}

#[test]
fn transitive_deps_of_blocked_task() {
    let mut list = TodoList::new();
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
    let mut list = TodoList::new();
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
    let mut list = TodoList::new();
    let a = list.add("a");
    itertools::assert_equal(
        list.transitive_adeps(a).iter_sorted(&list),
        vec![],
    );
}

#[test]
fn transitive_adeps_of_blocking_task() {
    let mut list = TodoList::new();
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
    let mut list = TodoList::new();
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

#[test]
fn punt_only_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.punt(a).expect("Cannot punt a");
    assert_eq!(list.position(a), Some(1));
}

#[test]
fn punt_first_of_three_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.punt(a).expect("Cannot punt a");
    itertools::assert_equal(list.incomplete_tasks(), vec![b, c, a]);
}

#[test]
fn cannot_punt_complete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).expect("Cannot check a");
    list.punt(a)
        .expect_err("Punting a complete task should be an error");
}

#[test]
fn punt_blocked_task_moves_to_end_of_layer() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).expect("Cannot block b on a");
    list.block(c).on(a).expect("Cannot block c on a");
    list.punt(b).expect("Could not punt b");
    itertools::assert_equal(list.incomplete_tasks(), vec![a, c, b]);
}

#[test]
fn remove_task_does_not_invalidate_task_ids() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let adeps = list.remove(a);
    assert_eq!(list.get(a), None);
    assert_eq!(list.status(a), None);
    assert_eq!(list.get(b).unwrap().desc, "b");
    assert_eq!(list.get(c).unwrap().desc, "c");
    itertools::assert_equal(adeps.iter_sorted(&list), vec![]);
}

#[test]
fn remove_task_updates_depth_of_adeps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    let adeps = list.remove(a);
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    itertools::assert_equal(adeps.iter_sorted(&list), vec![b]);
}

#[test]
fn remove_task_attaches_deps_to_adeps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let adeps = list.remove(b);
    itertools::assert_equal(list.all_tasks(), vec![a, c]);
    assert_eq!(list.status(c), Some(TaskStatus::Blocked));
    itertools::assert_equal(adeps.iter_sorted(&list), vec![c]);
}

#[test]
fn remove_task_attaches_all_deps_to_adeps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    let e = list.add("e");
    list.block(c).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.block(d).on(c).unwrap();
    list.block(e).on(c).unwrap();
    let adeps = list.remove(c);
    itertools::assert_equal(list.all_tasks(), vec![a, b, d, e]);
    assert_eq!(list.status(a), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
    assert_eq!(list.status(d), Some(TaskStatus::Blocked));
    assert_eq!(list.status(e), Some(TaskStatus::Blocked));
    itertools::assert_equal(adeps.iter_sorted(&list), vec![d, e]);
}

#[test]
fn sorted_by_priority_two_tasks() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add(NewOptions::new().desc("b").priority(2));
    itertools::assert_equal(list.all_tasks(), vec![b, a]);
}

#[test]
fn sorted_by_priority_three_tasks() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add(NewOptions::new().desc("b").priority(2));
    let c = list.add(NewOptions::new().desc("c").priority(3));
    itertools::assert_equal(list.all_tasks(), vec![c, b, a]);
}

#[test]
fn priority_tasks_before_no_priority_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("a").priority(1));
    let c = list.add("c");
    itertools::assert_equal(list.all_tasks(), vec![b, a, c]);
}

#[test]
fn tasks_with_negative_priority_appear_last() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").priority(-1));
    let c = list.add("c");
    itertools::assert_equal(list.all_tasks(), vec![a, c, b]);
}

#[test]
fn sort_by_implicit_priority() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add("b");
    let c = list.add(NewOptions::new().desc("c").priority(2));
    list.block(c).on(b).unwrap();
    // b appears first because it has an implicit priority of 2, higher than
    // a's priority of 1.
    itertools::assert_equal(list.all_tasks(), vec![b, a, c]);
}

#[test]
fn transitive_deps_sorted_by_priority() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add(NewOptions::new().desc("c").priority(1));
    list.block(c).on(b).unwrap();
    list.block(d).on(c).unwrap();
    assert_eq!(list.position(b), Some(1));
    itertools::assert_equal(list.all_tasks(), vec![b, a, c, d]);
}

#[test]
fn implicit_priority_resets_if_adep_with_priority_is_unblocked() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let b = list.add("b");
    let c = list.add(NewOptions::new().desc("c").priority(2));
    list.block(c).on(b).unwrap();
    list.unblock(c).from(b).unwrap();
    // c appears first because it has the highest priority, followed by a, which
    // has its own explicit priority. b had c's implicit priority before, but
    // because c was unblocked from b, b no longer has an implicit priority, and
    // so goes last.
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![c, a, b]);
}

#[test]
fn num_incomplete_tasks() {
    let mut list = TodoList::new();
    assert_eq!(list.num_incomplete_tasks(), 0);
    let a = list.add("a");
    assert_eq!(list.num_incomplete_tasks(), 1);
    list.check(a).unwrap();
    assert_eq!(list.num_incomplete_tasks(), 0);
}

#[test]
fn num_complete_tasks() {
    let mut list = TodoList::new();
    assert_eq!(list.num_complete_tasks(), 0);
    let a = list.add("a");
    assert_eq!(list.num_complete_tasks(), 0);
    list.check(a).unwrap();
    assert_eq!(list.num_complete_tasks(), 1);
}

#[test]
fn set_desc_existent() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert!(list.set_desc(a, "b"));
    assert_eq!(list.get(a).unwrap().desc, "b");
}

#[test]
fn set_desc_nonexistent() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.remove(a);
    assert!(!list.set_desc(a, "b"));
    assert_eq!(list.get(a), None);
}

#[test]
fn implicit_priority_of_unprioritized_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert_eq!(list.implicit_priority(a), Some(0));
}

#[test]
fn implicit_priority_of_task_with_explicit_priority() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    assert_eq!(list.implicit_priority(a), Some(1));
}

#[test]
fn implicit_priority_of_task_with_prioritized_adep() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").priority(1));
    assert_eq!(list.implicit_priority(a), Some(0));
    list.block(b).on(a).unwrap();
    assert_eq!(list.implicit_priority(a), Some(1));
}

#[test]
fn set_priority_simple() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert_eq!(list.get(a).map(|task| task.priority), Some(0));
    list.set_priority(a, 1);
    assert_eq!(list.get(a).map(|task| task.priority), Some(1));
}

#[test]
fn set_priority_updates_deps_position() {
    let mut list = TodoList::new();
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
    let mut list = TodoList::new();
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
    let mut list = TodoList::new();
    list.add("a");
    let b = list.add("b");
    let affected = list.set_priority(b, 1);
    itertools::assert_equal(affected.iter_sorted(&list), vec![b]);
}

#[test]
fn set_priority_returns_empty_set_if_priority_is_same() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let affected = list.set_priority(a, 0);
    itertools::assert_equal(affected.iter_sorted(&list), vec![]);
}

#[test]
fn set_priority_returns_affected_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let affected = list.set_priority(c, 1);
    itertools::assert_equal(affected.iter_sorted(&list), vec![a, b, c]);
}

#[test]
fn set_priority_returns_transitively_affected_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    list.block(b).on(a).unwrap();
    let affected = list.set_priority(c, 1);
    itertools::assert_equal(affected.iter_sorted(&list), vec![a, b, c]);
}

#[test]
fn set_priority_returns_empty_set_if_task_is_removed() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.remove(a);
    let affected = list.set_priority(a, 1);
    itertools::assert_equal(affected.iter_sorted(&list), vec![]);
}

#[test]
fn set_priority_includes_complete_deps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.check(a).unwrap();
    let affected = list.set_priority(b, 1);
    itertools::assert_equal(affected.iter_sorted(&list), vec![a, b]);
}

#[test]
fn set_priority_shows_affected_deps_without_b() {
    let mut list = TodoList::new();
    let b = list.add("b");
    let e = list.add("e");
    let a = list.add("a");
    let c = list.add("c");
    let d = list.add("d");
    list.block(d).on(a).unwrap();
    list.block(d).on(c).unwrap();
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![b, e, a, c, d]);
    let affected = list.set_priority(d, 1);
    assert_eq!(
        affected.iter_sorted(&list).collect::<Vec<_>>(),
        vec![a, c, d]
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![a, c, b, e, d]);
}

#[test]
fn set_priority_shows_affected_deps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    list.block(d).on(a).unwrap();
    list.block(d).on(c).unwrap();
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![a, b, c, d]);
    let affected = list.set_priority(d, 1);
    assert_eq!(
        affected.iter_sorted(&list).collect::<Vec<_>>(),
        vec![a, c, d]
    );
    // a and c have a higher implicit priority than b, so should appear first.
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![a, c, b, d]);
}

#[test]
fn set_priority_with_no_affected_deps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.set_priority(a, 1);
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.set_priority(b, 1)
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        vec![b]
    );
}

#[test]
fn set_due_date_simple() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let affected =
        list.set_due_date(a, Some(Utc.ymd(2021, 04, 13).and_hms(17, 00, 00)));
    assert_eq!(
        list.get(a).unwrap().due_date,
        Some(Utc.ymd(2021, 04, 13).and_hms(17, 00, 00))
    );
    assert_eq!(
        list.get(a).unwrap().implicit_due_date,
        Some(Utc.ymd(2021, 04, 13).and_hms(17, 00, 00))
    );
    assert_eq!(affected.iter_sorted(&list).collect::<Vec<_>>(), vec![a]);
}

#[test]
fn set_due_date_returns_transitively_affected_tasks() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    list.block(b).on(a).unwrap();
    let affected =
        list.set_due_date(c, Some(Utc.ymd(2021, 04, 13).and_hms(17, 00, 00)));
    assert_eq!(
        affected.iter_sorted(&list).collect::<Vec<_>>(),
        vec![a, b, c]
    );
}

#[test]
fn set_due_date_excludes_unaffected_tasks() {
    let mut list = TodoList::new();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.ymd(2021, 04, 13).and_hms(16, 00, 00)),
    );
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(b).unwrap();
    list.block(b).on(a).unwrap();
    let affected =
        list.set_due_date(c, Some(Utc.ymd(2021, 04, 13).and_hms(17, 00, 00)));
    assert_eq!(affected.iter_sorted(&list).collect::<Vec<_>>(), vec![b, c]);
}

#[test]
fn get_due_date() {
    let mut list = TodoList::new();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.ymd(2021, 04, 08).and_hms(23, 59, 59)),
    );
    assert_eq!(
        list.get(a).unwrap().due_date.unwrap(),
        Utc.ymd(2021, 04, 08).and_hms(23, 59, 59),
    );
}

#[test]
fn due_date_from_new_options() {
    let mut list = TodoList::new();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.ymd(2021, 04, 09).and_hms(12, 00, 00)),
    );
    assert_eq!(
        list.get(a).unwrap().due_date.unwrap(),
        Utc.ymd(2021, 04, 09).and_hms(12, 00, 00)
    );
}

#[test]
fn sort_by_explicit_due_date() {
    let mut list = TodoList::new();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 26, 00)),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 25, 00)),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![b, a]);
}

#[test]
fn sort_keeps_task_with_earlier_due_date_first() {
    let mut list = TodoList::new();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 26, 00)),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 30, 00)),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![a, b]);
}

#[test]
fn sort_puts_task_with_due_date_before_task_without_due_date() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 25, 00)),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![b, a]);
}

#[test]
fn sort_by_implicit_due_date() {
    let mut list = TodoList::new();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 30, 00)),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 00, 00)),
    );
    list.block(b).on(a).unwrap();
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(11, 00, 00)),
    );
    let d = list.add(
        NewOptions::new()
            .desc("d")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(13, 00, 00)),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![c, a, d, b]);
}

#[test]
fn sort_by_priority_then_due_date() {
    let mut list = TodoList::new();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .priority(2)
            .due_date(Utc.ymd(2021, 04, 11).and_hms(13, 00, 00)),
    );
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .priority(1)
            .due_date(Utc.ymd(2021, 04, 11).and_hms(11, 00, 00)),
    );
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .priority(2)
            .due_date(Utc.ymd(2021, 04, 11).and_hms(12, 00, 00)),
    );
    assert_eq!(list.all_tasks().collect::<Vec<_>>(), vec![c, a, b]);
}

#[test]
fn implicit_due_date_of_task_with_no_adeps_or_due_date() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert_eq!(list.implicit_due_date(a), Some(None));
}

#[test]
fn implicit_due_date_of_nonexistent_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.remove(a);
    assert_eq!(list.implicit_due_date(a), None);
}

#[test]
fn implicit_due_date_is_earliest_due_date_of_adeps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(19, 00, 00)),
    );
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(20, 00, 00)),
    );
    let d = list.add(
        NewOptions::new()
            .desc("d")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(18, 00, 00)),
    );
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    list.block(d).on(a).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(Utc.ymd(2021, 04, 11).and_hms(18, 00, 00)))
    );
}

#[test]
fn implicit_due_date_is_earliest_due_date_of_transitive_adeps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(19, 00, 00)),
    );
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(20, 00, 00)),
    );
    let d = list.add(
        NewOptions::new()
            .desc("d")
            .due_date(Utc.ymd(2021, 04, 11).and_hms(18, 00, 00)),
    );
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    list.block(d).on(b).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(Utc.ymd(2021, 04, 11).and_hms(18, 00, 00)))
    );
}

#[test]
fn default_budget_is_zero() {
    let mut list = TodoList::new();
    let budget = DurationInSeconds(0);
    let a = list.add(NewOptions::new().desc("a"));
    assert_eq!(list.get(a).unwrap().budget, budget);
}

#[test]
fn new_task_with_budget() {
    let mut list = TodoList::new();
    let budget = DurationInSeconds::from(Duration::days(1));
    let a = list.add(NewOptions::new().desc("a").budget(budget));
    assert_eq!(list.get(a).unwrap().budget, budget);
}

#[test]
fn dep_of_task_with_budget_incorporates_budget_in_due_date() {
    let mut list = TodoList::new();
    let budget = DurationInSeconds::from(Duration::days(1));
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 22).and_hms(23, 59, 59))
            .budget(budget),
    );
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(Utc.ymd(2021, 04, 21).and_hms(23, 59, 59)))
    );
}

#[test]
fn chain_of_tasks_with_budgets() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").budget(Duration::days(1)));
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .due_date(Utc.ymd(2021, 04, 22).and_hms(23, 59, 59))
            .budget(Duration::days(1)),
    );
    list.block(b).on(a).unwrap();
    assert_eq!(list.implicit_due_date(a), Some(None));
    list.block(c).on(b).unwrap();
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(Utc.ymd(2021, 04, 20).and_hms(23, 59, 59)))
    );
}

#[test]
fn set_budget_for_nonexistent_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.remove(a);
    assert_eq!(
        list.set_budget(a, Duration::days(1))
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        vec![]
    );
}

#[test]
fn set_budget_for_task_with_no_deps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert_eq!(
        list.set_budget(a, Duration::days(1))
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        vec![a]
    );
    assert_eq!(
        list.get(a).unwrap().budget,
        DurationInSeconds(Duration::days(1).num_seconds() as u32)
    );
}

#[test]
fn set_budget_updates_deps() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .due_date(Utc.ymd(2021, 04, 22).and_hms(23, 59, 59)),
    );
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.set_budget(b, Duration::days(1))
            .iter_sorted(&list)
            .collect::<Vec<_>>(),
        vec![a, b]
    );
    assert_eq!(
        list.implicit_due_date(a),
        Some(Some(Utc.ymd(2021, 04, 21).and_hms(23, 59, 59)))
    );
}

#[test]
fn start_date_defaults_to_creation_time() {
    let mut list = TodoList::new();
    let a = list.add(NewOptions::new().desc("a"));
    assert_eq!(
        list.get(a).unwrap().start_date,
        list.get(a).unwrap().creation_time
    );
}

#[test]
fn set_start_date_in_new_options() {
    let mut list = TodoList::new();
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(NewOptions::new().desc("a").start_date(start_date));
    assert_eq!(list.get(a).unwrap().start_date, start_date);
}

#[test]
fn new_task_with_start_time_later_than_now_starts_out_snoozed() {
    let mut list = TodoList::new();
    let now = Utc.ymd(2021, 05, 25).and_hms(09, 00, 00);
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    assert_eq!(list.status(a).unwrap(), TaskStatus::Blocked);
}

#[test]
fn unsnooze_up_to_before_snooze_date() {
    let mut list = TodoList::new();
    let now = Utc.ymd(2021, 05, 25).and_hms(09, 00, 00);
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    let now = Utc.ymd(2021, 05, 30).and_hms(09, 00, 00);
    let unsnoozed = list.unsnooze_up_to(now);
    assert_eq!(unsnoozed.len(), 0);
    assert_eq!(list.status(a).unwrap(), TaskStatus::Blocked);
}

#[test]
fn unsnooze_up_to_after_snooze_date() {
    let mut list = TodoList::new();
    let now = Utc.ymd(2021, 05, 25).and_hms(09, 00, 00);
    let start_date = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(start_date),
    );
    let now = Utc.ymd(2021, 06, 01).and_hms(09, 00, 00);
    let unsnoozed = list.unsnooze_up_to(now);
    itertools::assert_equal(unsnoozed.iter_sorted(&list), vec![a]);
    assert_eq!(list.status(a).unwrap(), TaskStatus::Incomplete);
}

#[test]
fn unsnooze_up_to_unsnoozes_multiple_tasks() {
    let mut list = TodoList::new();
    let now = Utc.ymd(2021, 06, 01).and_hms(00, 00, 00);
    let snooze_a = Utc.ymd(2021, 06, 02).and_hms(00, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    let snooze_b = Utc.ymd(2021, 06, 03).and_hms(00, 00, 00);
    let b = list.add(
        NewOptions::new()
            .desc("b")
            .creation_time(now)
            .start_date(snooze_b),
    );
    let snooze_c = Utc.ymd(2021, 06, 04).and_hms(00, 00, 00);
    let c = list.add(
        NewOptions::new()
            .desc("c")
            .creation_time(now)
            .start_date(snooze_c),
    );
    let now = snooze_b;
    let unsnoozed = list.unsnooze_up_to(now);
    itertools::assert_equal(unsnoozed.iter_sorted(&list), vec![a, b]);
    let now = snooze_c;
    let unsnoozed = list.unsnooze_up_to(now);
    itertools::assert_equal(unsnoozed.iter_sorted(&list), vec![c]);
}

#[test]
fn unsnooze_updates_depth_of_adeps() {
    let mut list = TodoList::new();
    let now = Utc.ymd(2021, 05, 25).and_hms(10, 00, 00);
    let snooze_a = Utc.ymd(2021, 05, 25).and_hms(11, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    let b = list.add(NewOptions::new().desc("b").due_date(snooze_a));
    let c = list.add("c");
    let d = list.add("d");
    list.block(b).on(a).unwrap();
    list.block(d).on(c).unwrap();
    // c is first because it is unblocked an unsnoozed.
    // a is next because it's snoozed, but was added before d, which is blocked
    // by c.
    // b is blocked by a, and so appears in a deeper layer than a.
    itertools::assert_equal(list.incomplete_tasks(), vec![c, a, d, b]);
    let now = Utc.ymd(2021, 05, 25).and_hms(12, 00, 00);
    list.unsnooze_up_to(now);
    // a and b now appear before c and d, respectively, because they are in
    // the same layer, and have a due date which sorts them earlier the other
    // tasks with no due date.
    itertools::assert_equal(list.incomplete_tasks(), vec![a, c, b, d]);
}

#[test]
fn check_snoozed_task() {
    let mut list = TodoList::new();
    let now = Utc.ymd(2021, 05, 25).and_hms(13, 00, 00);
    let snooze_a = Utc.ymd(2021, 05, 25).and_hms(14, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    itertools::assert_equal(
        list.check(CheckOptions { id: a, now: now })
            .unwrap()
            .iter_sorted(&list),
        vec![],
    );
    itertools::assert_equal(list.incomplete_tasks(), vec![]);
    itertools::assert_equal(list.complete_tasks(), vec![a]);
}

#[test]
fn force_check_snoozed_task() {
    let mut list = TodoList::new();
    let now = Utc.ymd(2021, 05, 25).and_hms(13, 00, 00);
    let snooze_a = Utc.ymd(2021, 05, 25).and_hms(14, 00, 00);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .start_date(snooze_a),
    );
    itertools::assert_equal(
        list.force_check(CheckOptions { id: a, now: now })
            .unwrap()
            .completed
            .iter_sorted(&list),
        vec![a],
    );
    itertools::assert_equal(list.incomplete_tasks(), vec![]);
    itertools::assert_equal(list.complete_tasks(), vec![a]);
}

#[test]
fn snooze_incomplete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    assert_eq!(
        list.snooze(a, Utc.ymd(2021, 05, 25).and_hms(14, 00, 00)),
        Ok(())
    );
}

#[test]
fn snooze_complete_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    list.check(a).unwrap();
    assert_eq!(
        list.snooze(a, Utc.ymd(2021, 05, 25).and_hms(15, 00, 00)),
        Err(vec![SnoozeWarning::TaskIsComplete])
    );
}

#[test]
fn snooze_blocked_task() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.snooze(b, Utc.ymd(2021, 05, 25).and_hms(15, 00, 00)),
        Ok(())
    );
    itertools::assert_equal(
        list.unsnooze_up_to(Utc.ymd(2021, 05, 25).and_hms(16, 00, 00))
            .iter_sorted(&list),
        vec![],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn snooze_task_until_after_due_date() {
    let mut list = TodoList::new();
    let due_date = Utc.ymd(2021, 05, 25).and_hms(20, 00, 00);
    let snooze = Utc.ymd(2021, 05, 26).and_hms(00, 00, 00);
    let a = list.add(NewOptions::new().desc("a").due_date(due_date));
    assert_eq!(
        list.snooze(a, snooze),
        Err(vec![SnoozeWarning::SnoozedUntilAfterDueDate {
            snoozed_until: snooze,
            due_date: due_date,
        }])
    );
}

#[test]
fn snooze_task_until_after_implicit_due_date() {
    let mut list = TodoList::new();
    let due_date = Utc.ymd(2021, 05, 25).and_hms(20, 00, 00);
    let snooze = Utc.ymd(2021, 05, 26).and_hms(00, 00, 00);
    let a = list.add("a");
    let b = list.add(NewOptions::new().desc("b").due_date(due_date));
    list.block(b).on(a).unwrap();
    assert_eq!(
        list.snooze(a, snooze),
        Err(vec![SnoozeWarning::SnoozedUntilAfterDueDate {
            snoozed_until: snooze,
            due_date: due_date,
        }])
    );
}

#[test]
fn snoozed_blocked_task_remains_snoozed_when_deps_completed() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.snooze(b, Utc.ymd(2021, 05, 25).and_hms(16, 00, 00))
        .unwrap();
    itertools::assert_equal(
        list.check(CheckOptions {
            id: a,
            now: Utc.ymd(2021, 05, 25).and_hms(15, 00, 00),
        })
        .unwrap()
        .iter_sorted(&list),
        vec![],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Blocked));
}

#[test]
fn snoozed_blocked_task_unsnoozes_when_deps_completed_after_snooze_date() {
    let mut list = TodoList::new();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.snooze(b, Utc.ymd(2021, 05, 25).and_hms(16, 00, 00))
        .unwrap();
    itertools::assert_equal(
        list.check(CheckOptions {
            id: a,
            now: Utc.ymd(2021, 05, 25).and_hms(17, 00, 00),
        })
        .unwrap()
        .iter_sorted(&list),
        vec![b],
    );
    assert_eq!(list.status(b), Some(TaskStatus::Incomplete));
}
