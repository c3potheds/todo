use chrono::Duration;

use {
    super::util::{format_prefix, format_task, lookup_tasks},
    cli::Split,
    model::{DurationInSeconds, NewOptions, TaskId, TaskSet, TodoList},
    printing::{Action, TodoPrinter},
    std::borrow::Cow,
};

#[derive(Default)]
struct SplitResult {
    kept: TaskSet,
    shards: TaskSet,
    to_print: TaskSet,
}

impl SplitResult {
    fn combine(self, other: SplitResult) -> SplitResult {
        SplitResult {
            kept: self.kept | other.kept,
            shards: self.shards | other.shards,
            to_print: self.to_print | other.to_print,
        }
    }
}

fn split(
    list: &mut TodoList,
    id: TaskId,
    into: Vec<String>,
    chain: bool,
    keep: bool,
) -> SplitResult {
    let deps: Vec<_> = list.deps(id).iter_sorted(list).collect();
    let adeps: Vec<_> = list.adeps(id).iter_sorted(list).collect();
    let num_shards: u32 = into.len().try_into().unwrap();
    let shards = into
        .iter()
        .map(|desc| {
            let task = list.get(id).unwrap();
            let options = NewOptions {
                desc: Cow::Owned(desc.clone()),
                now: task.creation_time,
                priority: task.priority,
                due_date: task.due_date,
                budget: if chain {
                    DurationInSeconds(task.budget.0 / num_shards)
                } else {
                    task.budget
                },
                start_date: task.start_date,
                tag: false,
            };
            list.add(options)
        })
        .collect::<Vec<_>>();
    shards.iter().copied().for_each(|shard| {
        deps.iter().copied().for_each(|dep| {
            list.block(shard).on(dep).unwrap();
        });
        adeps.iter().copied().for_each(|adep| {
            list.block(adep).on(shard).unwrap();
        });
        list.block(id).on(shard).unwrap();
    });
    if chain {
        use itertools::Itertools;
        shards.iter().tuple_windows().for_each(|(&a, &b)| {
            list.block(b).on(a).unwrap();
        });
    }
    let kept = if !keep {
        list.remove(id);
        TaskSet::default()
    } else {
        list.set_budget(id, Duration::zero());
        TaskSet::of(id)
    };
    SplitResult {
        kept: kept.clone(),
        shards: shards.iter().copied().collect(),
        to_print: kept
            | deps
                .iter()
                .chain(shards.iter())
                .chain(adeps.iter())
                .copied()
                .collect::<TaskSet>(),
    }
}

pub fn run(list: &mut TodoList, printer: &mut impl TodoPrinter, cmd: Split) {
    let prefix = cmd.prefix.join(" ");
    let result = lookup_tasks(list, &cmd.keys).iter_sorted(list).fold(
        SplitResult::default(),
        |so_far, id| {
            so_far.combine(split(
                list,
                id,
                cmd.into
                    .iter()
                    .map(|desc| format_prefix(&prefix, desc))
                    .collect(),
                cmd.chain,
                cmd.keep,
            ))
        },
    );
    result.to_print.iter_sorted(list).for_each(|id| {
        printer.print_task(&format_task(list, id).action(
            if result.shards.contains(id) {
                Action::New
            } else if result.kept.contains(id) {
                Action::Select
            } else {
                Action::None
            },
        ));
    });
}
