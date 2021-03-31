use app::util::format_task;
use app::util::lookup_tasks;
use chrono::DateTime;
use chrono::Utc;
use cli::Check;
use clock::Clock;
use model::CheckError;
use model::CheckOptions;
use model::ForceChecked;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::PrintableWarning;
use printing::TodoPrinter;

enum Reason {
    BlockedBy(Vec<TaskId>),
    AlreadyComplete,
}

struct CheckResult {
    completed: TaskSet,
    unblocked: TaskSet,
    cannot_complete: Vec<(TaskId, Reason)>,
}

fn check_with_fn<Check: FnMut(TaskId) -> CheckResult>(
    tasks_to_check: Vec<TaskId>,
    mut check_fn: Check,
) -> CheckResult {
    tasks_to_check.into_iter().fold(
        CheckResult {
            completed: TaskSet::new(),
            unblocked: TaskSet::new(),
            cannot_complete: Vec::new(),
        },
        |so_far, id| {
            let step = (check_fn)(id);
            CheckResult {
                completed: so_far.completed | step.completed,
                unblocked: so_far.unblocked | step.unblocked,
                cannot_complete: so_far
                    .cannot_complete
                    .into_iter()
                    .chain(step.cannot_complete.into_iter())
                    .collect(),
            }
        },
    )
}

fn force_check(
    model: &mut TodoList,
    now: DateTime<Utc>,
    tasks_to_check: Vec<TaskId>,
) -> CheckResult {
    check_with_fn(tasks_to_check, |id| {
        match model.force_check(CheckOptions { id: id, now: now }) {
            Ok(ForceChecked {
                completed,
                unblocked,
            }) => CheckResult {
                completed: completed,
                unblocked: unblocked,
                cannot_complete: Vec::new(),
            },
            Err(CheckError::TaskIsAlreadyComplete) => CheckResult {
                completed: TaskSet::new(),
                unblocked: TaskSet::new(),
                cannot_complete: vec![(id, Reason::AlreadyComplete)],
            },
            Err(CheckError::TaskIsBlockedBy(_)) => panic!(
                "Somehow got a TaskIsBlockedBy error from force_check()."
            ),
        }
    })
}

fn check(
    model: &mut TodoList,
    now: DateTime<Utc>,
    tasks_to_check: Vec<TaskId>,
) -> CheckResult {
    check_with_fn(tasks_to_check, |id| {
        match model.check(CheckOptions { id: id, now: now }) {
            Ok(unblocked) => CheckResult {
                completed: std::iter::once(id).collect(),
                unblocked: unblocked,
                cannot_complete: Vec::new(),
            },
            Err(CheckError::TaskIsAlreadyComplete) => CheckResult {
                completed: TaskSet::new(),
                unblocked: TaskSet::new(),
                cannot_complete: vec![(id, Reason::AlreadyComplete)],
            },
            Err(CheckError::TaskIsBlockedBy(deps)) => CheckResult {
                completed: TaskSet::new(),
                unblocked: TaskSet::new(),
                cannot_complete: vec![(id, Reason::BlockedBy(deps))],
            },
        }
    })
}

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    clock: &impl Clock,
    cmd: &Check,
) {
    let tasks_to_check = lookup_tasks(model, &cmd.keys);
    let now = clock.now();
    let result = if cmd.force {
        force_check(model, now, tasks_to_check)
    } else {
        check(model, now, tasks_to_check)
    };
    result
        .cannot_complete
        .into_iter()
        .for_each(|(id, reason)| match reason {
            Reason::AlreadyComplete => printer.print_warning(
                &PrintableWarning::CannotCheckBecauseAlreadyComplete {
                    cannot_check: model.position(id).unwrap(),
                },
            ),
            Reason::BlockedBy(deps) => printer.print_error(
                &PrintableError::CannotCheckBecauseBlocked {
                    cannot_check: model.position(id).unwrap(),
                    blocked_by: deps
                        .into_iter()
                        .flat_map(|dep| model.position(dep).into_iter())
                        .collect(),
                },
            ),
        });
    result.completed.iter_sorted(model).for_each(|id| {
        printer.print_task(&format_task(model, id, Action::Check))
    });
    result.unblocked.iter_sorted(model).for_each(|id| {
        printer.print_task(&format_task(model, id, Action::Unlock))
    });
}
