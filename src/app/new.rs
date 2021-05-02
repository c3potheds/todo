extern crate humantime;

use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use app::util::pairwise;
use app::util::parse_budget_or_print_error;
use app::util::parse_due_date_or_print_error;
use chrono::DateTime;
use chrono::Utc;
use cli::New;
use itertools::Itertools;
use model::NewOptions;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: New,
) {
    let due_date = match parse_due_date_or_print_error(now, &cmd.due, printer) {
        Ok(due_date) => due_date,
        Err(_) => {
            return;
        }
    };
    let budget = match parse_budget_or_print_error(&cmd.budget, printer) {
        Ok(budget) => budget,
        Err(_) => {
            return;
        }
    };
    let deps = lookup_tasks(model, &cmd.blocked_by);
    let adeps = lookup_tasks(model, &cmd.blocking);
    let before = lookup_tasks(model, &cmd.before);
    let before_deps = before
        .iter_unsorted()
        .flat_map(|id| model.deps(id).into_iter_unsorted())
        .collect::<TaskSet>();
    let after = lookup_tasks(model, &cmd.after);
    let after_adeps = after
        .iter_unsorted()
        .flat_map(|id| model.adeps(id).into_iter_unsorted())
        .collect::<TaskSet>();
    let deps = deps | before_deps | after;
    let adeps = adeps | before | after_adeps;
    let priority = cmd.priority;
    let mut to_print = HashSet::new();
    let new_tasks: Vec<_> = cmd
        .desc
        .into_iter()
        .map(|desc| {
            let id = model.add(NewOptions {
                desc: desc,
                now: now,
                priority: priority.unwrap_or(0),
                due_date: due_date,
                budget: budget,
            });
            to_print.insert(id);
            id
        })
        .collect();
    deps.iter_sorted(model)
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(dep, new)| match model.block(new).on(dep) {
            Ok(affected) => to_print.extend(affected.iter_unsorted()),
            Err(_) => printer.print_error(
                &PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block: format_task_brief(model, new),
                    requested_dependency: format_task_brief(model, dep),
                },
            ),
        });
    adeps
        .iter_sorted(model)
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(adep, new)| match model.block(adep).on(new) {
            Ok(affected) => to_print.extend(affected.iter_unsorted()),
            Err(_) => printer.print_error(
                &PrintableError::CannotBlockBecauseWouldCauseCycle {
                    cannot_block: format_task_brief(model, adep),
                    requested_dependency: format_task_brief(model, new),
                },
            ),
        });
    if cmd.chain {
        pairwise(new_tasks.iter().copied()).for_each(|(a, b)| {
            match model.block(b).on(a) {
                Ok(affected) => to_print.extend(affected.iter_unsorted()),
                Err(_) => {
                    panic!("This should never happen because all tasks are new")
                }
            }
        });
    }
    TaskSet::from_iter(to_print.into_iter())
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id).action(
                if new_tasks.contains(&id) {
                    Action::New
                } else {
                    Action::None
                },
            ));
        });
}
