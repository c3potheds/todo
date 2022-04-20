use {
    super::util::{format_task, lookup_tasks},
    cli::Rm,
    model::{TaskSet, TodoList},
    printing::{Action, PrintableTask, Status, TodoPrinter},
};

pub fn run(list: &mut TodoList, printer: &mut impl TodoPrinter, cmd: Rm) {
    lookup_tasks(list, &cmd.keys)
        .iter_sorted(list)
        .map(|id| {
            let task = list.get(id).unwrap();
            let pos = list.position(id).unwrap();
            printer.print_task(
                &PrintableTask::new(&task.desc, pos, Status::Removed)
                    .action(Action::Delete),
            );
            id
        })
        .collect::<Vec<_>>()
        .into_iter()
        .fold(TaskSet::default(), |so_far, id| so_far | list.remove(id))
        .iter_sorted(list)
        .for_each(|id| printer.print_task(&format_task(list, id)))
}
