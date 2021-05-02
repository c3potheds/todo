use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use app::util::should_include_done;
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
    printer.print_warning(
        &PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
            cannot_unblock: format_task_brief(model, blocked),
            requested_unblock_from: format_task_brief(model, blocking),
        },
    );
}

fn unblock_from_given(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    tasks_to_unblock: &TaskSet,
    tasks_to_unblock_from: &TaskSet,
) -> TaskSet {
    tasks_to_unblock
        .iter_sorted(model)
        .cartesian_product(
            tasks_to_unblock_from.iter_sorted(model).collect::<Vec<_>>(),
        )
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
    tasks_to_unblock: &TaskSet,
) -> TaskSet {
    tasks_to_unblock
        .iter_unsorted()
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
    let include_done = should_include_done(
        cmd.include_done,
        model,
        (tasks_to_unblock.clone() | tasks_to_unblock_from.clone())
            .iter_unsorted(),
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
                if tasks_to_unblock.contains(id) {
                    Action::Unlock
                } else {
                    Action::None
                },
            ));
        });
}
