use app::util::format_task;
use chrono::DateTime;
use chrono::Utc;
use model::TaskStatus;
use model::TodoList;
use printing::TodoPrinter;

pub struct Status {
    pub include_blocked: bool,
    pub include_done: bool,
}

pub fn run(
    model: &TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Status,
) {
    model
        .all_tasks()
        .filter(|&id| match model.status(id) {
            Some(TaskStatus::Blocked) => cmd.include_blocked,
            Some(TaskStatus::Complete) => cmd.include_done,
            Some(TaskStatus::Incomplete) => true,
            Some(TaskStatus::Removed) => {
                // TODO(printing.task-status): Distinguish between model
                // TaskStatus and printing TaskStatus; only the latter should
                // have the Removed variant.
                dbg!("Iterated TaskStatus::Removed task for some reason.");
                false
            }
            None => false,
        })
        .for_each(|id| printer.print_task(&format_task(model, id, now)))
}
