use app::util::format_task;
use app::util::lookup_tasks;
use cli::Edit;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

pub fn run(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Edit) {
    lookup_tasks(model, &cmd.keys)
        .into_iter()
        .flat_map(|id| {
            model
                .get_mut(id)
                .map(|task| {
                    task.desc = cmd.desc.clone();
                    id
                })
                .into_iter()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(model, id, Action::None));
        });
}
