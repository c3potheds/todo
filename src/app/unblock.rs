use app::util::any_tasks_are_complete;
use app::util::format_task;
use app::util::lookup_tasks;
use cli::Unblock;
use itertools::Itertools;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::PrintableWarning;
use printing::TodoPrinter;

fn print_unblock_warning(
    printer: &mut impl TodoPrinter,
    model: &TodoList,
    blocking: TaskId,
    blocked: TaskId,
) {
    model.position(blocked).zip(model.position(blocking)).map(
        |(cannot_unblock, requested_unblock_from)| {
            printer.print_warning(
                &PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                    cannot_unblock: cannot_unblock,
                    requested_unblock_from: requested_unblock_from,
                },
            )
        },
    );
}

fn unblock_from_given(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    tasks_to_unblock: &Vec<TaskId>,
    tasks_to_unblock_from: &Vec<TaskId>,
) -> TaskSet {
    tasks_to_unblock
        .iter()
        .copied()
        .cartesian_product(tasks_to_unblock_from.iter().copied())
        .flat_map(|(blocked, blocking)| {
            match model.unblock(blocked).from(blocking) {
                Ok(affected) => affected.into_iter_unsorted(),
                Err(_) => {
                    print_unblock_warning(printer, model, blocking, blocked);
                    TaskSet::new().into_iter_unsorted()
                }
            }
        })
        .collect()
}

fn unblock_from_all(
    model: &mut TodoList,
    tasks_to_unblock: &Vec<TaskId>,
) -> TaskSet {
    tasks_to_unblock
        .iter()
        .copied()
        .map(|id| {
            model.deps(id).iter_unsorted().for_each(|dep| {
                model.unblock(id).from(dep).unwrap();
            });
            id
        })
        .collect()
}

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Unblock,
) {
    let tasks_to_unblock = lookup_tasks(&model, &cmd.keys);
    let tasks_to_unblock_from = lookup_tasks(&model, &cmd.from);
    let include_done = cmd.include_done
        || any_tasks_are_complete(
            model,
            tasks_to_unblock
                .iter()
                .chain(tasks_to_unblock_from.iter())
                .copied(),
        );
    if !cmd.from.is_empty() && tasks_to_unblock_from.is_empty() {
        printer.print_error(&PrintableError::NoMatchForKeys {
            keys: cmd.from.clone(),
        });
        return;
    }
    let tasks_to_print = if tasks_to_unblock_from.is_empty() {
        unblock_from_all(model, &tasks_to_unblock)
    } else {
        unblock_from_given(
            model,
            printer,
            &tasks_to_unblock,
            &tasks_to_unblock_from,
        )
    };
    tasks_to_print
        .include_done(model, include_done)
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id).action(
                if tasks_to_unblock.contains(&id) {
                    Action::Unlock
                } else {
                    Action::None
                },
            ));
        });
}
