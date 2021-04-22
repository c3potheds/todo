use app::util::format_task;
use model::TaskStatus;
use model::TodoList;
use printing::TodoPrinter;

pub struct Status {
    pub include_blocked: bool,
    pub include_done: bool,
}

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter, cmd: &Status) {
    model
        .all_tasks()
        .filter(|&id| match model.status(id) {
            Some(TaskStatus::Blocked) => cmd.include_blocked,
            Some(TaskStatus::Complete) => cmd.include_done,
            Some(TaskStatus::Incomplete) => true,
            None => false,
        })
        .for_each(|id| printer.print_task(&format_task(model, id)))
}
