use app::util::format_task;
use app::util::lookup_tasks;
use app::util::parse_budget_or_print_error;
use app::util::should_include_done;
use cli::Budget;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

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
        .fold(TaskSet::new(), |so_far, id| {
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