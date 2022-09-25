use {
    super::util::{
        format_task, format_task_brief, lookup_task, should_include_done,
    },
    cli::Chain,
    model::{BlockError, TaskSet, TodoList},
    printing::{Action, PrintableError, TodoPrinter},
    std::collections::HashMap,
};

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    cmd: &Chain,
) -> bool {
    let tasks = cmd
        .keys
        .iter()
        .flat_map(|key| lookup_task(list, key).iter_sorted(list))
        .collect::<Vec<_>>();
    let include_done =
        should_include_done(cmd.include_done, list, tasks.iter().copied());
    let mut actions = HashMap::new();
    let mut mutated = false;
    use itertools::Itertools;
    tasks
        .iter()
        .copied()
        .tuple_windows()
        .fold(TaskSet::default(), |so_far, (a, b)| {
            match list.block(b).on(a) {
                Ok(affected) => {
                    mutated = true;
                    actions.insert(b, Action::Lock);
                    so_far | affected
                }
                Err(BlockError::WouldCycle(_)) => {
                    printer.print_error(
                        &PrintableError::CannotBlockBecauseWouldCauseCycle {
                            cannot_block: format_task_brief(list, b),
                            requested_dependency: format_task_brief(list, a),
                        },
                    );
                    so_far
                }
                Err(BlockError::WouldBlockOnSelf) => so_far,
            }
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .for_each(|id| {
            printer.print_task(
                &format_task(list, id)
                    .action(*actions.get(&id).unwrap_or(&Action::None)),
            );
        });
    mutated
}
