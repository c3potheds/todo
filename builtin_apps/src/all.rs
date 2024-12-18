use todo_app::Application;
use todo_cli::Options;
use todo_cli::SubCommand::*;
use todo_clock::Clock;
use todo_model::TodoList;
use todo_printing::PrintableResult;
use todo_text_editing::TextEditor;

use super::block;
use super::bottom;
use super::budget;
use super::chain;
use super::check;
use super::clean;
use super::due;
use super::edit;
use super::find;
use super::get;
use super::log;
use super::merge;
use super::new;
use super::path;
use super::priority;
use super::punt;
use super::put;
use super::restore;
use super::rm;
use super::snooze;
use super::snoozed;
use super::split;
use super::status;
use super::tag;
use super::top;
use super::unblock;
use super::unsnooze;

fn status_options(options: Options) -> status::Status {
    status::Status {
        include_blocked: options.include_blocked || options.include_all,
        include_done: options.include_done || options.include_all,
    }
}

/// Runs the 'todo' command line application. Returns whether the list was
/// modified; if so, the caller should save the list.
fn todo<'list>(
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
        Some(Clean(_)) => clean::run(list, now),
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

pub struct App {
    options: Options,
}

impl App {
    pub fn new(options: Options) -> Self {
        App { options }
    }
}

impl Application for App {
    type Result<'a> = PrintableResult<'a>;
    fn run<'a>(
        self,
        list: &'a mut TodoList,
        text_editor: &impl TextEditor,
        clock: &impl Clock,
    ) -> Self::Result<'a> {
        todo(list, text_editor, clock, self.options)
    }
}
