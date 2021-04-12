use app::block;
use app::chain;
use app::check;
use app::edit;
use app::find;
use app::get;
use app::log;
use app::new;
use app::path;
use app::priority;
use app::punt;
use app::put;
use app::restore;
use app::rm;
use app::split;
use app::status;
use app::top;
use app::unblock;
use cli::Options;
use cli::SubCommand;
use clock::Clock;
use model::TodoList;
use printing::TodoPrinter;
use text_editing::TextEditor;

fn status_options(options: Options) -> status::Status {
    status::Status {
        include_blocked: options.include_blocked || options.include_all,
        include_done: options.include_done || options.include_all,
    }
}

pub fn todo(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    text_editor: &impl TextEditor,
    clock: &impl Clock,
    options: Options,
) {
    use self::SubCommand::*;
    match options.cmd {
        Some(Block(cmd)) => block::run(list, printer, &cmd),
        Some(Chain(cmd)) => chain::run(list, printer, &cmd),
        Some(Check(cmd)) => check::run(list, printer, clock, &cmd),
        Some(Due(_)) => unimplemented!(),
        Some(Edit(cmd)) => edit::run(list, printer, text_editor, &cmd),
        Some(Find(cmd)) => find::run(list, printer, &cmd),
        Some(Get(cmd)) => get::run(list, printer, &cmd),
        Some(Log) => log::run(list, printer),
        Some(Path(cmd)) => path::run(list, printer, &cmd),
        Some(New(cmd)) => new::run(list, printer, clock, cmd),
        Some(Priority(cmd)) => priority::run(list, printer, &cmd),
        Some(Punt(cmd)) => punt::run(list, printer, &cmd),
        Some(Put(cmd)) => put::run(list, printer, &cmd),
        Some(Restore(cmd)) => restore::run(list, printer, &cmd),
        Some(Rm(cmd)) => rm::run(list, printer, cmd),
        Some(Split(cmd)) => split::run(list, printer, clock, cmd),
        Some(Top(cmd)) => top::run(list, printer, &cmd),
        Some(Unblock(cmd)) => unblock::run(list, printer, &cmd),
        None => status::run(list, printer, &status_options(options)),
    }
}
