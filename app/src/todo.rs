use {
    super::{
        block, bottom, budget, chain, check, due, edit, find, get, log, merge,
        new, path, priority, punt, put, restore, rm, snooze, snoozed, split,
        status, tag, top, unblock, unsnooze,
    },
    cli::{Options, SubCommand},
    clock::Clock,
    model::TodoList,
    printing::TodoPrinter,
    text_editing::TextEditor,
};

fn status_options(options: Options) -> status::Status {
    status::Status {
        include_blocked: options.include_blocked || options.include_all,
        include_done: options.include_done || options.include_all,
    }
}

/// Runs the 'todo' command line application. Returns whether the list was
/// modified; if so, the caller should save the list.
pub fn todo(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    text_editor: &impl TextEditor,
    clock: &impl Clock,
    options: Options,
) -> bool {
    use self::SubCommand::*;
    use printing::Printable;
    let now = clock.now();
    match options.cmd {
        Some(Block(cmd)) => block::run(list, &cmd).print(printer),
        Some(Bottom(cmd)) => bottom::run(list, &cmd).print(printer),
        Some(Budget(cmd)) => budget::run(list, &cmd).print(printer),
        Some(Chain(cmd)) => chain::run(list, &cmd).print(printer),
        Some(Check(cmd)) => check::run(list, now, &cmd).print(printer),
        Some(Config(_)) => unimplemented!(),
        Some(Due(cmd)) => due::run(list, printer, now, &cmd),
        Some(Edit(cmd)) => edit::run(list, text_editor, &cmd).print(printer),
        Some(Find(cmd)) => find::run(list, &cmd).print(printer),
        Some(Get(cmd)) => get::run(list, &cmd).print(printer),
        Some(Log) => log::run(list).print(printer),
        Some(Merge(cmd)) => merge::run(list, now, &cmd).print(printer),
        Some(New(cmd)) => new::run(list, printer, now, &cmd),
        Some(Path(cmd)) => path::run(list, &cmd).print(printer),
        Some(Priority(cmd)) => priority::run(list, &cmd).print(printer),
        Some(Punt(cmd)) => punt::run(list, &cmd).print(printer),
        Some(Put(cmd)) => put::run(list, &cmd).print(printer),
        Some(Restore(cmd)) => restore::run(list, &cmd).print(printer),
        Some(Rm(cmd)) => rm::run(list, printer, cmd),
        Some(Snooze(cmd)) => snooze::run(list, now, &cmd).print(printer),
        Some(Snoozed(_)) => snoozed::run(list, now).print(printer),
        Some(Split(cmd)) => split::run(list, cmd).print(printer),
        Some(Tag(cmd)) => tag::run(list, &cmd).print(printer),
        Some(Top(cmd)) => top::run(list, &cmd).print(printer),
        Some(Unblock(cmd)) => unblock::run(list, &cmd).print(printer),
        Some(Unsnooze(cmd)) => unsnooze::run(list, &cmd).print(printer),
        None => status::run(list, now, &status_options(options)).print(printer),
    }
}
