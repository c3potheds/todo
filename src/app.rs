use cli::Check;
use cli::Key;
use cli::New;
use cli::Options;
use cli::Restore;
use cli::SubCommand;
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

fn new(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &New,
) {
    let new_ids = cmd
        .desc
        .iter()
        .map(|desc| model.add(Task::new(desc)))
        .collect::<Vec<_>>();
    new_ids.iter().for_each(|&id| {
        printer.print_task(&format_task(printing_context, model, id))
    });
}

fn check(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Check,
) {
    let ids_to_check = cmd
        .keys
        .iter()
        .flat_map(|&Key::ByNumber(n)| model.lookup_by_number(n))
        .collect::<Vec<_>>();
    let checked_ids = ids_to_check
        .iter()
        .map(|&id| {
            model.check(id);
            id
        })
        .collect::<Vec<_>>();
    checked_ids.iter().for_each(|&id| {
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
    // This is inefficient, but there's no apparent way to coerce a daggy Walker
    // into a DoubleEndedIterator.
    let complete_tasks = model.complete_tasks().collect::<Vec<_>>();
    complete_tasks.iter().rev().for_each(|&id| {
        printer.print_task(&format_task(printing_context, model, id))
    });
}

fn restore(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    cmd: &Restore,
) {
    let ids_to_restore = cmd
        .keys
        .iter()
        .flat_map(|&Key::ByNumber(n)| model.lookup_by_number(n))
        .collect::<Vec<_>>();
    let restored_ids = ids_to_restore
        .iter()
        .copied()
        .filter(|&id| model.restore(id))
        .collect::<Vec<_>>();
    restored_ids.iter().for_each(|&id| {
        printer.print_task(&format_task(printing_context, model, id))
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
        None => status(model, printing_context, printer),
    }
}
