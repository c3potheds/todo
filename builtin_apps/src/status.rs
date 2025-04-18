use chrono::DateTime;
use chrono::Utc;
use todo_model::TaskStatus;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;

use super::util::format_task;

pub struct Status {
    pub include_blocked: bool,
    pub include_done: bool,
    // Add include_all flag to control focus filtering
    pub include_all: bool,
}

pub fn run<'list>(
    list: &'list mut TodoList,
    now: DateTime<Utc>,
    cmd: &Status,
) -> PrintableResult<'list> {
    let unsnoozed_tasks = list.unsnooze_up_to(now);
    let mut tasks_iter = list.all_tasks();

    // Apply focus filter conditionally
    let tasks_to_print = tasks_iter
        .filter(|&id| match list.status(id) {
            Some(TaskStatus::Blocked) => cmd.include_blocked,
            Some(TaskStatus::Complete) => cmd.include_done,
            Some(TaskStatus::Incomplete) => true,
            None => false,
        })
        // Apply focus filter *unless* include_all is true
        .filter(|&id| cmd.include_all || list.is_effectively_in_focus(id, now))
        .map(|id| {
            format_task(list, id).action(if unsnoozed_tasks.contains(id) {
                Action::Unsnooze
            } else {
                Action::None
            })
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated: !unsnoozed_tasks.is_empty(),
        ..Default::default()
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{task, Fixture, Mutated};
    use chrono::TimeZone;
    use todo_model::{FocusPredicate, TaskId};
    use chrono::{NaiveTime, Weekday};
    use std::collections::HashSet;
    use todo_testing::ymdhms; // Use testing helper for DateTime


    // Helper to create HashSet<Weekday>
    fn weekdays(days: &[Weekday]) -> HashSet<Weekday> {
        days.iter().cloned().collect()
    }
    // Helper to create NaiveTime
    fn time(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }

    #[test]
    fn status_shows_all_by_default() {
        // Basic test without focus to ensure it still works
        let mut fix = Fixture::default();
        fix.test("todo new task1 task2");
        fix.test("todo") // Default status command
            .modified(Mutated::No)
            .validate()
            .printed_task(&task("task1", 1, Incomplete))
            .printed_task(&task("task2", 2, Incomplete))
            .end();
    }


    #[test]
    fn status_focus_filtering() {
        let mut fix = Fixture::default();
        fix.test("todo new no_focus");                            // Task 0
        fix.test("todo new weekdays --focus weekdays");            // Task 1
        fix.test("todo new weekends --focus weekends");            // Task 2
        fix.test("todo new nine_to_five --focus 9am-5pm");       // Task 3
        fix.test("todo new blocked_by_weekdays -p weekdays");    // Task 4 (depends on task 1)
        fix.test("todo new blocked_by_weekends -p weekends");    // Task 5 (depends on task 2)

        // Scenario 1: Monday Noon (Weekdays=Y, Weekends=N, 9-5=Y)
        fix.clock.now = ymdhms(2023, 10, 30, 12, 0, 0); // Monday Noon
        fix.test("todo") // Default should filter
            .modified(Mutated::No) // Unsnooze doesn't happen here
            .validate()
            .printed_task(&task("no_focus", 1, Incomplete)) // Always shown if no focus
            .printed_task(&task("weekdays", 2, Incomplete).adeps_stats(1, 1)) // Shown (weekday focus)
            .printed_task(&task("nine_to_five", 4, Incomplete)) // Shown (time focus)
            // weekends (Task 3) is NOT shown
            // blocked_by_weekdays (Task 5) IS shown (dep is in focus)
            .printed_task(&task("blocked_by_weekdays", 5, Blocked).deps_stats(1, 1))
            // blocked_by_weekends (Task 6) is NOT shown (dep is out of focus)
            .end();

        // Scenario 2: Monday 6 PM (Weekdays=Y, Weekends=N, 9-5=N)
        fix.clock.now = ymdhms(2023, 10, 30, 18, 0, 0); // Monday 6 PM
        fix.test("todo")
            .modified(Mutated::No)
            .validate()
            .printed_task(&task("no_focus", 1, Incomplete))
            .printed_task(&task("weekdays", 2, Incomplete).adeps_stats(1, 1)) // Shown (weekday focus)
            // weekends (Task 3) is NOT shown
            // nine_to_five (Task 4) is NOT shown (time focus)
            .printed_task(&task("blocked_by_weekdays", 5, Blocked).deps_stats(1, 1)) // Shown (dep is in focus)
            // blocked_by_weekends (Task 6) is NOT shown
            .end();


        // Scenario 3: Saturday Noon (Weekdays=N, Weekends=Y, 9-5=Y)
        fix.clock.now = ymdhms(2023, 11, 4, 12, 0, 0); // Saturday Noon
        fix.test("todo")
            .modified(Mutated::No)
            .validate()
            .printed_task(&task("no_focus", 1, Incomplete))
            // weekdays (Task 2) is NOT shown
            .printed_task(&task("weekends", 3, Incomplete).adeps_stats(1, 1)) // Shown (weekend focus)
            .printed_task(&task("nine_to_five", 4, Incomplete)) // Shown (time focus)
            // blocked_by_weekdays (Task 5) is NOT shown
            .printed_task(&task("blocked_by_weekends", 6, Blocked).deps_stats(1, 1)) // Shown (dep is in focus)
            .end();


         // Scenario 4: Monday Noon, but with --include-all (-a)
         fix.clock.now = ymdhms(2023, 10, 30, 12, 0, 0); // Monday Noon
         fix.test("todo -a") // Should ignore focus filter
             .modified(Mutated::No)
             .validate()
             .printed_task(&task("no_focus", 1, Incomplete))
             .printed_task(&task("weekdays", 2, Incomplete).adeps_stats(1, 1))
             .printed_task(&task("weekends", 3, Incomplete).adeps_stats(1, 1))
             .printed_task(&task("nine_to_five", 4, Incomplete))
             .printed_task(&task("blocked_by_weekdays", 5, Blocked).deps_stats(1, 1))
             .printed_task(&task("blocked_by_weekends", 6, Blocked).deps_stats(1, 1))
             .end();
    }
}
