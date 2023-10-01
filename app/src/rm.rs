use {
    super::util::{format_task, lookup_tasks},
    todo_cli::Rm,
    todo_model::{TaskSet, TodoList},
    todo_printing::{PrintableAppSuccess, PrintableInfo, PrintableResult},
};

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: Rm,
) -> PrintableResult<'list> {
    let tasks_to_remove = lookup_tasks(list, &cmd.keys);
    let (removed_tasks, affected_tasks) =
        tasks_to_remove.iter_sorted(list).fold(
            (Vec::new(), TaskSet::default()),
            |(mut removed, affected), id| {
                let task = list.get(id).unwrap();
                removed.push(PrintableInfo::Removed {
                    desc: task.desc.to_string(),
                });
                (removed, affected | list.remove(id))
            },
        );
    let tasks_to_print = affected_tasks
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    let mutated = !removed_tasks.is_empty();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        infos: removed_tasks,
        mutated,
        ..Default::default()
    })
}
