use app::block;
use app::check;
use app::edit;
use app::get;
use app::log;
use app::new;
use app::punt;
use app::put;
use app::restore;
use app::status;
use app::unblock;
use cli::Options;
use cli::SubCommand;
use model::TodoList;
use printing::PrintingContext;
use printing::TodoPrinter;

pub fn todo(
    model: &mut TodoList,
    printing_context: &PrintingContext,
    printer: &mut impl TodoPrinter,
    options: &Options,
) {
    match &options.cmd {
        Some(SubCommand::New(cmd)) => {
            new::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Check(cmd)) => {
            check::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Log) => log::run(model, printing_context, printer),
        Some(SubCommand::Restore(cmd)) => {
            restore::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Block(cmd)) => {
            block::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Unblock(cmd)) => {
            unblock::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Get(cmd)) => {
            get::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Punt(cmd)) => {
            punt::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Edit(cmd)) => {
            edit::run(model, printing_context, printer, &cmd)
        }
        Some(SubCommand::Put(cmd)) => {
            put::run(model, printing_context, printer, &cmd)
        }
        None => status::run(
            model,
            printing_context,
            printer,
            &status::Status {
                include_blocked: options.include_blocked || options.include_all,
                include_done: options.include_done || options.include_all,
            },
        ),
    }
}
