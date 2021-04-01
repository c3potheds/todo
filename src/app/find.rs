use app::util::format_task;
use cli::Find;
use model::TodoList;
use printing::TodoPrinter;

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter, cmd: &Find) {
    model
        .all_tasks()
        .filter(|&id| {
            let task = model.get(id).unwrap();
            cmd.terms
                .iter()
                .map(|term| term.to_lowercase())
                .any(|term| task.desc.to_lowercase().contains(&term))
        })
        .for_each(|id| printer.print_task(&format_task(model, id)))
}
