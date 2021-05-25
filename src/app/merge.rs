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

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: &Merge,
) {
    let tasks_to_merge = lookup_tasks(list, &cmd.keys);
    let deps = &tasks_to_merge
        .iter_unsorted()
        .flat_map(|id| list.deps(id).into_iter_unsorted())
        .collect::<TaskSet>()
        - &tasks_to_merge;
    let adeps = &tasks_to_merge
        .iter_unsorted()
        .flat_map(|id| list.adeps(id).into_iter_unsorted())
        .collect::<TaskSet>()
        - &tasks_to_merge;
    let transitive_deps = &tasks_to_merge
        .iter_unsorted()
        .flat_map(|id| list.transitive_deps(id).into_iter_unsorted())
        .collect::<TaskSet>()
        - &tasks_to_merge;
    let transitive_adeps = &tasks_to_merge
        .iter_unsorted()
        .flat_map(|id| list.transitive_adeps(id).into_iter_unsorted())
        .collect::<TaskSet>()
        - &tasks_to_merge;
    let cycle_through = transitive_deps & transitive_adeps;
    if !cycle_through.is_empty() {
        let adeps_of = cycle_through
            .iter_unsorted()
            .flat_map(|id| list.deps(id).into_iter_unsorted())
            .collect::<TaskSet>()
            & tasks_to_merge.clone();
        let deps_of = cycle_through
            .iter_unsorted()
            .flat_map(|id| list.adeps(id).into_iter_unsorted())
            .collect::<TaskSet>()
            & tasks_to_merge.clone();
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
        .map(|secs| DurationInSeconds(secs))
        .unwrap_or_default();
    let merged = list.add(NewOptions {
        desc: cmd.into.clone(),
        now: now,
        priority: priority,
        due_date: due_date,
        budget: budget,
        start_date: None,
    });
    deps.iter_sorted(list).for_each(|dep| {
        // This shouldn't happen if we correctly detected cycles above.
        list.block(merged).on(dep).unwrap();
    });
    adeps.iter_sorted(list).for_each(|adep| {
        // This shouldn't happen if we correctly detected cycles above.
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
