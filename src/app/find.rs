use {super::util::format_task, crate::cli::Find};
use {
    model::{TaskStatus, TodoList},
    printing::TodoPrinter,
};

pub fn run(list: &TodoList, printer: &mut impl TodoPrinter, cmd: &Find) {
    list.all_tasks()
        .filter(|&id| {
            let task = list.get(id).unwrap();
            cmd.terms
                .iter()
                .map(|term| term.to_lowercase())
                .any(|term| task.desc.to_lowercase().contains(&term))
        })
        .filter(|&id| {
            cmd.include_done || list.status(id) != Some(TaskStatus::Complete)
        })
        .for_each(|id| printer.print_task(&format_task(list, id)))
}
