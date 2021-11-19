use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use chrono::DateTime;
use chrono::Utc;
use cli::Check;
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
use std::collections::HashMap;

enum Reason {
    BlockedBy(Vec<TaskId>),
    AlreadyComplete,
}

struct CheckResult {
    to_print: HashMap<TaskId, Action>,
    cannot_complete: Vec<(TaskId, Reason)>,
}

fn check_with_fn<Check: FnMut(TaskId) -> CheckResult>(
    tasks_to_check: Vec<TaskId>,
    mut check_fn: Check,
) -> CheckResult {
    tasks_to_check.into_iter().fold(
        CheckResult {
            to_print: HashMap::new(),
            cannot_complete: Vec::new(),
        },
        |mut so_far, id| {
            let step = (check_fn)(id);
            so_far.to_print.extend(step.to_print.into_iter());
            so_far
                .cannot_complete
                .extend(step.cannot_complete.into_iter());
            so_far
        },
    )
}

fn force_check(
    model: &mut TodoList,
    now: DateTime<Utc>,
    tasks_to_check: TaskSet,
) -> CheckResult {
    check_with_fn(
        tasks_to_check.iter_sorted(model).collect(),
        |id| match model.force_check(CheckOptions { id, now }) {
            Ok(ForceChecked {
                completed,
                unblocked,
            }) => CheckResult {
                to_print: unblocked
                    .into_iter_unsorted()
                    .map(|id| (id, Action::Unlock))
                    .chain(
                        completed
                            .into_iter_unsorted()
                            .map(|id| (id, Action::Check)),
                    )
                    .collect(),
                cannot_complete: Vec::new(),
            },
            Err(CheckError::TaskIsAlreadyComplete) => CheckResult {
                to_print: HashMap::new(),
                cannot_complete: vec![(id, Reason::AlreadyComplete)],
            },
            Err(CheckError::TaskIsBlockedBy(_)) => panic!(
                "Somehow got a TaskIsBlockedBy error from force_check()."
            ),
        },
    )
}

fn check(
    model: &mut TodoList,
    now: DateTime<Utc>,
    tasks_to_check: TaskSet,
) -> CheckResult {
    check_with_fn(
        tasks_to_check.iter_sorted(model).collect(),
        |id| match model.check(CheckOptions { id, now }) {
            Ok(unblocked) => CheckResult {
                to_print: {
                    let mut to_print = HashMap::new();
                    to_print.insert(id, Action::Check);
                    to_print.extend(
                        unblocked
                            .into_iter_unsorted()
                            .map(|id| (id, Action::Unlock)),
                    );
                    to_print
                },
                cannot_complete: Vec::new(),
            },
            Err(CheckError::TaskIsAlreadyComplete) => CheckResult {
                to_print: HashMap::new(),
                cannot_complete: vec![(id, Reason::AlreadyComplete)],
            },
            Err(CheckError::TaskIsBlockedBy(deps)) => CheckResult {
                to_print: HashMap::new(),
                cannot_complete: vec![(id, Reason::BlockedBy(deps))],
            },
        },
    )
}

fn print_cannot_complete_error(
    model: &TodoList,
    printer: &mut impl TodoPrinter,
    id: TaskId,
    reason: Reason,
) {
    match reason {
        Reason::AlreadyComplete => printer.print_warning(
            &PrintableWarning::CannotCheckBecauseAlreadyComplete {
                cannot_check: format_task_brief(model, id),
            },
        ),
        Reason::BlockedBy(deps) => {
            printer.print_error(&PrintableError::CannotCheckBecauseBlocked {
                cannot_check: format_task_brief(model, id),
                blocked_by: deps
                    .into_iter()
                    .map(|dep| format_task_brief(model, dep))
                    .collect(),
            })
        }
    }
}

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Check,
) {
    let tasks_to_check = lookup_tasks(model, &cmd.keys);
    let CheckResult {
        to_print,
        cannot_complete,
    } = if cmd.force {
        force_check(model, now, tasks_to_check)
    } else {
        check(model, now, tasks_to_check)
    };
    cannot_complete.into_iter().for_each(|(id, reason)| {
        print_cannot_complete_error(model, printer, id, reason)
    });
    to_print
        .keys()
        .copied()
        .collect::<TaskSet>()
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id).action(to_print[&id]));
        });
}
