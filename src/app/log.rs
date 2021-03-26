use app::util::format_task;
use chrono::Datelike;
use model::TodoList;
use printing::Action;
use printing::LogDate;
use printing::TodoPrinter;

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter) {
    let mut most_recent_shown = None;
    model.complete_tasks().for_each(|id| {
        let mut formatted_task = format_task(model, id, Action::None);
        let to_show =
            model
                .get(id)
                .unwrap()
                .completion_time
                .map(|completion_time| {
                    LogDate::YearMonthDay(
                        completion_time.year() as u16,
                        completion_time.month() as u8,
                        completion_time.day() as u8,
                    )
                });
        formatted_task.log_date = Some(if to_show != most_recent_shown {
            most_recent_shown = to_show.clone();
            to_show.unwrap()
        } else {
            LogDate::Invisible
        });
        printer.print_task(&formatted_task);
    });
}
