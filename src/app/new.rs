use app::util::format_task;
use app::util::lookup_tasks;
use app::util::pairwise;
use chrono::DateTime;
use chrono::Local;
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
    let due_date_string = cmd.due.join(" ");
    let due_date = if !due_date_string.is_empty() {
        match ::time_format::parse_time(
            Local,
            now.with_timezone(&Local),
            &due_date_string,
        ) {
            Ok(due_date) => Some(due_date.with_timezone(&Utc)),
            Err(_) => {
                printer.print_error(&PrintableError::CannotParseDueDate {
                    cannot_parse: due_date_string.clone(),
                });
                return;
            }
        }
    } else {
        None
    };
    let deps = lookup_tasks(model, &cmd.blocked_by);
    let adeps = lookup_tasks(model, &cmd.blocking);
    let before = lookup_tasks(model, &cmd.before);
    let before_deps = before
        .iter()
        .copied()
        .flat_map(|id| model.deps(id).into_iter_unsorted())
        .collect::<TaskSet>();
    let after = lookup_tasks(model, &cmd.after);
    let after_adeps = after
        .iter()
        .copied()
        .flat_map(|id| model.adeps(id).into_iter_unsorted())
        .collect::<TaskSet>();
    let deps = deps
        .into_iter()
        .chain(before_deps.into_iter_unsorted())
        .chain(after.into_iter())
        .collect::<TaskSet>()
        .iter_sorted(model)
        .collect::<Vec<_>>();
    let adeps = adeps
        .into_iter()
        .chain(before.into_iter())
        .chain(after_adeps.into_iter_unsorted())
        .collect::<TaskSet>()
        .iter_sorted(model)
        .collect::<Vec<_>>();
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
            });
            to_print.insert(id);
            id
        })
        .collect();
    deps.into_iter()
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(dep, new)| {
            match model.block(new).on(dep) {
                Ok(affected) => to_print.extend(affected.iter_unsorted()),
                // TODO(app.new.print-warning-on-cycle): print a warning, but
                // continue in the error case.
                Err(_) => panic!("Cannot block"),
            }
        });
    adeps
        .into_iter()
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(adep, new)| {
            match model.block(adep).on(new) {
                Ok(affected) => to_print.extend(affected.iter_unsorted()),
                // TODO(app.new.print-warning-on-cycle): print a warning, but
                // continue in the error case.
                Err(_) => panic!("cannot block"),
            }
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
