use {
    super::util::{format_task, format_task_brief, lookup_tasks},
    chrono::{DateTime, Utc},
    cli::Check,
    model::{
        CheckError, CheckOptions, ForceChecked, TaskId, TaskSet, TodoList,
    },
    printing::{Action, PrintableError, PrintableWarning, TodoPrinter},
};

enum Reason {
    BlockedBy(Vec<TaskId>),
    AlreadyComplete,
}

#[derive(Default)]
struct CheckResult {
    checked: TaskSet,
    unlocked: TaskSet,
    cannot_complete: Vec<(TaskId, Reason)>,
    mutated: bool,
}

impl std::ops::BitOr for CheckResult {
    type Output = Self;

    fn bitor(mut self, other: Self) -> Self {
        self.cannot_complete.extend(other.cannot_complete);
        CheckResult {
            checked: self.checked | other.checked,
            unlocked: self.unlocked | other.unlocked,
            cannot_complete: self.cannot_complete,
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
        .fold(CheckResult::default(), |so_far, id| so_far | check_fn(id))
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
            CheckResult {
                checked: completed,
                unlocked: unblocked,
                mutated,
                ..Default::default()
            }
        }
        Err(CheckError::TaskIsAlreadyComplete) => CheckResult {
            cannot_complete: vec![(id, Reason::AlreadyComplete)],
            ..Default::default()
        },
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
        Ok(unblocked) => CheckResult {
            checked: TaskSet::of(id),
            unlocked: unblocked,
            mutated: true,
            ..Default::default()
        },
        Err(CheckError::TaskIsAlreadyComplete) => CheckResult {
            cannot_complete: vec![(id, Reason::AlreadyComplete)],
            ..Default::default()
        },
        Err(CheckError::TaskIsBlockedBy(deps)) => CheckResult {
            cannot_complete: vec![(id, Reason::BlockedBy(deps))],
            ..Default::default()
        },
    })
}

fn print_cannot_complete_error(
    list: &TodoList,
    printer: &mut impl TodoPrinter,
    id: TaskId,
    reason: Reason,
) {
    match reason {
        Reason::AlreadyComplete => printer.print_warning(
            &PrintableWarning::CannotCheckBecauseAlreadyComplete {
                cannot_check: format_task_brief(list, id),
            },
        ),
        Reason::BlockedBy(deps) => {
            printer.print_error(&PrintableError::CannotCheckBecauseBlocked {
                cannot_check: format_task_brief(list, id),
                blocked_by: deps
                    .into_iter()
                    .map(|dep| format_task_brief(list, dep))
                    .collect(),
            })
        }
    }
}

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Check,
) -> bool {
    let tasks_to_check = lookup_tasks(list, &cmd.keys);
    let CheckResult {
        checked,
        unlocked,
        cannot_complete,
        mutated,
    } = if cmd.force {
        force_check(list, now, tasks_to_check)
    } else {
        check(list, now, tasks_to_check)
    };
    cannot_complete.into_iter().for_each(|(id, reason)| {
        print_cannot_complete_error(list, printer, id, reason)
    });
    checked.iter_sorted(list).for_each(|id| {
        printer.print_task(&format_task(list, id).action(Action::Check));
    });
    (unlocked - checked).iter_sorted(list).for_each(|id| {
        printer.print_task(&format_task(list, id).action(Action::Unlock));
    });
    mutated
}
