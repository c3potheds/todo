use app::util::format_task;
use app::util::lookup_tasks;
use app::util::pairwise;
use cli::New;
use clock::Clock;
use itertools::Itertools;
use model::NewOptions;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::TodoPrinter;

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    clock: &impl Clock,
    cmd: New,
) {
    let deps = lookup_tasks(model, &cmd.blocked_by);
    let adeps = lookup_tasks(model, &cmd.blocking);
    let before = lookup_tasks(model, &cmd.before);
    let before_deps = before
        .iter()
        .copied()
        .flat_map(|id| model.deps(id).into_iter_unsorted())
        .collect::<TaskSet>();
    let after = lookup_tasks(model, &cmd.after);
    let after_adeps = after
        .iter()
        .copied()
        .flat_map(|id| model.adeps(id).into_iter_unsorted())
        .collect::<TaskSet>();
    let deps = deps
        .into_iter()
        .chain(before_deps.into_iter_unsorted())
        .chain(after.into_iter())
        .collect::<TaskSet>()
        .iter_sorted(model)
        .collect::<Vec<_>>();
    let adeps = adeps
        .into_iter()
        .chain(before.into_iter())
        .chain(after_adeps.into_iter_unsorted())
        .collect::<TaskSet>()
        .iter_sorted(model)
        .collect::<Vec<_>>();
    let now = clock.now();
    let priority = cmd.priority;
    let new_tasks: Vec<_> = cmd
        .desc
        .into_iter()
        .map(|desc| {
            model.add(NewOptions {
                desc: desc,
                now: now,
                priority: priority.unwrap_or(0),
                due_date: None,
            })
        })
        .collect();
    deps.iter()
        .copied()
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(dep, new)| {
            // TODO(app.new.print-warning-on-cycle): print a warning, but
            // continue in the error case.
            model.block(new).on(dep).expect("Cannot block");
        });
    adeps
        .iter()
        .copied()
        .cartesian_product(new_tasks.iter().copied())
        .for_each(|(adep, new)| {
            // TODO(app.new.print-warning-on-cycle): print a warning, but
            // continue in the error case.
            model.block(adep).on(new).expect("Cannot block");
        });
    if cmd.chain {
        pairwise(new_tasks.iter().copied()).for_each(|(a, b)| {
            model.block(b).on(a).expect(
                "This should never happen because all blocking tasks are new",
            );
        });
    }
    deps.into_iter()
        .for_each(|id| printer.print_task(&format_task(model, id)));
    new_tasks.into_iter().for_each(|id| {
        printer.print_task(&format_task(model, id).action(Action::New))
    });
    adeps
        .into_iter()
        .for_each(|id| printer.print_task(&format_task(model, id)));
}
