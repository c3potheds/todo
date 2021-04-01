use app::util::format_task;
use app::util::lookup_tasks;
use cli::Top;
use model::TaskStatus;
use model::TodoList;
use printing::TodoPrinter;

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter, cmd: &Top) {
    let underneath = lookup_tasks(model, &cmd.keys);
    model
        .all_tasks()
        .filter(|&id| {
            if !cmd.include_done
                && model.status(id) == Some(TaskStatus::Complete)
            {
                return false;
            }
            if underneath.len() > 0 {
                underneath.iter().all(|&top| model.deps(top).contains(id))
            } else {
                model.adeps(id).iter_unsorted().collect::<Vec<_>>().len() == 0
            }
        })
        .for_each(|id| printer.print_task(&format_task(model, id)))
}
