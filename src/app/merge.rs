use app::util::format_task;
use app::util::format_tasks_brief;
use app::util::lookup_tasks;
use chrono::DateTime;
use chrono::Utc;
use cli::Merge;
use model::DurationInSeconds;
use model::NewOptions;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::TodoPrinter;
use std::borrow::Cow;

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Merge,
) {
    let tasks_to_merge = lookup_tasks(list, &cmd.keys);
    let deps = &tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.deps(id))
        - &tasks_to_merge;
    let adeps = &tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| so_far | list.adeps(id))
        - &tasks_to_merge;
    let transitive_deps = &tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.transitive_deps(id)
        })
        - &tasks_to_merge;
    let transitive_adeps = &tasks_to_merge
        .iter_unsorted()
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.transitive_adeps(id)
        })
        - &tasks_to_merge;
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
        printer.print_error(&PrintableError::CannotMerge {
            cycle_through: format_tasks_brief(list, &cycle_through),
            adeps_of: format_tasks_brief(list, &adeps_of),
            deps_of: format_tasks_brief(list, &deps_of),
        });
        return;
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
    let merged = list.add(NewOptions {
        desc: Cow::Owned(cmd.into.to_string()),
        now,
        priority,
        due_date,
        budget,
        start_date,
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
    (deps | TaskSet::of(merged) | adeps)
        .iter_sorted(list)
        .for_each(|id| {
            printer.print_task(&format_task(list, id).action(
                if id == merged {
                    Action::Select
                } else {
                    Action::None
                },
            ));
        });
}
