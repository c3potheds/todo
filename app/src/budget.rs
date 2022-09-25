use {
    super::util::{
        format_task, lookup_tasks, parse_budget_or_print_error,
        should_include_done,
    },
    cli::Budget,
    model::{TaskSet, TodoList},
    printing::{Action, TodoPrinter},
};

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Budget,
) -> bool {
    let budget = match parse_budget_or_print_error(&cmd.budget, printer) {
        Ok(budget) => budget,
        Err(_) => {
            return false;
        }
    };
    let tasks = lookup_tasks(list, &cmd.keys);
    let include_done =
        should_include_done(cmd.include_done, list, tasks.iter_unsorted());
    let mut mutated = false;
    tasks
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
        .for_each(|id| {
            printer.print_task(&format_task(list, id).action(
                if tasks.contains(id) {
                    Action::Select
                } else {
                    Action::None
                },
            ))
        });
    mutated
}
