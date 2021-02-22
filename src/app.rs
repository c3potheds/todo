use cli::Check;
use cli::Key;
use cli::New;
use cli::Options;
use cli::SubCommand;
use model::Task;
use model::TaskId;
use model::TodoList;
use printing::PrintableTask;
use printing::TodoPrinter;

fn format_task<'a>(model: &'a TodoList, id: TaskId) -> PrintableTask<'a> {
    PrintableTask {
        desc: &model.get(id).desc,
        number: model.get_number(id).unwrap(),
    }
}

fn new(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &New) {
    let new_ids = cmd
        .desc
        .iter()
        .map(|desc| model.add(Task::new(desc)))
        .collect::<Vec<_>>();
    new_ids
        .iter()
        .for_each(|&id| printer.print_task(&format_task(model, id)));
}

fn check(model: &mut TodoList, printer: &mut impl TodoPrinter, cmd: &Check) {
    let checked_ids = cmd
        .keys
        .iter()
        .flat_map(|&Key::ByNumber(n)| model.lookup_by_number(n))
        .copied()
        .collect::<Vec<_>>();
    let checked_ids = checked_ids
        .iter()
        .map(|&id| {
            model.check(id);
            id
        })
        .collect::<Vec<_>>();
    checked_ids
        .iter()
        .for_each(|&id| printer.print_task(&format_task(model, id)));
}

fn status(model: &TodoList, printer: &mut impl TodoPrinter) {
    model
        .incomplete_tasks()
        .for_each(|&id| printer.print_task(&format_task(model, id)))
}

fn log(model: &TodoList, printer: &mut impl TodoPrinter) {
    model
        .complete_tasks()
        .rev()
        .for_each(|&id| printer.print_task(&format_task(model, id)));
}

pub fn todo(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    options: &Options,
) {
    match &options.cmd {
        Some(SubCommand::New(cmd)) => new(model, printer, &cmd),
        Some(SubCommand::Check(cmd)) => check(model, printer, &cmd),
        Some(SubCommand::Log) => log(model, printer),
        None => status(model, printer),
    }
}
