use app::util::format_task;
use chrono::DateTime;
use chrono::Utc;
use cli::Find;
use model::TaskStatus;
use model::TodoList;
use printing::TodoPrinter;

pub fn run(
    model: &TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Find,
) {
    model
        .all_tasks()
        .filter(|&id| {
            let task = model.get(id).unwrap();
            cmd.terms
                .iter()
                .map(|term| term.to_lowercase())
                .any(|term| task.desc.to_lowercase().contains(&term))
        })
        .filter(|&id| {
            cmd.include_done || model.status(id) != Some(TaskStatus::Complete)
        })
        .for_each(|id| printer.print_task(&format_task(model, id, now)))
}
