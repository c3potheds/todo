use chrono::DateTime;
use chrono::Utc;
use todo_cli::Check;
use todo_model::CheckError;
use todo_model::CheckOptions;
use todo_model::ForceChecked;
use todo_model::TaskId;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableError;
use todo_printing::PrintableResult;
use todo_printing::PrintableWarning;

use super::util::format_task;
use super::util::format_task_brief;
use super::util::lookup_tasks;

#[derive(Default)]
struct Checked {
    checked: TaskSet,
    unlocked: TaskSet,
    already_complete: TaskSet,
    mutated: bool,
}

struct CannotComplete {
    cannot_complete: TaskId,
    blocked_by: Vec<TaskId>,
}

type CheckResult = Result<Checked, Vec<CannotComplete>>;

impl std::ops::BitOr for Checked {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let checked = self.checked | other.checked;
        let unlocked = (self.unlocked | other.unlocked) - checked.clone();
        Checked {
            checked,
            unlocked,
            already_complete: self.already_complete | other.already_complete,
            mutated: self.mutated || other.mutated,
        }
    }
}

fn check_with_fn<Check: FnMut(TaskId) -> CheckResult>(
    tasks_to_check: Vec<TaskId>,
    mut check_fn: Check,
) -> CheckResult {
    tasks_to_check
        .into_iter()
        .try_fold(Checked::default(), |a, id| Ok(a | check_fn(id)?))
}

fn force_check(
    list: &mut TodoList,
    now: DateTime<Utc>,
    tasks_to_check: TaskSet,
) -> CheckResult {
    check_with_fn(tasks_to_check.iter_sorted(list).collect(), |id| match list
        .force_check(CheckOptions { id, now })
    {
        Ok(ForceChecked {
            completed,
            unblocked,
        }) => {
            let mutated = !completed.is_empty() || !unblocked.is_empty();
            Ok(Checked {
                checked: completed,
                unlocked: unblocked,
                already_complete: TaskSet::default(),
                mutated,
            })
        }
        Err(CheckError::TaskIsAlreadyComplete) => Ok(Checked {
            already_complete: TaskSet::of(id),
            ..Default::default()
        }),
        Err(CheckError::TaskIsBlockedBy(_)) => {
            panic!("Somehow got a TaskIsBlockedBy error from force_check().")
        }
    })
}

fn check(
    list: &mut TodoList,
    now: DateTime<Utc>,
    tasks_to_check: TaskSet,
) -> CheckResult {
    check_with_fn(tasks_to_check.iter_sorted(list).collect(), |id| match list
        .check(CheckOptions { id, now })
    {
        Ok(unblocked) => Ok(Checked {
            checked: TaskSet::of(id),
            unlocked: unblocked,
            already_complete: TaskSet::default(),
            mutated: true,
        }),
        Err(CheckError::TaskIsAlreadyComplete) => Ok(Checked {
            already_complete: TaskSet::of(id),
            ..Default::default()
        }),
        Err(CheckError::TaskIsBlockedBy(deps)) => Err(vec![CannotComplete {
            cannot_complete: id,
            blocked_by: deps,
        }]),
    })
}

pub fn run<'list>(
    list: &'list mut TodoList,
    now: DateTime<Utc>,
    cmd: &Check,
) -> PrintableResult<'list> {
    let tasks_to_check = lookup_tasks(list, &cmd.keys);
    let Checked {
        checked,
        unlocked,
        already_complete,
        mutated,
    } = if cmd.force {
        force_check(list, now, tasks_to_check)
    } else {
        check(list, now, tasks_to_check)
    }
    .map_err(|cannot_completes| {
        cannot_completes
            .into_iter()
            .map(
                |CannotComplete {
                     cannot_complete,
                     blocked_by,
                 }| PrintableError::CannotCheckBecauseBlocked {
                    cannot_check: format_task_brief(list, cannot_complete),
                    blocked_by: blocked_by
                        .into_iter()
                        .map(|dep| format_task_brief(list, dep))
                        .collect(),
                },
            )
            .collect::<Vec<_>>()
    })?;
    let warnings = already_complete
        .iter_sorted(list)
        .map(|id| PrintableWarning::CannotCheckBecauseAlreadyComplete {
            cannot_check: format_task_brief(list, id),
        })
        .collect();
    let tasks_to_print = checked
        .iter_sorted(list)
        .map(|id| format_task(list, id).action(Action::Check))
        .chain(
            unlocked
                .iter_sorted(list)
                .map(|id| format_task(list, id).action(Action::Unlock)),
        )
        .collect();
    Ok(PrintableAppSuccess {
        warnings,
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}
