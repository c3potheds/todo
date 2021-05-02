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
            if !underneath.is_empty() {
                underneath.iter_unsorted().all(|top| {
                    model.deps(top).contains(id)
                        && !model.adeps(id).iter_unsorted().any(|adep| {
                            model.transitive_adeps(adep).contains(top)
                        })
                })
            } else {
                model.adeps(id).is_empty()
            }
        })
        .for_each(|id| printer.print_task(&format_task(model, id)))
}
