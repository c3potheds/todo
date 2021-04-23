use app::util::format_task;
use app::util::lookup_tasks;
use app::util::pairwise;
use chrono::DateTime;
use chrono::Utc;
use cli::Split;
use model::NewOptions;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

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
    now: DateTime<Utc>,
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
                desc: desc.clone(),
                // Inherit the creation time from the source task,
                // but if there was no creation time for some
                // reason, take the current moment as the creation
                // time.
                now: task.creation_time.unwrap_or(now),
                priority: task.priority,
                due_date: task.due_date,
                budget: task.budget,
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

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    now: DateTime<Utc>,
    cmd: Split,
) {
    let to_split = lookup_tasks(list, &cmd.keys);
    let result = to_split.iter().copied().fold(
        SplitResult {
            shards: TaskSet::new(),
            to_print: TaskSet::new(),
        },
        |so_far, id| {
            so_far.combine(split(
                list,
                now,
                id,
                cmd.into.iter().map(|desc| desc.clone()),
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
