use app::util::format_task;
use app::util::lookup_tasks;
use cli::Rm;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableTask;
use printing::Status;
use printing::TodoPrinter;

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: Rm) {
    lookup_tasks(model, &cmd.keys)
        .iter_sorted(model)
        .map(|id| {
            let task = model.get(id).unwrap();
            let pos = model.position(id).unwrap();
            printer.print_task(
                &PrintableTask::new(&task.desc, pos, Status::Removed)
                    .action(Action::Delete),
            );
            id
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flat_map(|id| model.remove(id).into_iter_unsorted())
        .collect::<TaskSet>()
        .iter_sorted(model)
        .for_each(|id| printer.print_task(&format_task(model, id)))
}
