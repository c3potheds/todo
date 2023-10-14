use {
    super::{
        block, bottom, budget, chain, check, due, edit, find, get, log, merge,
        new, path, priority, punt, put, restore, rm, snooze, snoozed, split,
        status, tag, top, unblock, unsnooze,
    },
    todo_cli::{Options, SubCommand::*},
    todo_clock::Clock,
    todo_model::TodoList,
    todo_printing::PrintableResult,
    todo_text_editing::TextEditor,
};

fn status_options(options: Options) -> status::Status {
    status::Status {
        include_blocked: options.include_blocked || options.include_all,
        include_done: options.include_done || options.include_all,
    }
}

/// Runs the 'todo' command line application. Returns whether the list was
/// modified; if so, the caller should save the list.
pub fn todo<'list>(
    list: &'list mut TodoList,
    text_editor: &impl TextEditor,
    clock: &impl Clock,
    options: Options,
) -> PrintableResult<'list> {
    let now = clock.now();
    match options.cmd {
        Some(Block(cmd)) => block::run(list, &cmd),
        Some(Bottom(cmd)) => bottom::run(list, &cmd),
        Some(Budget(cmd)) => budget::run(list, &cmd),
        Some(Chain(cmd)) => chain::run(list, &cmd),
        Some(Check(cmd)) => check::run(list, now, &cmd),
        Some(Config(_)) => unimplemented!(),
        Some(Due(cmd)) => due::run(list, now, &cmd),
        Some(Edit(cmd)) => edit::run(list, text_editor, &cmd),
        Some(Find(cmd)) => find::run(list, &cmd),
        Some(Get(cmd)) => get::run(list, &cmd),
        Some(Log) => log::run(list),
        Some(Merge(cmd)) => merge::run(list, now, &cmd),
        Some(New(cmd)) => new::run(list, now, &cmd),
        Some(Path(cmd)) => path::run(list, &cmd),
        Some(Priority(cmd)) => priority::run(list, &cmd),
        Some(Punt(cmd)) => punt::run(list, &cmd),
        Some(Put(cmd)) => put::run(list, &cmd),
        Some(Restore(cmd)) => restore::run(list, &cmd),
        Some(Rm(cmd)) => rm::run(list, cmd),
        Some(Snooze(cmd)) => snooze::run(list, now, &cmd),
        Some(Snoozed(cmd)) => snoozed::run(list, now, &cmd),
        Some(Split(cmd)) => split::run(list, cmd),
        Some(Tag(cmd)) => tag::run(list, &cmd),
        Some(Top(cmd)) => top::run(list, &cmd),
        Some(Unblock(cmd)) => unblock::run(list, &cmd),
        Some(Unsnooze(cmd)) => unsnooze::run(list, &cmd),
        None => status::run(list, now, &status_options(options)),
    }
}
