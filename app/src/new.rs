use {
    super::util::{
        format_task, format_task_brief, format_tasks_brief, lookup_tasks,
        parse_budget, parse_due_date, parse_snooze_date,
    },
    chrono::{DateTime, Utc},
    std::{borrow::Cow, collections::HashSet, iter::FromIterator},
    todo_cli::New,
    todo_model::{CheckError, CheckOptions, NewOptions, TaskSet, TodoList},
    todo_printing::{
        Action, PrintableAppSuccess, PrintableError, PrintableResult,
    },
};

pub fn run<'list>(
    list: &'list mut TodoList,
    now: DateTime<Utc>,
    cmd: &New,
) -> PrintableResult<'list> {
    let due_date = parse_due_date(now, &cmd.due).map_err(|e| vec![e])?;
    let budget = parse_budget(&cmd.budget).map_err(|e| vec![e])?;
    let snooze_date = parse_snooze_date(now, &cmd.snooze)
        .map_err(|e| vec![e])?;
    let deps = lookup_tasks(list, &cmd.blocked_by);
    let adeps = lookup_tasks(list, &cmd.blocking);
    let before = lookup_tasks(list, &cmd.before);
    let by = lookup_tasks(list, &cmd.by);
    let before_deps = before
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.deps(id));
    let after = lookup_tasks(list, &cmd.after);
    let after_adeps = after
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.adeps(id));
    let (by_deps, by_adeps) = by.iter_unsorted().fold(
        (TaskSet::default(), TaskSet::default()),
        |(so_far_deps, so_far_adeps), id| {
            (so_far_deps | list.deps(id), so_far_adeps | list.adeps(id))
        },
    );
    let deps = deps | before_deps | after | by_deps;
    let adeps = adeps | before | after_adeps | by_adeps;
    let priority = cmd.priority;
    let mut to_print = HashSet::new();
    let new_tasks: TaskSet = cmd
        .desc
        .iter()
        .map(|desc| {
            let id = list.add(NewOptions {
                desc: Cow::Owned(desc.trim().to_string()),
                now,
                priority: priority.unwrap_or(0),
                due_date,
                budget,
                start_date: snooze_date,
                tag: cmd.tag,
            });
            to_print.insert(id);
            id
        })
        .collect();
    to_print.extend(
        deps.product(&new_tasks, list)
            .try_fold(
                TaskSet::default(),
                |so_far, (a, b)| -> Result<_, Vec<PrintableError>> {
                    Ok(so_far
                        | list.block(b).on(a).map_err(|_| {
                            vec![
                            PrintableError::CannotBlockBecauseWouldCauseCycle {
                                cannot_block: format_task_brief(list, b),
                                requested_dependency: format_task_brief(
                                    list, a,
                                ),
                            },
                        ]
                        })?)
                },
            )?
            .iter_unsorted(),
    );
    if cmd.done {
        to_print.extend(
            new_tasks
                .iter_sorted(list)
                .try_fold(TaskSet::default(), |so_far, id| {
                    match list.check(CheckOptions { id, now }) {
                        Ok(affected) => Ok(so_far | affected),
                        Err(CheckError::TaskIsBlockedBy(blocking)) => {
                            Err(vec![
                                PrintableError::CannotCheckBecauseBlocked {
                                    cannot_check: format_task_brief(list, id),
                                    blocked_by: format_tasks_brief(
                                        list,
                                        &TaskSet::from_iter(blocking),
                                    ),
                                },
                            ])
                        }
                        _ => Ok(so_far),
                    }
                })?
                .iter_unsorted(),
        );
    }
    to_print.extend(
        new_tasks
            .product(&adeps, list)
            .try_fold(
                TaskSet::default(),
                |so_far, (a, b)| -> Result<_, Vec<PrintableError>> {
                    Ok(so_far
                        | list.block(b).on(a).map_err(|_| {
                            vec![
                            PrintableError::CannotBlockBecauseWouldCauseCycle {
                                cannot_block: format_task_brief(list, b),
                                requested_dependency: format_task_brief(
                                    list, a,
                                ),
                            },
                        ]
                        })?)
                },
            )?
            .iter_unsorted(),
    );
    if cmd.chain {
        use itertools::Itertools;
        new_tasks.iter_sorted(list).tuple_windows().for_each(
            |(a, b)| match list.block(b).on(a) {
                Ok(affected) => to_print.extend(affected.iter_unsorted()),
                Err(_) => {
                    panic!("This should never happen because all tasks are new")
                }
            },
        );
    }
    let tasks_to_print = TaskSet::from_iter(to_print)
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id).action(if new_tasks.contains(id) {
                Action::New
            } else {
                Action::None
            })
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated: true,
        ..Default::default()
    })
}
