use app::util::format_task;
use app::util::lookup_tasks;
use cli::Priority;
use model::TaskSet;
use model::TodoList;
use printing::TodoPrinter;

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Priority,
) {
    let tasks = lookup_tasks(list, &cmd.keys);
    tasks
        .into_iter()
        .flat_map(|id| list.set_priority(id, cmd.priority).into_iter_unsorted())
        .collect::<TaskSet>()
        .iter_sorted(list)
        .for_each(|id| printer.print_task(&format_task(list, id)));
}
