use chrono::Datelike;
use chrono::Local;
use todo_model::TodoList;
use todo_printing::LogDate;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;

use super::util::format_task;

pub fn run<'list>(list: &'list TodoList) -> PrintableResult<'list> {
    let mut most_recent_shown = None;
    let tasks_to_print =
        list.complete_tasks()
            .map(|id| {
                let formatted_task = format_task(list, id);
                let to_show = list.get(id).unwrap().completion_time.map(
                    |completion_time| {
                        let completion_time =
                            completion_time.with_timezone(&Local);
                        LogDate::YearMonthDay(
                            completion_time.year() as u16,
                            completion_time.month() as u8,
                            completion_time.day() as u8,
                        )
                    },
                );
                formatted_task.log_date(if to_show != most_recent_shown {
                    most_recent_shown.clone_from(&to_show);
                    to_show.unwrap()
                } else {
                    LogDate::Invisible
                })
            })
            .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        ..Default::default()
    })
}
