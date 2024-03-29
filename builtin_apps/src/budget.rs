use {
    super::util::{
        format_task, lookup_tasks, parse_budget, should_include_done,
    },
    todo_cli::Budget,
    todo_model::{TaskSet, TodoList},
    todo_printing::{Action, PrintableAppSuccess, PrintableResult},
};

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Budget,
) -> PrintableResult<'list> {
    let budget = parse_budget(&cmd.budget).map_err(|e| vec![e])?;
    let tasks = lookup_tasks(list, &cmd.keys);
    let include_done =
        should_include_done(cmd.include_done, list, tasks.iter_unsorted());
    let mut mutated = false;
    let tasks = tasks
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            let affected_by_id = list.set_budget(id, budget);
            if affected_by_id.is_empty() {
                return so_far;
            }
            mutated = true;
            so_far | affected_by_id
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id).action(if tasks.contains(id) {
                Action::Select
            } else {
                Action::None
            })
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks,
        mutated,
        ..Default::default()
    })
}
