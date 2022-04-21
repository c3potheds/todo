use {
    super::util::{format_prefix, format_task, lookup_tasks},
    cli::Split,
    model::{DurationInSeconds, NewOptions, TaskId, TaskSet, TodoList},
    printing::{Action, TodoPrinter},
    std::borrow::Cow,
};

struct SplitResult {
    shards: TaskSet,
    to_print: TaskSet,
}

impl SplitResult {
    fn combine(self, other: SplitResult) -> SplitResult {
        SplitResult {
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
    });
    if chain {
        use itertools::Itertools;
        shards.iter().tuple_windows().for_each(|(&a, &b)| {
            list.block(b).on(a).unwrap();
        });
    }
    list.remove(id);
    SplitResult {
        shards: shards.iter().copied().collect(),
        to_print: deps
            .iter()
            .chain(shards.iter())
            .chain(adeps.iter())
            .copied()
            .collect(),
    }
}

pub fn run(list: &mut TodoList, printer: &mut impl TodoPrinter, cmd: Split) {
    let prefix = cmd.prefix.join(" ");
    let result = lookup_tasks(list, &cmd.keys).iter_sorted(list).fold(
        SplitResult {
            shards: TaskSet::default(),
            to_print: TaskSet::default(),
        },
        |so_far, id| {
            so_far.combine(split(
                list,
                id,
                cmd.into
                    .iter()
                    .map(|desc| format_prefix(&prefix, desc))
                    .collect(),
                cmd.chain,
            ))
        },
    );
    result.to_print.iter_sorted(list).for_each(|id| {
        printer.print_task(&format_task(list, id).action(
            if result.shards.contains(id) {
                Action::Select
            } else {
                Action::None
            },
        ));
    });
}
