use {
    super::util::format_task,
    todo_cli::Find,
    todo_model::{TaskStatus, TodoList},
    todo_printing::{Action, PrintableAppSuccess, PrintableResult},
};

pub fn run<'list>(list: &'list TodoList, cmd: &Find) -> PrintableResult<'list> {
    Ok(PrintableAppSuccess {
        tasks: list
            .all_tasks()
            .filter_map(|id| {
                let task = list.get(id).unwrap();
                if !cmd.include_done
                    && list.status(id) == Some(TaskStatus::Complete)
                {
                    return None;
                }
                cmd.terms
                    .iter()
                    .map(|term| term.to_lowercase())
                    .any(|term| task.desc.to_lowercase().contains(&term))
                    .then(|| format_task(list, id).action(Action::Select))
                    .or_else(|| {
                        task.implicit_tags
                            .iter()
                            .filter_map(|&tag_id| list.get(tag_id))
                            .map(|task| task.desc.to_lowercase())
                            .any(|desc| {
                                cmd.terms
                                    .iter()
                                    .map(|term| term.to_lowercase())
                                    .any(|term| desc.contains(&term))
                            })
                            .then(|| format_task(list, id))
                    })
            })
            .collect(),
        ..Default::default()
    })
}
