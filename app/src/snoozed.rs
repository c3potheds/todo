use {
    super::util::format_task,
    chrono::{DateTime, Utc},
    todo_model::TodoList,
    todo_printing::{PrintableAppSuccess, PrintableResult},
};

pub fn run<'list>(
    list: &'list TodoList,
    now: DateTime<Utc>,
) -> PrintableResult<'list> {
    Ok(PrintableAppSuccess {
        tasks: list
            .all_tasks()
            .filter(|&id| {
                list.get(id)
                    .map(|task| task.start_date > now)
                    .unwrap_or_else(|| false)
            })
            .map(|id| format_task(list, id))
            .collect(),
        ..Default::default()
    })
}
