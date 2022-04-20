use model::{TaskSet, TodoList};
use printing::{Action, TodoPrinter};
use {
    super::util::{
        format_task, lookup_tasks, parse_budget_or_print_error,
        should_include_done,
    },
    crate::cli::Budget,
};

pub fn run(list: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Budget) {
    let budget = match parse_budget_or_print_error(&cmd.budget, printer) {
        Ok(budget) => budget,
        Err(_) => {
            return;
        }
    };
    let tasks = lookup_tasks(list, &cmd.keys);
    let include_done =
        should_include_done(cmd.include_done, list, tasks.iter_unsorted());
    tasks
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.set_budget(id, budget)
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
}
