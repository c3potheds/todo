use app::util::format_task;
use chrono::DateTime;
use chrono::Utc;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

pub struct Status {
    pub include_blocked: bool,
    pub include_done: bool,
}

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Status,
) {
    let unsnoozed_tasks = list.unsnooze_up_to(now);
    list.all_tasks()
        .filter(|&id| match list.status(id) {
            Some(TaskStatus::Blocked) => cmd.include_blocked,
            Some(TaskStatus::Complete) => cmd.include_done,
            Some(TaskStatus::Incomplete) => true,
            None => false,
        })
        .for_each(|id| {
            printer.print_task(&format_task(list, id).action(
                if unsnoozed_tasks.contains(id) {
                    Action::Unsnooze
                } else {
                    Action::None
                },
            ))
        })
}
