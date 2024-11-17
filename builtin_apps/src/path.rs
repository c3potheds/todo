use todo_cli::Path;
use todo_model::TaskId;
use todo_model::TaskSet;
use todo_model::TodoList;
use todo_printing::Action;
use todo_printing::PrintableAppSuccess;
use todo_printing::PrintableResult;
use todo_printing::PrintableWarning;

use super::util::format_task;
use super::util::format_task_brief;
use super::util::format_tasks_brief;
use super::util::lookup_task;

struct NoPathFound(TaskId, TaskId);

pub fn run<'list>(list: &'list TodoList, cmd: &Path) -> PrintableResult<'list> {
    let (tasks, mut warnings) = cmd.keys.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut tasks, mut warnings), key| {
            let found_tasks = lookup_task(list, key);
            if found_tasks.is_empty() {
                warnings.push(PrintableWarning::NoMatchFoundForKey {
                    requested_key: key.clone(),
                });
            } else if found_tasks.len() > 1 {
                warnings.push(PrintableWarning::AmbiguousKey {
                    key: key.clone(),
                    matches: format_tasks_brief(list, &found_tasks),
                });
            }
            found_tasks.iter_sorted(list).for_each(|id| {
                tasks.push(id);
                // Hack to handle the one-key case. Since each item appears
                // twice, a path will be found between a task and itself if no
                // other tasks were matched.
                tasks.push(id);
            });
            (tasks, warnings)
        },
    );
    use itertools::Itertools;
    let tasks_to_print = match tasks.iter().copied().tuple_windows().try_fold(
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
            warnings.push(PrintableWarning::NoPathFoundBetween(
                format_task_brief(list, a),
                format_task_brief(list, b),
            ));
            return Ok(PrintableAppSuccess {
                tasks: Vec::new(),
                warnings,
                ..Default::default()
            });
        }
    }
    .iter_sorted(list)
    .map(|id| {
        format_task(list, id).action(if tasks.contains(&id) {
            Action::Select
        } else {
            Action::None
        })
    })
    .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        warnings,
        ..Default::default()
    })
}
