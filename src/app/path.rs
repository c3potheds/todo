use app::util::format_task;
use app::util::format_task_brief;
use app::util::format_tasks_brief;
use app::util::lookup_task;
use app::util::pairwise;
use cli::Path;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableWarning;
use printing::TodoPrinter;

struct NoPathFound(TaskId, TaskId);

pub fn run(list: &TodoList, printer: &mut impl TodoPrinter, cmd: &Path) {
    let tasks = cmd
        .keys
        .iter()
        .flat_map(|key| {
            let matches = lookup_task(list, key);
            if matches.is_empty() {
                printer.print_warning(&PrintableWarning::NoMatchFoundForKey {
                    requested_key: key.clone(),
                });
            } else if matches.len() > 1 {
                printer.print_warning(&PrintableWarning::AmbiguousKey {
                    key: key.clone(),
                    matches: format_tasks_brief(list, &matches),
                });
            }
            matches
                .iter_sorted(list)
                // Hack to handle the one-key case. Since each item appears
                // twice, a path will be found between a task and itself if no
                // other tasks were matched.
                .flat_map(|id| std::iter::once(id).chain(std::iter::once(id)))
        })
        .collect::<Vec<_>>();
    match pairwise(tasks.iter().copied()).try_fold(
        TaskSet::default(),
        |so_far, (a, b)| {
            let a_and_adeps = TaskSet::of(a) | list.transitive_adeps(a);
            let b_and_deps = TaskSet::of(b) | list.transitive_deps(b);
            let path = a_and_adeps & b_and_deps;
            if path.is_empty() {
                return Err(NoPathFound(a, b));
            }
            Ok(so_far | path)
        },
    ) {
        Ok(path) => path,
        Err(NoPathFound(a, b)) => {
            printer.print_warning(&PrintableWarning::NoPathFoundBetween(
                format_task_brief(list, a),
                format_task_brief(list, b),
            ));
            return;
        }
    }
    .iter_sorted(list)
    .for_each(|id| {
        printer.print_task(&format_task(list, id).action(
            if tasks.contains(&id) {
                Action::Select
            } else {
                Action::None
            },
        ))
    });
}
