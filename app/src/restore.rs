use {
    super::util::{
        format_task, format_task_brief, format_tasks_brief, lookup_tasks,
    },
    todo_cli::Restore,
    todo_model::{ForceRestored, RestoreError, TaskId, TaskSet, TodoList},
    todo_printing::{
        Action, PrintableAppSuccess, PrintableError, PrintableResult,
        PrintableWarning,
    },
};

#[derive(Default)]
struct Restored {
    restored: TaskSet,
    blocked: TaskSet,
    already_incomplete: TaskSet,
    mutated: bool,
}

impl std::ops::BitOr for Restored {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let restored = self.restored | other.restored;
        let blocked = (self.blocked | other.blocked) - restored.clone();
        Restored {
            restored,
            blocked,
            already_incomplete: self.already_incomplete
                | other.already_incomplete,
            mutated: self.mutated || other.mutated,
        }
    }
}

struct CannotRestore {
    cannot_restore: TaskId,
    blocking_complete: Vec<TaskId>,
}

type RestoreResult = Result<Restored, Vec<CannotRestore>>;

fn restore_with_fn<Restore: FnMut(TaskId) -> RestoreResult>(
    tasks_to_restore: Vec<TaskId>,
    mut restore_fn: Restore,
) -> RestoreResult {
    tasks_to_restore
        .into_iter()
        .rev()
        .try_fold(Restored::default(), |acc, id| Ok(acc | restore_fn(id)?))
}

fn force_restore(
    list: &mut TodoList,
    tasks_to_restore: Vec<TaskId>,
) -> RestoreResult {
    restore_with_fn(tasks_to_restore, |id| match list.force_restore(id) {
        Ok(ForceRestored { restored, blocked }) => Ok(Restored {
            restored,
            blocked,
            mutated: true,
            ..Default::default()
        }),
        Err(RestoreError::TaskIsAlreadyIncomplete) => Ok(Restored {
            already_incomplete: TaskSet::of(id),
            ..Default::default()
        }),
        Err(RestoreError::WouldRestore(_)) => {
            panic!("Somehow got a WouldRestore error from force_restore().")
        }
    })
}

fn restore(
    list: &mut TodoList,
    tasks_to_restore: Vec<TaskId>,
) -> RestoreResult {
    restore_with_fn(tasks_to_restore, |id| match list.restore(id) {
        Ok(blocked) => Ok(Restored {
            restored: TaskSet::of(id),
            blocked,
            mutated: true,
            ..Default::default()
        }),
        Err(RestoreError::TaskIsAlreadyIncomplete) => Ok(Restored {
            already_incomplete: TaskSet::of(id),
            ..Default::default()
        }),
        Err(RestoreError::WouldRestore(adeps)) => Err(vec![CannotRestore {
            cannot_restore: id,
            blocking_complete: adeps.iter_sorted(list).collect(),
        }]),
    })
}

fn format_cannot_restore(
    list: &TodoList,
    cannot_restore: CannotRestore,
) -> PrintableError {
    let CannotRestore {
        cannot_restore,
        blocking_complete,
    } = cannot_restore;
    PrintableError::CannotRestoreBecauseAntidependencyIsComplete {
        cannot_restore: format_task_brief(list, cannot_restore),
        complete_antidependencies: format_tasks_brief(
            list,
            &blocking_complete.into_iter().collect(),
        ),
    }
}

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: &Restore,
) -> PrintableResult<'list> {
    let tasks_to_restore =
        lookup_tasks(list, &cmd.keys).iter_sorted(list).collect();
    let Restored {
        restored,
        blocked,
        already_incomplete,
        mutated,
    } = if cmd.force {
        force_restore(list, tasks_to_restore)
    } else {
        restore(list, tasks_to_restore)
    }
    .map_err(|cannot_restore| {
        cannot_restore
            .into_iter()
            .map(|e| format_cannot_restore(list, e))
            .collect::<Vec<_>>()
    })?;
    use PrintableWarning::CannotRestoreBecauseAlreadyIncomplete;
    let warnings = already_incomplete
        .iter_sorted(list)
        .map(|id| CannotRestoreBecauseAlreadyIncomplete {
            cannot_restore: format_task_brief(list, id),
        })
        .collect();
    let tasks_to_print = restored
        .iter_sorted(list)
        .map(|id| format_task(list, id).action(Action::Uncheck))
        .chain(
            blocked
                .iter_sorted(list)
                .map(|id| format_task(list, id).action(Action::Lock)),
        )
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        warnings,
        mutated,
        ..Default::default()
    })
}
