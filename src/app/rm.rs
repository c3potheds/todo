use app::util::format_task;
use app::util::lookup_tasks;
use cli::Rm;
use model::TaskSet;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::PrintableTask;
use printing::TodoPrinter;

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: Rm) {
    lookup_tasks(model, &cmd.keys)
        .into_iter()
        .map(|id| {
            let task = model.get(id).unwrap();
            let pos = model.position(id).unwrap();
            printer.print_task(
                &PrintableTask::new(&task.desc, pos, TaskStatus::Removed)
                    .action(Action::Delete),
            );
            id
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flat_map(|id| model.remove(id).into_iter_unsorted())
        .collect::<TaskSet>()
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id, Action::None))
        })
}
