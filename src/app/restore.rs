use app::util::format_task;
use app::util::format_task_brief;
use app::util::format_tasks_brief;
use app::util::lookup_tasks;
use cli::Restore;
use model::ForceRestored;
use model::RestoreError;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::PrintableWarning;
use printing::TodoPrinter;
use std::collections::HashSet;

enum Reason {
    BlockingComplete(TaskSet),
    AlreadyIncomplete,
}

struct RestoreResult {
    restored: TaskSet,
    blocked: TaskSet,
    cannot_restore: Vec<(TaskId, Reason)>,
}

fn restore_with_fn<Restore: FnMut(TaskId) -> RestoreResult>(
    tasks_to_restore: Vec<TaskId>,
    mut restore_fn: Restore,
) -> RestoreResult {
    tasks_to_restore.into_iter().rev().fold(
        RestoreResult {
            restored: TaskSet::new(),
            blocked: TaskSet::new(),
            cannot_restore: Vec::new(),
        },
        |so_far, id| {
            let step = (restore_fn)(id);
            RestoreResult {
                restored: so_far.restored | step.restored,
                blocked: so_far.blocked | step.blocked,
                cannot_restore: so_far
                    .cannot_restore
                    .into_iter()
                    .chain(step.cannot_restore.into_iter())
                    .collect(),
            }
        },
    )
}

fn force_restore(
    model: &mut TodoList,
    tasks_to_restore: Vec<TaskId>,
) -> RestoreResult {
    restore_with_fn(tasks_to_restore, |id| match model.force_restore(id) {
        Ok(ForceRestored { restored, blocked }) => RestoreResult {
            restored,
            blocked,
            cannot_restore: Vec::new(),
        },
        Err(RestoreError::TaskIsAlreadyIncomplete) => RestoreResult {
            restored: TaskSet::new(),
            blocked: TaskSet::new(),
            cannot_restore: vec![(id, Reason::AlreadyIncomplete)],
        },
        Err(RestoreError::WouldRestore(_)) => {
            panic!("Somehow got a WouldRestore error from force_restore().")
        }
    })
}

fn restore(
    model: &mut TodoList,
    tasks_to_restore: Vec<TaskId>,
) -> RestoreResult {
    restore_with_fn(tasks_to_restore, |id| match model.restore(id) {
        Ok(blocked) => RestoreResult {
            restored: std::iter::once(id).collect(),
            blocked,
            cannot_restore: Vec::new(),
        },
        Err(RestoreError::TaskIsAlreadyIncomplete) => RestoreResult {
            restored: TaskSet::new(),
            blocked: TaskSet::new(),
            cannot_restore: vec![(id, Reason::AlreadyIncomplete)],
        },
        Err(RestoreError::WouldRestore(adeps)) => RestoreResult {
            restored: TaskSet::new(),
            blocked: TaskSet::new(),
            cannot_restore: vec![(id, Reason::BlockingComplete(adeps))],
        },
    })
}

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Restore,
) {
    let tasks_to_restore =
        lookup_tasks(model, &cmd.keys).iter_sorted(model).collect();
    let result = if cmd.force {
        force_restore(model, tasks_to_restore)
    } else {
        restore(model, tasks_to_restore)
    };
    result
        .cannot_restore
        .into_iter()
        .for_each(|(id, reason)| match reason {
            Reason::AlreadyIncomplete => printer.print_warning(
                &PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
                    cannot_restore: format_task_brief(model, id),
                },
            ),
            Reason::BlockingComplete(adeps) => printer.print_error(
                &PrintableError::CannotRestoreBecauseAntidependencyIsComplete {
                    cannot_restore: format_task_brief(model, id),
                    complete_antidependencies: format_tasks_brief(
                        model, &adeps,
                    ),
                },
            ),
        });
    // A task that was restored may become blocked by another task's restoration
    // and thus may show up in more than one of the TaskSets.
    let mut do_not_print_again = HashSet::new();
    result.restored.iter_sorted(model).for_each(|id| {
        do_not_print_again.insert(id);
        printer.print_task(&format_task(model, id).action(Action::Uncheck))
    });
    result.blocked.iter_sorted(model).for_each(|id| {
        if !do_not_print_again.contains(&id) {
            printer.print_task(&format_task(model, id).action(Action::Lock));
        }
    });
}
