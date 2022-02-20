use app::util::format_task;
use app::util::lookup_task;
use cli::Top;
use model::TaskSet;
use model::TaskStatus;
use model::TodoList;
use printing::PrintableWarning;
use printing::TodoPrinter;

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter, cmd: &Top) {
    // Handle the case where no tasks are specified. In this case, we want to
    // print all tasks that do not have any antidependencies (including complete
    // tasks iff '--include_done' is passed).)
    if cmd.keys.is_empty() {
        model
            .all_tasks()
            .filter(|&id| {
                cmd.include_done
                    || model.status(id) != Some(TaskStatus::Complete)
            })
            .filter(|&id| model.adeps(id).is_empty())
            .for_each(|id| {
                printer.print_task(&format_task(model, id));
            });
        return;
    }

    // Handle the case where tasks are specified. If no matches are found, print
    // warnings. The top tasks for the specified tasks are deps that directly
    // block the specified tasks, and do not have any antidependencies that
    // themselves directly or indirectly block the specified tasks. In other
    // words, the top tasks for a given task are the ones that, if completed,
    // would unblock the given task.
    cmd.keys
        .iter()
        .fold(TaskSet::default(), |so_far, key| {
            let tasks = lookup_task(model, key);
            if tasks.is_empty() {
                printer.print_warning(&PrintableWarning::NoMatchFoundForKey {
                    requested_key: key.clone(),
                });
            }
            so_far | tasks
        })
        .iter_unsorted()
        // For each matching task, find the top tasks that directly block it.
        .fold(TaskSet::default(), |so_far, id| {
            model
                .deps(id)
                .include_done(model, cmd.include_done)
                .iter_unsorted()
                .filter(|&dep| {
                    !model
                        .adeps(dep)
                        .iter_unsorted()
                        .any(|adep| model.transitive_adeps(adep).contains(id))
                })
                .collect::<TaskSet>()
                | so_far
        })
        .iter_sorted(model)
        .for_each(|id| {
            printer.print_task(&format_task(model, id));
        });
}
