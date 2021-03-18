use app::block;
use app::chain;
use app::check;
use app::edit;
use app::find;
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
use printing::TodoPrinter;
use text_editing::TextEditor;

pub fn todo(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    text_editor: &impl TextEditor,
    options: &Options,
) {
    match &options.cmd {
        Some(SubCommand::Block(cmd)) => block::run(model, printer, &cmd),
        Some(SubCommand::Chain(cmd)) => chain::run(model, printer, &cmd),
        Some(SubCommand::Check(cmd)) => check::run(model, printer, &cmd),
        Some(SubCommand::Edit(cmd)) => {
            edit::run(model, printer, text_editor, &cmd)
        }
        Some(SubCommand::Find(cmd)) => find::run(model, printer, &cmd),
        Some(SubCommand::Get(cmd)) => get::run(model, printer, &cmd),
        Some(SubCommand::Log) => log::run(model, printer),
        Some(SubCommand::New(cmd)) => new::run(model, printer, &cmd),
        Some(SubCommand::Punt(cmd)) => punt::run(model, printer, &cmd),
        Some(SubCommand::Put(cmd)) => put::run(model, printer, &cmd),
        Some(SubCommand::Restore(cmd)) => restore::run(model, printer, &cmd),
        Some(SubCommand::Unblock(cmd)) => unblock::run(model, printer, &cmd),
        None => status::run(
            model,
            printer,
            &status::Status {
                include_blocked: options.include_blocked || options.include_all,
                include_done: options.include_done || options.include_all,
            },
        ),
    }
}
