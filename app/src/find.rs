use {
    super::util::{format_task, lookup_tasks},
    cli::Find,
    lookup_key::Key,
    model::{TaskSet, TaskStatus, TodoList},
    printing::{PrintableAppSuccess, PrintableResult},
};

fn find_with_tag<'list>(
    list: &'list TodoList,
    cmd: &Find,
) -> PrintableResult<'list> {
    let keys = cmd
        .terms
        .iter()
        .map(|term| Key::ByName(term.to_string()))
        .collect::<Vec<_>>();
    Ok(PrintableAppSuccess {
        tasks: lookup_tasks(list, keys.iter())
            .iter_sorted(list)
            .fold(TaskSet::default(), |so_far, id| {
                so_far | list.transitive_deps(id) | TaskSet::of(id)
            })
            .iter_sorted(list)
            .filter(|&id| {
                cmd.include_done
                    || list.status(id) != Some(TaskStatus::Complete)
            })
            .map(|id| format_task(list, id))
            .collect(),
        ..Default::default()
    })
}

pub fn run<'list>(list: &'list TodoList, cmd: &Find) -> PrintableResult<'list> {
    if cmd.tag {
        return find_with_tag(list, cmd);
    }
    Ok(PrintableAppSuccess {
        tasks: list
            .all_tasks()
            .filter(|&id| {
                let task = list.get(id).unwrap();
                cmd.terms
                    .iter()
                    .map(|term| term.to_lowercase())
                    .any(|term| task.desc.to_lowercase().contains(&term))
            })
            .filter(|&id| {
                cmd.include_done
                    || list.status(id) != Some(TaskStatus::Complete)
            })
            .map(|id| format_task(list, id))
            .collect(),
        ..Default::default()
    })
}
