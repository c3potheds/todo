use app::util::format_task;
use chrono::DateTime;
use chrono::Utc;
use model::TodoList;
use printing::TodoPrinter;

pub fn run(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
) {
    list.all_tasks()
        .filter(|&id| {
            list.get(id)
                .map(|task| task.start_date > now)
                .unwrap_or_else(|| false)
        })
        .for_each(|id| printer.print_task(&format_task(list, id)));
}
