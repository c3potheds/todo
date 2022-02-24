extern crate humantime;

use app::util::format_prefix;
use app::util::format_task;
use app::util::format_task_brief;
use app::util::format_tasks_brief;
use app::util::lookup_tasks;
use app::util::pairwise;
use app::util::parse_budget_or_print_error;
use app::util::parse_due_date_or_print_error;
use app::util::parse_snooze_date_or_print_error;
use chrono::DateTime;
use chrono::Utc;
use cli::New;
use model::CheckError;
use model::CheckOptions;
use model::NewOptions;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::borrow::Cow;

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &New,
) {
    let due_date = match parse_due_date_or_print_error(now, &cmd.due, printer) {
        Ok(due_date) => due_date,
        Err(_) => return,
    };
    let budget = match parse_budget_or_print_error(&cmd.budget, printer) {
        Ok(budget) => budget,
        Err(_) => return,
    };
    let snooze_date =
        match parse_snooze_date_or_print_error(now, &cmd.snooze, printer) {
            Ok(Some(snooze_date)) => snooze_date,
            Ok(None) => now,
            Err(_) => return,
        };
    let deps = lookup_tasks(model, &cmd.blocked_by);
    let adeps = lookup_tasks(model, &cmd.blocking);
    let before = lookup_tasks(model, &cmd.before);
    let before_deps = before
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | model.deps(id));
    let after = lookup_tasks(model, &cmd.after);
    let after_adeps = after
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | model.adeps(id));
    let deps = deps | before_deps | after;
    let adeps = adeps | before | after_adeps;
    let priority = cmd.priority;
    let mut to_print = HashSet::new();
    let prefix = cmd.prefix.join(" ");
    let new_tasks: TaskSet = cmd
        .desc
        .iter()
        .map(|desc| {
            let id = model.add(NewOptions {
                desc: Cow::Owned(format_prefix(&prefix, desc)),
                now,
                priority: priority.unwrap_or(0),
                due_date,
                budget,
                start_date: snooze_date,
            });
            to_print.insert(id);
            id
        })
        .collect();
    deps.product(&new_tasks, model).for_each(|(dep, new)| {
        match model.block(new).on(dep) {
            Ok(affected) => to_print.extend(affected.iter_unsorted()),
            Err(_) => printer.print_error(
                &PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block: format_task_brief(model, new),
                    requested_dependency: format_task_brief(model, dep),
                },
            ),
        }
    });
    adeps.product(&new_tasks, model).for_each(|(adep, new)| {
        match model.block(adep).on(new) {
            Ok(affected) => to_print.extend(affected.iter_unsorted()),
            Err(_) => printer.print_error(
                &PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block: format_task_brief(model, adep),
                    requested_dependency: format_task_brief(model, new),
                },
            ),
        }
    });
    if cmd.chain {
        pairwise(new_tasks.iter_sorted(model)).for_each(|(a, b)| {
            match model.block(b).on(a) {
                Ok(affected) => to_print.extend(affected.iter_unsorted()),
                Err(_) => {
                    panic!("This should never happen because all tasks are new")
                }
            }
        });
    }
    if cmd.done {
        new_tasks.iter_sorted(model).for_each(|id| {
            if let Err(CheckError::TaskIsBlockedBy(blocking)) =
                model.check(CheckOptions { id, now })
            {
                printer.print_error(
                    &PrintableError::CannotCheckBecauseBlocked {
                        cannot_check: format_task_brief(model, id),
                        blocked_by: format_tasks_brief(
                            model,
                            &TaskSet::from_iter(blocking),
                        ),
                    },
                );
            }
        });
    }
    TaskSet::from_iter(to_print.into_iter())
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id).action(
                if new_tasks.contains(id) {
                    Action::New
                } else {
                    Action::None
                },
            ));
        });
}
