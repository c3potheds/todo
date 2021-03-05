use app::util::format_task;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::PrintingContext;
use printing::TodoPrinter;

pub struct Status {
    pub include_blocked: bool,
}

pub fn run(
    model: &TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Status,
) {
    model
        .incomplete_tasks()
        .take_while(|&id| {
            cmd.include_blocked
                || model.get_status(id) == Some(TaskStatus::Incomplete)
        })
        .for_each(|id| {
            printer.print_task(&format_task(
                printing_context,
                model,
                id,
                Action::None,
            ))
        })
}
