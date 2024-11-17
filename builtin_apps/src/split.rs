use std::borrow::Cow;

use chrono::Duration;
use todo_cli::Split;
use todo_model::DurationInSeconds;
use todo_model::NewOptions;
use todo_model::TaskId;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;

use super::util::format_task;
use super::util::lookup_tasks;

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
    into: &[String],
    chain: bool,
    keep: bool,
    tag: Option<bool>,
) -> SplitResult {
    let original_is_tag = match list.get(id) {
        Some(task) => task.tag,
        None => false,
    };
    let deps: Vec<_> = list.deps(id).iter_sorted(list).collect();
    let adeps: Vec<_> = list.adeps(id).iter_sorted(list).collect();
    let num_shards: u32 = into.len().try_into().unwrap();
    let shards = into
        .iter()
        .map(|desc| {
            let task = list.get(id).unwrap();
            let options = NewOptions {
                desc: Cow::Owned(desc.trim().to_string()),
                now: task.creation_time,
                priority: task.priority,
                due_date: task.due_date,
                budget: if chain {
                    DurationInSeconds(task.budget.0 / num_shards)
                } else {
                    task.budget
                },
                start_date: Some(task.start_date),
                tag: match tag {
                    Some(value) => value,
                    None => !keep && original_is_tag,
                },
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

pub fn run<'list>(
    list: &'list mut TodoList,
    cmd: Split,
) -> PrintableResult<'list> {
    let SplitResult {
        kept,
        shards,
        to_print,
    } = lookup_tasks(list, &cmd.keys)
        .iter_sorted(list)
        .map(|id| split(list, id, &cmd.into, cmd.chain, cmd.keep, cmd.tag))
        .fold(SplitResult::default(), SplitResult::combine);
    let mutated = !to_print.is_empty();
    let tasks_to_print = to_print
        .iter_sorted(list)
        .map(|id| {
            format_task(list, id).action(if shards.contains(id) {
                Action::New
            } else if kept.contains(id) {
                Action::Select
            } else {
                Action::None
            })
        })
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}
