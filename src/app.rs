use cli::Block;
use cli::Check;
use cli::Key;
use cli::New;
use cli::Options;
use cli::Restore;
use cli::SubCommand;
use itertools::Itertools;
use model::Task;
use model::TaskId;
use model::TodoList;
use printing::PrintableTask;
use printing::PrintingContext;
use printing::TodoPrinter;

fn format_task<'a>(
    context: &'a PrintingContext,
    model: &'a TodoList,
    id: TaskId,
) -> PrintableTask<'a> {
    let number = model.get_number(id).unwrap();
    PrintableTask {
        context: context,
        desc: &model.get(id).unwrap().desc,
        number: number,
        status: model.get_status(id).unwrap(),
    }
}

fn lookup_tasks(model: &TodoList, keys: &Vec<Key>) -> Vec<TaskId> {
    keys.iter()
        .flat_map(|&Key::ByNumber(n)| model.lookup_by_number(n))
        .collect::<Vec<_>>()
}

fn new(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &New,
) {
    cmd.desc
        .iter()
        .map(|desc| model.add(Task::new(desc)))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(printing_context, model, id))
        });
}

fn check(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Check,
) {
    lookup_tasks(&model, &cmd.keys)
        .into_iter()
        .filter_map(|id| if model.check(id) { Some(id) } else { None })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(printing_context, model, id))
        });
}

fn status(
    model: &TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
) {
    model.incomplete_tasks().for_each(|id| {
        printer.print_task(&format_task(printing_context, model, id))
    })
}

fn log(
    model: &TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
) {
    model.complete_tasks().for_each(|id| {
        printer.print_task(&format_task(printing_context, model, id))
    });
}

fn restore(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Restore,
) {
    lookup_tasks(&model, &cmd.keys)
        .into_iter()
        .filter(|&id| model.restore(id))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(printing_context, model, id))
        });
}

fn block(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Block,
) {
    lookup_tasks(&model, &cmd.keys)
        .into_iter()
        .cartesian_product(lookup_tasks(&model, &cmd.on).into_iter())
        .filter(|&(blocked, blocking)| model.block(blocked).on(blocking))
        .map(|(blocked, _)| blocked)
        .unique()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|id| {
            printer.print_task(&format_task(printing_context, model, id));
        });
}

pub fn todo(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    options: &Options,
) {
    match &options.cmd {
        Some(SubCommand::New(cmd)) => {
            new(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Check(cmd)) => {
            check(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Log) => log(model, printing_context, printer),
        Some(SubCommand::Restore(cmd)) => {
            restore(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Block(cmd)) => {
            block(model, printing_context, printer, &cmd)
        }
        None => status(model, printing_context, printer),
    }
}
