use chrono::DateTime;
use chrono::Utc;
use todo_cli::Snoozed;
use todo_model::TodoList;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;

use super::util::format_task;
use crate::util::parse_due_date;

pub fn run<'list>(
    list: &'list TodoList,
    now: DateTime<Utc>,
    cmd: &Snoozed,
) -> PrintableResult<'list> {
    let until = match &cmd.until {
        Some(chunks) => parse_due_date(now, chunks).map_err(|e| vec![e])?,
        None => None,
    };
    Ok(PrintableAppSuccess {
        tasks: list
            .all_tasks()
            .filter(|&id| {
                list.get(id)
                    .map(|task| {
                        task.start_date > now
                            && (match until {
                                Some(limit) => task.start_date <= limit,
                                None => true,
                            })
                    })
                    .unwrap_or_else(|| false)
            })
            .map(|id| format_task(list, id))
            .collect(),
        ..Default::default()
    })
}
