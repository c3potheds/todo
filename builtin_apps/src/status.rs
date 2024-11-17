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
}

pub fn run<'list>(
    list: &'list mut TodoList,
    now: DateTime<Utc>,
    cmd: &Status,
) -> PrintableResult<'list> {
    let unsnoozed_tasks = list.unsnooze_up_to(now);
    let tasks_to_print = list
        .all_tasks()
        .filter(|&id| match list.status(id) {
            Some(TaskStatus::Blocked) => cmd.include_blocked,
            Some(TaskStatus::Complete) => cmd.include_done,
            Some(TaskStatus::Incomplete) => true,
            None => false,
        })
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
