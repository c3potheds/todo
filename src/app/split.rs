use app::util::format_prefix;
use app::util::format_task;
use app::util::lookup_tasks;
use app::util::pairwise;
use cli::Split;
use model::NewOptions;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;
use std::borrow::Cow;

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
    into: impl Iterator<Item = String>,
    chain: bool,
) -> SplitResult {
    let deps: Vec<_> = list.deps(id).iter_sorted(list).collect();
    let adeps: Vec<_> = list.adeps(id).iter_sorted(list).collect();
    let shards = into
        .map(|desc| {
            let task = list.get(id).unwrap();
            let options = NewOptions {
                desc: Cow::Owned(desc.to_string()),
                now: task.creation_time,
                priority: task.priority,
                due_date: task.due_date,
                budget: task.budget,
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
        pairwise(shards.iter()).for_each(|(&a, &b)| {
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
                cmd.into.iter().map(|desc| format_prefix(&prefix, desc)),
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
