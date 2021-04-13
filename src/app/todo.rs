use app::block;
use app::chain;
use app::check;
use app::due;
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
    let now = clock.now();
    match options.cmd {
        Some(Block(cmd)) => block::run(list, printer, now, &cmd),
        Some(Chain(cmd)) => chain::run(list, printer, now, &cmd),
        Some(Check(cmd)) => check::run(list, printer, now, &cmd),
        Some(Due(cmd)) => due::run(list, printer, now, &cmd),
        Some(Edit(cmd)) => edit::run(list, printer, text_editor, now, &cmd),
        Some(Find(cmd)) => find::run(list, printer, now, &cmd),
        Some(Get(cmd)) => get::run(list, printer, now, &cmd),
        Some(Log) => log::run(list, printer, now),
        Some(Path(cmd)) => path::run(list, printer, now, &cmd),
        Some(New(cmd)) => new::run(list, printer, now, cmd),
        Some(Priority(cmd)) => priority::run(list, printer, now, &cmd),
        Some(Punt(cmd)) => punt::run(list, printer, now, &cmd),
        Some(Put(cmd)) => put::run(list, printer, now, &cmd),
        Some(Restore(cmd)) => restore::run(list, printer, now, &cmd),
        Some(Rm(cmd)) => rm::run(list, printer, now, cmd),
        Some(Split(cmd)) => split::run(list, printer, now, cmd),
        Some(Top(cmd)) => top::run(list, printer, now, &cmd),
        Some(Unblock(cmd)) => unblock::run(list, printer, now, &cmd),
        None => status::run(list, printer, now, &status_options(options)),
    }
}
