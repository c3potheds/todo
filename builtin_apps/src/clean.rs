use chrono::DateTime;
use chrono::Utc;
use todo_model::TodoList;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;

use crate::util::format_task;

pub fn run<'list>(
    list: &'list mut TodoList,
    _now: DateTime<Utc>,
) -> PrintableResult<'list> {
    let tasks_to_print = list.clean();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print
            .iter_sorted(list)
            .map(|id| format_task(list, id))
            .collect(),
        mutated: true,
        ..Default::default()
    })
}
