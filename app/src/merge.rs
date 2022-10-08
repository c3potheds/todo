use {
    super::util::{format_task, format_tasks_brief, lookup_tasks},
    chrono::{DateTime, Utc},
    cli::Merge,
    model::{DurationInSeconds, NewOptions, TaskSet, TodoList},
    printing::{Action, PrintableAppSuccess, PrintableError, PrintableResult},
    std::borrow::Cow,
};

pub fn run<'list>(
    list: &'list mut TodoList,
    now: DateTime<Utc>,
    cmd: &Merge,
) -> PrintableResult<'list> {
    let tasks_to_merge = lookup_tasks(list, &cmd.keys);
    let deps = tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.deps(id))
        - tasks_to_merge.clone();
    let adeps = tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.adeps(id))
        - tasks_to_merge.clone();
    let transitive_deps = tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.transitive_deps(id)
        })
        - tasks_to_merge.clone();
    let transitive_adeps = tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.transitive_adeps(id)
        })
        - tasks_to_merge.clone();
    let cycle_through = transitive_deps & transitive_adeps;
    if !cycle_through.is_empty() {
        let adeps_of = cycle_through
            .iter_unsorted()
            .fold(TaskSet::default(), |so_far, id| so_far | list.deps(id))
            & tasks_to_merge.clone();
        let deps_of = cycle_through
            .iter_unsorted()
            .fold(TaskSet::default(), |so_far, id| so_far | list.adeps(id))
            & tasks_to_merge;
        return Err(vec![PrintableError::CannotMerge {
            cycle_through: format_tasks_brief(list, &cycle_through),
            adeps_of: format_tasks_brief(list, &adeps_of),
            deps_of: format_tasks_brief(list, &deps_of),
        }]);
    }
    let priority = tasks_to_merge
        .iter_unsorted()
        .map(|id| list.get(id).unwrap().priority)
        .max()
        .unwrap_or(0);
    let due_date = tasks_to_merge
        .iter_unsorted()
        .flat_map(|id| list.get(id).unwrap().due_date)
        .min();
    let budget = tasks_to_merge
        .iter_unsorted()
        .map(|id| list.get(id).unwrap().budget.0)
        .max()
        .map(DurationInSeconds)
        .unwrap_or_default();
    let start_date = tasks_to_merge
        .iter_unsorted()
        .map(|id| list.get(id).unwrap().start_date)
        .max()
        .unwrap_or(now);
    let tag = match cmd.tag {
        Some(value) => value,
        None => tasks_to_merge
            .iter_unsorted()
            .all(|id| list.get(id).map_or_else(|| true, |data| data.tag)),
    };
    let merged = list.add(NewOptions {
        desc: Cow::Owned(cmd.into.to_string()),
        now,
        priority,
        due_date,
        budget,
        start_date,
        tag,
    });
    deps.iter_sorted(list).for_each(|dep| {
        // This shouldn't panic if we correctly detected cycles above.
        list.block(merged).on(dep).unwrap();
    });
    adeps.iter_sorted(list).for_each(|adep| {
        // This shouldn't panic if we correctly detected cycles above.
        list.block(adep).on(merged).unwrap();
    });
    tasks_to_merge.iter_sorted(list).for_each(|id| {
        list.remove(id);
    });
    let tasks_to_print = (deps | TaskSet::of(merged) | adeps)
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id).action(if id == merged {
                Action::Select
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
