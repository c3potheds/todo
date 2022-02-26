use app::util::format_task;
use chrono::Datelike;
use chrono::Local;
use model::TodoList;
use printing::LogDate;
use printing::TodoPrinter;

pub fn run(list: &TodoList, printer: &mut impl TodoPrinter) {
    let mut most_recent_shown = None;
    list.complete_tasks().for_each(|id| {
        let mut formatted_task = format_task(list, id);
        let to_show =
            list.get(id)
                .unwrap()
                .completion_time
                .map(|completion_time| {
                    let completion_time = completion_time.with_timezone(&Local);
                    LogDate::YearMonthDay(
                        completion_time.year() as u16,
                        completion_time.month() as u8,
                        completion_time.day() as u8,
                    )
                });
        formatted_task =
            formatted_task.log_date(if to_show != most_recent_shown {
                most_recent_shown = to_show.clone();
                to_show.unwrap()
            } else {
                LogDate::Invisible
            });
        printer.print_task(&formatted_task);
    });
}
