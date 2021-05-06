use app::util::format_prefix;
use app::util::format_task;
use app::util::lookup_tasks;
use cli::Prefix;
use model::TodoList;
use printing::TodoPrinter;

pub fn run(list: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Prefix) {
    let prefix = cmd.prefix.join(" ");
    lookup_tasks(list, &cmd.keys)
        .iter_sorted(list)
        .for_each(|id| {
            list.set_desc(
                id,
                &format_prefix(&prefix, &list.get(id).unwrap().desc),
            );
            printer.print_task(&format_task(list, id));
        });
}
